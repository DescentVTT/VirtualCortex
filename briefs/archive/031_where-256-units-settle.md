---
status: archived
date: 2026-09-20
---

> **Executed 2026-09-21 in pull request #80.** Writes ADR-0070 (where 256 units settle:
> on the instrument's own executor under the drive alone over eighty windows, brief 026's
> clause first holds at the ninth window, at 0.818 of the prior's excitatory sum, and the sum
> is still falling at the eightieth, at 0.551, by 0.24 per cent per window, so the clause
> reads a rate and ADR-0055's item is discharged with both numbers; the lead-in the rule
> derived is nine windows, committed before the run behind it; behind it the calibration's
> measure reads 50 of 64 in the first block and 34 to 44 after, so the criterion fails at the
> first block and 256 units is not usable for this task behind a lead-in of any length,
> because the measure follows the sum's level and the drive alone carries the sum below the
> level at which the instrument sees; a learning round's criterion stays at 1 024 units; no
> constant moves). No finding; image format 14 unchanged; the determinism pin untouched.
> Every deliverable is done; notes under the boxes say where the tree departs from the text:
> the run without the lead-in is the existing weekly test rather than a third run, and the
> gate's one test carries the rules over the pinned tables beside the first four windows.
> The report is in the pull request and in `CHANGELOG.md`. The body below describes the
> tree before execution and is not maintained; its relative links gained one `../`.

# Brief 031 — Where 256 units settle: the windows at which the excitatory sum reaches a fixed point under the drive alone, and whether a lead-in of them holds the instrument through a whole run

> A self-contained prompt for one round of work. Read `briefs/README.md`, then this file, then
> the files it names. Frozen when executed; the outcome lives in the ADRs that accept it, the
> changelog and the code.

## Mission

Two measurements taken six days apart, in different experiments, are the same fact.
[ADR-0055](../../docs/adr/0055-a-weight-that-settles.md) left an item open in whitepaper §11.1:
at 1 024 units the excitatory sum settles at 0.45 of the prior's over eighty windows and holds
there, but "at 256 units with the gain held over sixteen windows the sum is **still falling** at
the sixteenth", and "the item stays open for the windows at which 256 units settle".
[ADR-0066](../../docs/adr/0066-the-reward-path-measured-again.md) then measured the same thing from
the other side: in the learning task at 256 units the calibration's measure falls from 57 of 64
in the first block to 31 in the eighth "as the excitatory sum falls to 0.55 of the prior's, so
the instrument loses the stimulus within a block once the weights move", while at 1 024 units it
stays at 57 to 64 through the run.

The consequence is not academic. It is why ADR-0066's criterion had to drop to 1 024 units alone,
why brief 030's does too, and why every learning round after them pays for the expensive size.
This round asks the open item directly — **where do 256 units settle, if they settle** — and then
asks the one question that follows: **does a lead-in of that many windows, run before the first
trial, hold the instrument through a whole run?**

It changes **no rule**. Not the pair rule, not ADR-0055's depression, not the inhibitory rule,
not the controller, not the prior, not one constant of the instrument. A lead-in is the harness
running the engine under the drive before the task starts. The round's whole content is a
measurement, a constant derived from it by a rule written here, and a criterion written before
the run. An answer either way is worth having: if the instrument holds, every later round may
measure at the cheap size; if it does not, the tree says so once, with numbers, and later rounds
stop paying for a 256-unit run that cannot be read.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). This round adds **no
  mechanism at all** — no rule, no controller, no constant of the engine. In particular it does
  **not** build per-unit synaptic scaling (Turrigiano): §11.1 gates that on H-8's measurement and
  states its reason against this very problem — "a gain multiplies what arrives and cannot lift a
  weight from zero" — and ADR-0055 already answered the drain in the pair rule. A round that
  reached for it here would be reaching for a mechanism the tree has already argued is the wrong
  one. No new rule, no dependency, no new tool, no version bump of a tool.
