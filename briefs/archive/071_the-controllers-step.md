---
status: archived
date: 2026-10-13
---

> **Executed 2026-10-13 (UTC) in pull request #206.** Writes ADR-0177 (the controller's step, measured); opens
> F-68, on the dispatched run's longest test and ADR-0176's rule for a shard's bound, and changes no workflow for
> it. No file under `src/` and no file of a state crate changed, and no pinned number of an earlier round moved.
> The protocol — the derivation given how it settles its network, the settled network's rule under a step,
> the steps, the bound, the criterion, H-37's clauses and constants and the three predicted readings' rules — was
> committed and pushed before any lead-in was run under a step of the scan; the tree's 114 whole-domain tests and
> the calibration at an eighth then passed, 115 of 115.
>
> **H-37 is no, on clause 0, at every step, as ADR-0175 predicted, and its stopping rule reached step 5.** At a
> sixteenth, one in 32 and one in 64 the lead-in settles in its sums by the 37th, the 41st and the 51st window and
> in its gain at none of 320. At one in 128 and one in 256 it settles, at the 71st and the 114th, and the frozen
> block fails both measures: the volley 48.5 and 48.7 units of 51 a presentation, the stimulus seen in 46 and 49
> trials of 64. Every step holds the network at the saturation rule's rate, 24.2 to 24.5 Hz a unit on average, at a
> gain of 2.49 to 2.50 with the excitatory sum at 0.45 of the prior's; half its windows are at the estimate's
> ceiling and the gain goes a step up and a step down in turn. The gain's band narrows with the step, 2.295 to
> 2.654, 2.419 to 2.581, 2.458 to 2.540, 2.483 to 2.515 and 2.488 to 2.507, and does not move. The three predicted
> readings each held: the ceiling in every lead-in; no window of 1 145 with an estimate between a half and two, the
> greatest below the ceiling 0.498; every lead-in ending at 33 002 to 42 021 spikes. No step holds, so no arm was
> run. The next decision, an ADR on what the controller regulates, its estimate or a rate, is named and not taken.
>
> Every deliverable below is done and its box ticked. The arms' box is ticked by its last sentence: no step holds,
> no arm is run, and ADR-0177 says so. The weekly was dispatched at `scope=exhaustive`, run 38352602613; its twelve
> whole-domain shards are green, none past 48 per cent of its bound, and it dispatched no sweep. No sweep was
> owed. Seven things differ from the brief's letter or add to it, each by its empowerment clause, and each recorded in ADR-0177:
> - **the derivation takes how it settles its network as one value** (`Settling`: the step, whether its rule reads
>   the gains, and its bound) where the brief asked for its step. ADR-0077's second candidate was held to the sums
>   alone within 160 windows and its gains settle at none of them, so under the scan's rule the calibration at an
>   eighth would not have reached the frozen block the brief names; it runs under ADR-0077's own rule
>   (`AT_AN_EIGHTH`);
> - **the calibration at an eighth is a weekly test of its own**, a sixth whole-domain test beside the five steps;
> - an estimate *between a half and two* counts both ends, read before the run as the reading least kind to the
>   prediction;
> - **no capacity of `config` was sized**: the train is asserted window by window and each step pins its greatest
>   window, 43 631 spikes at most of the 335 872 the ring holds; what would follow a loud failure was written
>   before the run and did not arise;
> - **no arm was built**, before the scan or after it: where a step holds the body is designed in ADR-0177, and no
>   step held;
> - the lead-in's reading carries an oracle of the controller's rule (`regulated`), and every window's gain is held
>   to it; ADR-0175's sentences about ADR-0077's two lead-ins are held by rule in the gate;
> - H-37's names in the tests are `SCAN_*`, `scan_*` and `controlled*`.
>
> Relative links gained one `../` so that they resolve from `archive/`; no word, claim or figure changed.
>
> *The body below describes the tree before execution and is not maintained.*

# Brief 071: The controller's step — ADR-0175's question: with the criticality controller on, five steps from a sixteenth to one in 256 scanned on seed 22 at 1 024 units, each held to a lead-in settled in its sums and in its gain and to the frozen block's two measures; H-25's two arms under the largest step that holds; nothing under `src/` changed; H-37 run once

## Mission

**This brief builds nothing in the engine and runs one hypothesis once.** Every learning result was read with the gain
held and the engine's criticality controller off. The tree already holds what the controller does at its step of an
eighth: it raises the gain until its saturation rule fires, and the instrument is drowned
([ADR-0077](../../docs/adr/0077-the-background-side.md)). [ADR-0175](../../docs/adr/0175-the-controllers-step.md) asks the
one lever that needs no build, the step: is there a step at which the controller holds the network where the
stimulus fires once and is seen, and does the learning configuration learn there with the controller on?

ADR-0175 wrote **H-37** before any run, and **predicts no**: at every step of the scan the controller takes the
network to the saturation rule's rate. It names what would refute that, a window whose estimate reads near one.

When the round is done, the tree holds:
- the derivation taking its step, with every pinned number as it was;
- each step of the scan as a weekly `exhaustive` test: its lead-in, what each rule returned, the frozen block's
  measures;
- where a step holds, H-25's two arms under it with the controller on, as weekly `exhaustive` tests;
- H-37's verdict and **the step of its stopping rule reached**, with the next decision it names and does not take.

**Do not run this round beside brief 065's**: both dispatch the weekly workflow.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). The controller is ADR-0036's, run as built.
  **Nothing is adopted**:
  - no change to the estimate, the saturation rule, the gain's bounds or the window;
  - no controller that regulates a rate: ADR-0175 names it as the decision after a no, and does not take it;
  - no change to the gate's clamp, a single error's weight, the critic's step or the schedule;
  - the whole punishment of ADR-0155 and the exploration of ADR-0163 stay unset, and the feedback is the true one;
  - no dependency, and no version bump of a tool.
- **No file under `src/` and no file of a state crate changes.**
- **One network**: ADR-0044's prior at 1 024 units from seed 22, H-25's.
- **The steps are a sixteenth, one in 32, one in 64, one in 128 and one in 256**, in that order: `0x1000`, `0x0800`,
  `0x0400`, `0x0200`, `0x0100` in Q0.16. No other step is run, before or after.
- **The controller is on from the lead-in's first window**, and the step is carried in the image through the frozen
  block and every trial of an arm.
- **With the step at zero the derivation returns seed 22's constants**, and at an eighth its lead-in is ADR-0077's
  second candidate's. Every pinned number holds. The determinism pin does not move.
- **A step at which a rule gives no value, or the frozen block fails a measure, is a reading of H-37** (its clause 0).
  It stops that step, not the round, and it is not repaired.
- **H-37's clauses, its constants, the lead-in's bound and criterion and the three predicted readings are written
  before any lead-in is run under a step of the scan** and do not move after it. There is no second attempt.
- **The prediction is ADR-0175's**: no, on clause 0, at every step. The round reports it against what was read, and
  each predicted reading as held or missed.
- **The engine and the harness are read before a description of them is trusted**, this brief's and ADR-0175's
  included.
- **No capacity of a run's configuration is reached silently.** Under the controller the population fires at ten
  to twenty times the rate the runs were read at: the round shows that the train held every window read from it, that
  no injection was refused and that no worker dropped a spike.