- Every claim is Implemented, Specified, Target or Hypothesis. What a run holds at 256 units is
  stated with the prior's parameters and the configuration; nothing here says anything about
  1 024 units, which ADR-0055 and ADR-0066 have measured, or about Appendix A's scale
  ([ADR-0010](../../docs/adr/0010-measured-or-target.md)). No timing figure from a developer machine.
- The repository wins over the document; a disagreement is a numbered finding in whitepaper §11.
  ADR-0055's and ADR-0066's numbers stay in them and their tests keep pinning them; this round is
  compared with them, never folded into them.
- No `f32`/`f64`, in the crates, the tests and the oracles; a fraction of a sum is an integer
  ratio in Q16.16 or a count; every operation on a state field saturates or wraps by name
  ([ADR-0029](../../docs/adr/0029-structural-enforcement.md)).
- Every loop ends by construction: a countdown, a range, a scan by `get`, a slice's iterator;
  never by a comparison alone that one operator flip turns into a walk without end
  ([ADR-0062](../../docs/adr/0062-the-first-complete-sweeps-list.md)). A settling loop in particular
  never ends by "until it settles": it runs a fixed number of windows and the clause is read
  afterwards.
- Every quantity has one owner ([ADR-0016](../../docs/adr/0016-thirty-two-crate-architecture.md)).
  **No new crate**; the crate count stays 32; no record changes; the image format stays 14; no
  reserved byte is taken.
- **Nothing is chosen after a run.** The lead-in is **derived** by the rule in Deliverable A from
  the settling measurement, never picked to make a criterion pass, and it is committed before the
  run that uses it. What a run says beyond the criterion is a reading (ADR-0051, ADR-0054,
  ADR-0060, ADR-0066).