- No `f32`/`f64`, oracles included; every operation on a state field saturates or wraps by name
  ([ADR-0029](../../docs/adr/0029-structural-enforcement.md)). Every loop ends by construction
  ([ADR-0062](../../docs/adr/0062-the-first-complete-sweeps-list.md)): a lead-in is a `for` over its bound.
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive`. The runtime's gate grows by at most one test
  ([ADR-0061](../../docs/adr/0061-the-learning-runs-leave-the-gate.md)). The mutation gate on the changed lines must pass
  ([ADR-0030](../../docs/adr/0030-verification-governance.md)); with no line under `src/` changed it has nothing to
  mutate, and says so.
- **The round is evidenced as `briefs/README.md`'s "Evidencing a round" says**
  ([ADR-0150](../../docs/adr/0150-a-round-waits-for-what-it-checks.md),
  [ADR-0158](../../docs/adr/0158-the-sweep-off-the-next-decisions-path.md)): the runs run once, the dispatch's scope
  follows the diff, the merge waits for the whole-domain shards, the ADR cites the pull request's commits, the commit
  that asks for the merge sets it to `accepted`, and a sweep that has ended and that no ADR has recorded is recorded by
  this round's.
- **The pull request leaves draft with the commit that asks for the merge.**
- **No shard of the weekly job passes 60 per cent of its bound** under the regenerated deal. The bound is 180 minutes
  ([ADR-0176](../../docs/adr/0176-a-shards-bound-by-its-longest-test.md)).
- No pinned number of an earlier round moves.
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-10-13 against `main` at `3abb601`, after ADR-0174 merged. Line numbers move; the symbols and the
quoted sentences are what to re-derive.

1. **The controller** (`crates/cortex-homeostasis/src/lib.rs`, `HomeostaticDrivePool::regulate`): once a window,
   *"moves the gain by $g \leftarrow g\,(1 - \kappa\,\operatorname{clamp}(\hat\sigma - 1, -1, 1))$"*, with $\kappa$ the
   record's `control_step_q0_16`, at most `CONTROL_STEP_MAX_Q0_16`; the estimate is the window's lag-one slope, or
   `SIGMA_MAX_Q16`, 16, for a window `is_saturated`; the gain is clamped to `GAIN_MIN_Q16` and `GAIN_MAX_Q16`, 0.25 and
   4.0. A window is $2^{17}$ ticks, thirty-two bins. The executor's ceiling is `saturation_ceiling(units)`
   (`runtime/cortex-runtime/src/executor.rs`), *"one spike per unit per bin on average"*.
2. **The harness's step** (`runtime/cortex-runtime/tests/instrument/harness.rs`): `CONTROL_STEP_EIGHTH` `0x2000`;
   `BACKGROUNDS = [0, CONTROL_STEP_EIGHTH]`; `SETTLED = 0`; `candidate_from(p, units, gain, step)`;
   `lead_in_until_settled_over(…, bound)` and `lead_in_window`, whose `LeadInWindow` holds the population's spikes,
   the readouts' spikes, the inhibitory and the excitatory sum, the fraction at the target, and *"the gain and the
   estimate as the window's regulation left them"*; `settled_within(prior, sums)` with `SETTLED_WINDOWS` 16 and
   `SETTLED_PER_MYRIAD` 75; `LEAD_IN_BOUND` 80 and `LEAD_IN_EXTENDED` 160; `quiet_within`; `frozen_image` and
   `frozen_from`. `controller_read` applies `settled_within` to a lead-in's gains.
3. **`config(units, workers, baseline)`** sets `control_step_q0_16: 0`. An engine decoded from an image carries the
   image's homeostasis record, its gain and its step.
4. **The derivation** (`tests/inhibition.rs`, `derive_at(units, seed, name)`): its settled network is
   `candidate_from(&p, units, gain, BACKGROUNDS[SETTLED])`, and the frozen image is asserted to carry
   `BACKGROUNDS[SETTLED]`. Several tests assert `(SETTLED, BACKGROUNDS[SETTLED]) == (0, 0)`. An arm is
   `arm_on(ground, share, arm)` (ADR-0174).
5. **ADR-0077's two candidates, pinned**: `BACKGROUND_LEAD_IN_1024`, `SETTLED_AT_1024` `[Some(96), Some(35)]`,
   `BACKGROUND_MEASURES_1024` `[[true, true, false], [false, false, false]]`, `CONTROLLER_READ`; the weekly tests
   `the_settled_network_at_1024_units_exhaustive` and `the_controller_at_1024_units_exhaustive`
   (`tests/instrument.rs`), 176 and 80 s in the last dispatch.
6. **What the controller's lead-in at an eighth reads**: the windows ran at gains of 1.750, 1.906, 2.047, 2.216, 2.385
   and 2.606 with 2 446, 7 918, 13 799, 22 710, 31 858 and 44 560 spikes, the estimate 0.287, 0.409, 0.339, 0.390,
   0.258 and then 16; from the sixth window the gain alternates between 2.21 and 2.80. Over its thirty-five windows
   the estimate reads 0.00 to 0.41, or 16 in fourteen of them, and nothing between. With the gain held it reads 0.00
   to 0.45 over ninety-six windows, at 2 132 to 2 462 spikes a window.
7. **The saturation rule's rate at 1 024 units**: 1 024 spikes a bin, 32 768 a window, 24.4 Hz a unit.
8. **The weekly job**: twelve shards of 180 minutes (ADR-0176, merged with this brief), each running its tests two at
   a time. The cost table holds 114 tests and 81 785 s.
9. **The dispatch's scope** ([ADR-0075](../../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)): `scope=exhaustive`
   for a round that changes no file under `src/`.
10. **No sweep is owed**: ADR-0174 records the schedule's weekly one, and brief 070's dispatch ran none.

## Deliverables

- [x] **The derivation given its step, in the tests.**
  - `derive_at` and what it calls take the step where they read `BACKGROUNDS[SETTLED]`.
  - With the step at zero, on seed 22, each rule returns what the tree pins and every pinned table holds.
- [x] **H-37's protocol, in a new ADR at the next free number (`ls docs/adr`), before any lead-in is run under a step
  of the scan.**
  - The steps, in their order, and the network.
  - The settled network's rule under a step, as an integer rule: the first window at which ADR-0055's criterion holds
    over the excitatory sums and over the gains, within a bound of 320 windows; nothing otherwise.
  - What every later rule returns when nothing passes, and that a step holds when every rule gives a value and the
    frozen block passes both measures.
  - Which capacity of `config` a run under the controller could reach, and how the round shows it was not reached.
  - H-37's clauses and constants, restated from ADR-0175 and pinned in the tests.
  - The three predicted readings as rules over the tables: in every lead-in the estimate at its ceiling in one window
    at least; no window of any lead-in with an estimate between a half and two; every lead-in's last window holding
    more than five times 2 308 spikes.
  - The readings, no clause, as ADR-0175 lists them; **the estimate by the rate of its window** over every window of
    every lead-in among them.
- [x] **The calibration**, before any such run: every pinned number of the tree holds; with the step at zero the
  derivation returns seed 22's constants; at an eighth the lead-in the derivation runs is ADR-0077's second
  candidate's, window for window, and the frozen block from that candidate's thirty-fifth window reads the measures
  ADR-0077 pinned.
- [x] **The scan**: each of the five steps a weekly `exhaustive` test, run once: its lead-in pinned by window, what
  each rule returned or that it returned nothing, and the frozen block's readings where it was reached.
- [x] **The arms**, where a step holds: H-25's two under the largest step that holds, each a weekly `exhaustive`
  test, run once, their tables pinned per block, the gain's course through the arm among them. Where none holds, no
  arm is run and the ADR says so.
- [x] **The ADR's reading.**
  - H-37's verdict by clause, and for clause 0 what each step returned.
  - The prediction and each of the three predicted readings against what was read.
  - Per step, beside the gain held and beside an eighth: the gain's course and the band it keeps, the rate's course,
    the windows the saturation rule fires in, the sums' courses and the frozen block's readings.
  - The estimate by rate, over the five lead-ins: whether it reads near one anywhere.
  - **The step of the stopping rule reached, and the next decision it names, not taken.**
- [x] **The gate.** At most one runtime test: the settled network's rule over sums and gains at its edges, clause 0 and
  the predicted readings' rules over tables written by hand, and the steps and their bound.
- [x] **The evidence**, as "Evidencing a round" says. A weekly dispatched on this round's branch at
  `scope=exhaustive`. Its twelve whole-domain shards are green and reproduce every pinned number. The cost table is
  regenerated from their artifacts. The ADR names the dispatch and each shard's wall time against its bound.
- [x] **The documents, in the same pull request.**
  - Whitepaper §11.1's H-37 with its verdict and step, and §9.
  - The ADR index and `CHANGELOG.md`.
  - `CLAUDE.md` only where the standing changes (ADR-0150); `README.md` and `docs/zh-TW`'s reader's guide as the
    result requires.
  - The whitepaper's version in both declarations, with its date
    ([ADR-0064](../../docs/adr/0064-the-documentation-gate-and-the-version.md)).
- [x] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned, the round's ADR set to
  `accepted` by the commit that asks for the merge, and the pull request marked ready for review with it.

## Not empowered

- **No file under `src/` and no file of a state crate changes.** No rule, no record, no format.
- No change to the controller: its estimate, its saturation rule, its bounds or its window. No rule that regulates a
  rate, in the engine or in the harness.
- The steps are the five, the network is seed 22 at 1 024 units and the lead-in's bound is 320 windows. None is fitted
  to the result, and none other is run.
- No rule that derives a constant is changed, widened or given a fallback so that a step holds. The settled
  network's rule is ADR-0175's: the criterion over the sums and over the gains.
- H-37's clauses, its constants and its predicted readings do not move after a lead-in is run under a step of the
  scan. There is no second attempt.
- No pinned number of an earlier round moves.
- No change to `.github/workflows/ci.yml`, `scripts/exhaustive-shard.sh` or `scripts/exhaustive-costs.mjs`; the cost
  table changes only by regeneration from this round's dispatch.
- No float anywhere, no dependency, and no `unsafe`.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- how the derivation takes its step: a form given it that `derive_at` calls with zero, with every call of the tree the
  call it was;
- how the settled network's rule reads the gains beside the sums, and how the calibration reaches ADR-0077's tables
  through it, whose lead-in was held to the sums alone;
- a capacity of `config` that a run under the controller would reach: sized by what the run can put in it, with every
  earlier run's as it was, and said in the ADR;
- how each step's lead-in and frozen block are dealt into tests, and whether the local runs go one after another or
  side by side;
- how each reading is computed and pinned, the estimate by rate among them;
- how an arm under a step is built on `arm_on`, and what it reads of the controller through the run.

It may not:
- change a file under `src/` or of a state crate;
- change the controller, a step, the bound, the criterion, the network, the task or the schedule;
- move a clause, a constant or a predicted reading after a lead-in is run under a step of the scan;
- reach the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

If the calibration fails, the round stops there as H-37's stopping rule says: the finding is written, and no lead-in
is run under a step of the scan.

## Verification

```bash
cargo check --workspace --all-targets --locked
cargo test --workspace --locked
cargo test --workspace --release --locked
cargo test --workspace --release --locked -- --ignored exhaustive --list
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked
cargo bench -p cortex-bench --bench hot_path --locked -- --test
cargo +1.85 check --workspace --all-targets --locked
cargo +1.85 test --workspace --locked
npm ci
npm run spec
git diff origin/main...HEAD > target/pr.diff && cargo mutants --workspace --in-diff target/pr.diff
gh workflow run ci.yml --ref <this round's branch> -f scope=exhaustive
node scripts/exhaustive-costs.mjs from <the dispatch's downloaded artifacts> --run <its id> > scripts/exhaustive-costs.tsv
```

Every command exits 0. The dispatched run's twelve whole-domain shards are green and reproduce every pinned number.
In the history, the steps, the bound, the criterion, H-37's constants and its predicted readings precede any lead-in
run under a step of the scan.

## Report

The closing message states:
- how the derivation took its step, and that with the step at zero and at an eighth nothing pinned moved;
- the protocol, and when it was committed;
- per step, what each rule returned and the frozen block's measures;
- H-37's verdict by clause, and the prediction and each predicted reading against what was read;
- the gain's band, the rate and the saturation rule's windows per step, and the estimate by rate over the five
  lead-ins;
- whether arms ran, and what they read;
- the step of the stopping rule reached and the next decision it names, not taken;
- that no pinned number moved and no file under `src/` changed, and which capacity of the configuration, if any, was
  sized;
- the scope ADR-0075 gave the diff, each shard's time against its bound and the regenerated cost table;
- that the pull request is ready for review;
- what was not done and why.