- The determinism pin of [ADR-0030](../../docs/adr/0030-verification-governance.md) does not move,
  and neither does any pinned number of ADR-0055, ADR-0065 or ADR-0066: nothing here changes a
  rule of any crate. A run with a lead-in is a **new** run with its own pins beside them, never a
  change to theirs. The mutation gate on the changed lines must pass; every number an arithmetic
  oracle can produce is computed by that oracle before the test that asserts it.
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive` and runs in the weekly job;
  the pull request's gate grows by at most one test, and that test runs no whole run
  ([ADR-0061](../../docs/adr/0061-the-learning-runs-leave-the-gate.md)).
- The engine never amends its own code ([ADR-0031](../../docs/adr/0031-policy-amendment.md)); no
  registry entry (F-37). Conventional Commits with a real body; never commit on `main`; the
  required checks keep their names.

## Context

Re-derived on 2026-09-20 against `main` at `cefac3a`. Line numbers move; the symbols and the
quoted sentences are what to re-derive.

1. **The open item, quoted.** Whitepaper §11.1, under the per-unit scaling row, as
   [ADR-0055](../../docs/adr/0055-a-weight-that-settles.md) left it on 2026-09-14: "Over eighty
   windows at 1 024 units under the controller the excitatory sum settles at 0.45 of the prior's
   and every one of the last sixteen windows is within 0.75 per cent of the sixty-fourth at both
   periods, a night inside them; at 256 units with the gain held over sixteen windows the sum is
   still falling at the sixteenth, by 1.2 per cent per window at the 5 Hz period and 1.9 at
   20 Hz, decelerating from 2.6, so brief 026's 256-unit clause (each of the last four windows
   moving the sum by less than two per cent) failed at the 20 Hz period by 0.27 points in the
   thirteenth window while the 1 024-unit clause held; the rule stands, no constant moved, and
   **the item stays open for the windows at which 256 units settle**."
2. **The same fact in the learning task.** ADR-0066: "at 256 units the calibration's measure
   falls from 57 of 64 in the first block to 31 in the eighth as the excitatory sum falls to 0.55
   of the prior's, so the instrument loses the stimulus within a block once the weights move,
   while at 1 024 units it stays at 57 to 64 through the run." Its criterion is therefore at
   1 024 units alone, and brief 030 keeps it there, with the 256-unit run as one reading under a
   "not measured" rule.
3. **The geometry, and why a lead-in might not be enough.** A window is
   $2^{\mathrm{ACTIVITY\_BIN\_SHIFT} + \mathrm{ACTIVITY\_WINDOW\_SHIFT}} = 2^{12+5} = 2^{17}$
   ticks (`crates/cortex-homeostasis/src/lib.rs`). A run of 512 trials of $2^{14}$ ticks is
   $2^{23}$ ticks, which is **sixty-four windows** — so ADR-0066's 256-unit run was already four
   times the length at which ADR-0055 last looked, and its calibration measure was still falling
   in the eighth block. The round must be able to conclude that the task's own stimulation keeps
   the sum moving, in which case no lead-in fixes it; that is a result and is stated as one.
4. **The harness.** `runtime/cortex-runtime/tests/reference.rs`: `windows(exec, drive, windows)`
   runs whole windows and returns their readings; `weights_by_polarity(exec) -> (i64, i64)` is
   the sum of the inhibitory magnitudes and the sum of the excitatory weights over the arena;
   `at_gain(p, config, gain)` holds a gain through the image. **`settle(exec)` in that file
   already means something else** — running quiet until the network is quiescent — so the lead-in
   of this round needs its own name. `runtime/cortex-runtime/tests/instrument.rs` carries the
   instrument's constants, its geometry and its calibration measure.
5. **The configuration to hold.** ADR-0065's and ADR-0066's, unchanged: the prior of ADR-0044 at
   seed 22 synthesised at 256 units, the gain the calibration picked at that size (2.0) held
   through the image, no controller, no sleep, the inhibitory period at its default, the drive of
   ADR-0044, `LEAD_IN` as the instrument already defines it before the first trial. This round's
   lead-in is **in addition to** that one and is measured in windows, not ticks.
6. **The budget** ([ADR-0061](../../docs/adr/0061-the-learning-runs-leave-the-gate.md),
   [ADR-0067](../../docs/adr/0067-the-weekly-dispatch-has-a-scope.md)). The weekly `exhaustive`
   job's time is a range and not a figure — 33 m, 56 m, 40 m and 1 h 11 m on the four runs on
   record, runner variance of about two to one dominating — against a 120-minute bound, and
   brief 030 adds six runs before this one. Eighty windows at 256 units is $80 \times 2^{17}$
   ticks, about one run of the task's size; this round adds three such runs (Deliverable A, and
   the two of Deliverable C). The round **measures the job's time and states it**, and if it
   would near the bound it says so rather than raising it. This round **adds tests**, which is
   [ADR-0067](../../docs/adr/0067-the-weekly-dispatch-has-a-scope.md)'s class, so its evidence
   dispatch is `-f scope=both`.
7. **What the answer decides.** If the instrument holds at 256 units behind a lead-in, a later
   learning round may carry its criterion at that size, where a run is about a quarter of the
   1 024-unit cost, and the budget of item 6 stops being the binding constraint on the learning
   track. If it does not, §11.1 says so with the numbers and later rounds stop spending a run on
   a size whose readings cannot be read. Either way the open item of ADR-0055 is discharged.

## Deliverables

- [x] **The settling measurement (the next free ADR number; `ls docs/adr`)**
  (`depends-on: ADR-0055`; ADR-0044, ADR-0065 and ADR-0066 named). At **256 units**, in the
  configuration of Context item 5, with **no task and no stimulus** — the drive alone — for
  **eighty windows**, the same length ADR-0055 gave 1 024 units. Per window, read and pin: the
  two sums of `weights_by_polarity`, the population's spikes, and the fraction of units at the
  target. **The clause is brief 026's, unchanged**: a window $W$ satisfies it when each of the
  last four windows up to and including $W$ moved the excitatory sum by **less than two per
  cent**. The measurement reports the smallest such $W$.
  - **The lead-in is derived, not chosen**: it is that smallest $W$, or **eighty** if no window
    up to eighty satisfies the clause. No other value may be used, and it is not adjusted after
    any run of Deliverable C.
  - The ADR states, beside the reading, whether the sum is still falling at the eightieth window
    and at what rate per window, so that §11.1's item is discharged with a number either way.
  - **Done as ADR-0070**, `the_settling_at_256_units_exhaustive` (`8f370b7` on `main`, `f99bd64`
    on the branch before the rebase): the clause first
    holds at the ninth window (1.82, 1.79, 1.58 and 1.44 per cent of the fifth's; the sum
    0.818 of the prior's); the lead-in derived is nine; at the eightieth the sum is 0.551 of
    the prior's and still falling by 0.24 per cent per window, decelerating from 3.86.
- [x] **The constants commit**, preceding the first commit that holds a run of Deliverable C: the
  lead-in in windows, and the instrument's constants restated unchanged.
  - **Done:** `5149dbe` on `main` (`b3b3cab` on the branch before the rebase; `LEAD_IN_WINDOWS`,
    nine; the criterion's rule; `run_behind`), before `20da91c` (`4a655c7`), the first commit
    that holds a run behind it; ADR-0070 cites both.
- [x] **The instrument behind the lead-in (the same ADR or the next)**. At **256 units**, two
  runs of the task as ADR-0066 composed it, the rewarded run in the task's order, 512 trials:
  - **with** the lead-in before the first trial, and
  - **without** it, which must reproduce ADR-0066's 256-unit reading (the calibration's measure
    57 of 64 in the first block falling to 31 in the eighth) so that the lead-in is the only
    difference between them.

  Per block, ADR-0066's readings: the calibration's measure, the two sums by polarity, the
  readouts' spikes by stimulus, the window before the injection, the correct trials and the ties.
  **The criterion, written before the run:** with the lead-in, the calibration's measure is
  **at least 56 of 64 in every block of the run**.
  - If it holds, 256 units is usable for this task behind the lead-in and the ADR says so with
    the per-block numbers; a later round may carry a criterion there.
  - If it does not, 256 units is **not usable** for this task, the ADR says that with the block
    at which the measure crosses, and names — **without building** — what would be needed. The
    correct trials of either run are a reading, never a clause: this round measures the
    instrument, not learning, and no clause of ADR-0066's criterion is restated or re-decided
    here.
  - **Done in ADR-0070**, `the_recalibrated_rewarded_run_at_256_units_behind_the_lead_in_exhaustive`
    (`20da91c` on `main`, `4a655c7` before the rebase): the measure reads 50, 44, 43, 35, 41,
    34, 36, 36 of 64, so the criterion
    fails at the first block and 256 units is not usable behind the lead-in; what would be
    needed is named and not built. The run **without** the lead-in is the existing weekly
    test `the_recalibrated_rewarded_run_at_256_units_on_four_workers_exhaustive`, held to
    ADR-0066's table and run again on the dispatch, rather than a third run added to the
    weekly job.
- [x] **The gate.** One test at 256 units, running no whole run: the first four windows of the
  settling measurement, held to the first four rows of its pinned table, as ADR-0061's gate test
  is held to the first row of its own. Nothing else added to the gate.
  - **Done:** `the_first_four_windows_of_the_settling_at_256_units_and_the_rules_over_its_tables`;
    the one test also carries the clause at its edges, the lead-in as derived from the pinned
    table and the criterion at its edges over the pinned runs, which run nothing heavy, so
    that the gate grows by one test and still holds the rules.
- [x] **The evidence.** `gh workflow run ci.yml --ref <branch> -f scope=both`
  ([ADR-0067](../../docs/adr/0067-the-weekly-dispatch-has-a-scope.md): this round adds tests), green
  in every job, every pinned number reproduced on the hosted runner, no survivor; its run id, the
  six runtime shards' times, the runtime suite's time before and after, and the weekly exhaustive
  job's time against its 120-minute bound, all in the ADR.
  - **Done:** run [35528304550](https://github.com/DescentVTT/VirtualCortex/actions/runs/35528304550); the exhaustive job 69 m 03 s of its 120-minute bound, every pinned number held; the rest in ADR-0070's
    evidence bullet.
- [x] **The documents, in the same pull request.** Whitepaper §11.1's per-unit scaling item — the
  open clause "the item stays open for the windows at which 256 units settle" is answered with
  the window or with the statement that eighty are not enough, and the row's gain keeps its
  Specified status and its precondition (H-8), which this round does not touch; §11.1's **H-12**
  gains a sentence on whether 256 units is usable behind a lead-in; §9, the ADR index,
  `CHANGELOG.md`, and `docs/zh-TW`'s reader's guide if it names the sizes. The whitepaper's
  version moves in **both** declarations with its date
  ([ADR-0064](../../docs/adr/0064-the-documentation-gate-and-the-version.md)). Executable directives
  under every sentence that claims a test or a constant exists.
- [x] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned, the
  frozen banner naming the pull request, the ADR and the criterion's outcome.

## Not empowered

- **No rule of the engine changes.** Not `STDP_A_PLUS_Q1_15`, `STDP_A_MINUS_Q1_15`,
  `STDP_DEPRESSION_REFERENCE_Q1_15`, the windows, `ELIGIBILITY_TAU_SHIFT`, `DOPAMINE_TAU_SHIFT`,
  ADR-0055's magnitude-scaled depression, the inhibitory rule or its period, the estimator, the
  controller, the sleep constants, `Prior` or the reference prior's parameters.
- **No per-unit synaptic scaling**, no second controller, no rate-dependent balance of $A_-$
  against $A_+$, no structural rule, no new learning rule of any kind. §11.1 names those as the
  next controller round's subject and gates the first on H-8; naming one in the ADR is the whole
  of this round's licence about them.
- No change to the instrument: not the geometry or its masks, the rotations, the readout window,
  the gain, the trial, the block, the run, the baseline, the reward, the stimulus, or either
  seed. No recalibration.
- No constant chosen or moved after a run; no lead-in other than the derived one; no criterion
  changed after a run; no clause dropped because it failed.
- No record change, no new section, no format bump, no reserved byte taken; no new crate; no
  dependency in a state crate; no `unsafe` outside the runtime's arena access; no float anywhere.
- No more than one test added to the pull request's gate, and it runs no whole run. No change to
  `.github/workflows/ci.yml`. No renaming of a required check. No move of the determinism pin,
  and no edit to a number ADR-0055, ADR-0065 or ADR-0066 pinned.
- No claim about 1 024 units, which ADR-0055 and ADR-0066 have measured, and none about
  Appendix A's scale. No statement about learning: this round measures an instrument.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in the
ADR that owns it: where the lead-in lives and what it is called (a helper in `instrument.rs`, a
parameter of the harness — not `settle`, which is taken); whether the settling measurement and
the two task runs are one test or three; whether the round writes one ADR or two; which numbers
the ADR restates; and **whether the settling measurement runs to 160 windows instead of 80** —
permitted only when the clause has not held by the eightieth, with the cost stated against the
exhaustive job's bound, and with the lead-in still "the smallest window satisfying the clause, or
the bound". It may not reach the standing directives, the whitepaper's invariants or the
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
gh workflow run ci.yml --ref <this round's branch> -f scope=both
```

Every command exits 0; the in-diff gate reports no survivor in CI. The dispatched run is green in
every job, reproduces every pinned number and finds no survivor over the tree. The constants
commit precedes the first commit that holds a run of Deliverable C. The determinism pin, the
image format and every number ADR-0055, ADR-0065 and ADR-0066 pinned are unchanged.

## Report

The closing message states: where 256 units settle — the smallest window satisfying brief 026's
clause, or that eighty (or 160) are not enough, with the sum's rate of change at the last window
either way; the derived lead-in and the commit that froze it; the criterion, block by block, for
the run behind the lead-in, beside the run without it and ADR-0066's numbers for the same run;
whether 256 units is usable for this task and what that means for the cost of a later learning
round; the runtime suite's time before and after, the weekly exhaustive job's time against its
bound, and what the mutation gate and the sweep found; what was not done and why; and what the
re-examination after the round recommends next.
