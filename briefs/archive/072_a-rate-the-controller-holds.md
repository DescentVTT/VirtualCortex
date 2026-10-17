---
status: archived
date: 2026-10-13
integrity: sha256-471d317a4fd719c0994a9104b83a62ecbe537f4a7db21cae201509b6aa7512c7
---

> **Executed 2026-10-13 (UTC) in pull request #208.** Writes ADR-0180 (a rate the controller holds, built) and
> ADR-0181 (measured); opens no finding. The rule is built in `cortex-homeostasis` and composed by the executor,
> its target a period of the image at the modulator section's `[56..60)`, and the image's format is 22. Unset,
> every run is the run it was: no pinned number of an earlier round moved but a whole-image pin with the format,
> six of them re-pinned and twenty-five held through the version written back. The protocol — the settled
> network's rule under the rate's rule, the six steps, the target, the bound, H-38's clauses and constants and the
> four predicted readings' rules — was committed and pushed before any lead-in was run under the rate's rule; the
> tree's 120 whole-domain tests then passed with the target unset, 120 of 120.
>
> **H-38 is yes, as ADR-0178 predicted, and its stopping rule reached step 3.** Clause 0 holds at every step of
> the six: each lead-in settles at the window its sums do, 96 to 102, every later rule gives a value, and the
> frozen block passes both measures, the stimulus seen in 62 to 64 trials of 64. The step taken is an eighth, the
> largest. Under it H-25's two arms hold H-25's four clauses: 118 to 124 of each mapping's last 128 correct, each
> reversal passed in 15 to 19 blocks, the highest coupling 1.220 of its image's, the outside sum at the image's.
> The controller holds the count at its target, 2 306 spikes a window where the gain held's arms hold 2 814 to
> 2 838, with the gain two per cent below its image's and alternating. Each of the four predicted readings held.
> The steepness of the rate in the gain reads fifteen where ADR-0178 took nine to fourteen, so an eighth is at
> its loop's edge and a sixteenth well inside it; no arm ran under a sixteenth. The next decision, an ADR on what
> the engine carries by default, the rule and its step, is named and not taken.
>
> Every deliverable below is done and its box ticked. The weekly was dispatched at `scope=both`, run 38417864748;
> its twelve whole-domain shards are green, and its sweep had not ended when the round asked for its merge, which
> ADR-0181 says. No sweep was owed. Seven things differ from the brief's letter or add to it, each by its
> empowerment clause or recorded as what it is, and each in ADR-0180 or ADR-0181:
> - **the round writes two ADRs**, the build and the measurement;
> - **the rule is a second function**, `regulate_rate`, the two rules sharing the move and the clearing, and the
>   target is the caller's count, as the saturation ceiling is; the target sits in the modulator section, the
>   homeostasis record being full, and the loader refuses a period whose count is no spike a window or more
>   than a window's bins count;
> - **the gain's mean's criterion asks for both blocks of sixteen windows whole**, so the settled network's rule
>   gives no window below the thirty-second; ADR-0178 does not say what stands for windows that did not run, and
>   this was decided before any run;
> - **twenty-five whole-image pins keep their numbers** and are held through the version written back to 21,
>   where the brief speaks of the pins moving under a masked check; the six that carried a pin written back
>   moved, by one mask;
> - **every arm the tree runs through `arm_on` now reads its gain window by window**, not H-38's alone;
> - **the arms ran twice, where the round's rule is once.** The watch the protocol added to every arm counted a
>   trial's ticks and missed the 500 the run steps before an arm's first trial; two spikes fell there in each of
>   H-38's arms. The watch was repaired and the arms run again: every table of the first run reproduced, and no
>   constant, criterion or rule of the protocol moved;
> - **the sixth re-pinned image is read by a paused test**, run by hand to its reading: no weekly test holds it.
>
> Relative links gained one `../` so that they resolve from `archive/`; no word, claim or figure changed.
>
> *The body below describes the tree before execution and is not maintained.*

# Brief 072: A rate the controller holds — ADR-0178's rule built: the controller's gain moved by the window's count over a target in the estimate's place, the target a period in ticks of the image and zero unset; then six steps scanned under it on seed 22 at 1 024 units and H-25's two arms under the largest that holds; H-38 run once

## Mission

**This brief builds one rule of the controller and runs one hypothesis once.** H-37 read that the criticality
controller as built holds the network at its saturation rule's rate, 24.4 Hz, at every step, its estimate reading
near one at no rate ([ADR-0177](../../docs/adr/0177-the-controllers-step-measured.md)).
[ADR-0178](../../docs/adr/0178-a-rate-the-controller-holds.md) gives the controller a second rule, ADR-0036's option 2
reopened: the gain moved by the window's count over a target, in the estimate's place. The target is the settled
network's own rate, by the rule that already derives the inhibitory target.

ADR-0178 wrote **H-38** before any run and **predicts yes**: at one step of six the lead-in settles and the frozen
block passes both measures, and under the largest such step H-25's two arms hold H-25's four clauses with the
controller on. It says what would refute that.

When the round is done, the tree holds:
- the rate's rule in `cortex-homeostasis`, composed by the executor, a parameter of the image, with its own ADR and
  tests, and **the controller as built bit for bit where it is unset**;
- each step of the scan under the rule as a weekly `exhaustive` test, and H-25's two arms under the largest step that
  holds, where one does;
- H-38's verdict and **the step of its stopping rule reached**, with the next decision it names and does not take.

**Do not run this round beside brief 065's**: both dispatch the weekly workflow.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). The one mechanism is ADR-0178's: a rule of the
  controller's own form with the window's count over its target in the estimate's place. **Nothing else is
  adopted**:
  - no change to the estimate, the saturation rule, the gain's bounds, the step's bound, the window or the bins;
  - no gain of a unit's own (§8.8), and no second target;
  - no change to the critic, the address, the signed gate, STDP, the membrane or the inhibitory rule;
  - the whole punishment of ADR-0155 and the exploration of ADR-0163 stay unset, and the feedback is the true one;
  - no dependency, and no version bump of a tool.
- **Unset, bit for bit.** With the target at zero the controller is ADR-0036's: every run is the run it was, every
  pinned number holds — ADR-0053's day, ADR-0077's two candidates and H-37's five steps among them — and the
  determinism pin does not move. Where the image's format moves, the whole-image pins move with it under a masked
  check, as ADR-0095, ADR-0131, ADR-0134 and ADR-0155 did.
- **The state crate's invariants hold** (`CLAUDE.md`): `#![no_std]`, no dependency, no `unsafe`, no `f32`/`f64`;
  every operation on a state field saturates or wraps by name
  ([ADR-0029](../../docs/adr/0029-structural-enforcement.md)); a record's size and alignment asserted at compile time;
  any change to a record or a section bumps `CortexFileHeader::version`, updates its table in whitepaper §5.2 and
  gets a changelog entry. A new rule carries a test over the lattice of `testkit/prop.rs`.
- **One network**: ADR-0044's prior at 1 024 units from seed 22, H-25's.
- **The target is the period H-25's derivation returns with the gain held**, 58 201 ticks on seed 22. No other target
  is run.
- **The steps are an eighth, a sixteenth, one in 32, one in 64, one in 128 and one in 256**, in that order. No other
  step is run.
- **The rule is on from the lead-in's first window**, and the step and the target are carried in the image through
  the frozen block and every trial of an arm.
- **A step at which a rule gives no value, or the frozen block fails a measure, is a reading of H-38** (its clause 0).
  It stops that step, not the round, and it is not repaired.
- **H-38's clauses, its constants, the lead-in's bound and criteria and the four predicted readings are written
  before any lead-in is run under the rate's rule** and do not move after it. There is no second attempt.
- **The prediction is ADR-0178's**: yes. The round reports it against what was read, and each predicted reading as
  held or missed.
- **The engine and the harness are read before a description of them is trusted**, this brief's and ADR-0178's
  included.
- **No capacity of a run's configuration is reached silently.**
- Every loop ends by construction ([ADR-0062](../../docs/adr/0062-the-first-complete-sweeps-list.md)).
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive`. For the measurement, the runtime's gate grows
  by at most one test ([ADR-0061](../../docs/adr/0061-the-learning-runs-leave-the-gate.md)); the build's own tests are
  beside it. The mutation gate on the changed lines must pass
  ([ADR-0030](../../docs/adr/0030-verification-governance.md)).
- **The round is evidenced as `briefs/README.md`'s "Evidencing a round" says**
  ([ADR-0150](../../docs/adr/0150-a-round-waits-for-what-it-checks.md),
  [ADR-0158](../../docs/adr/0158-the-sweep-off-the-next-decisions-path.md)): the runs run once, the dispatch's scope
  follows the diff, the merge waits for the whole-domain shards and not for the sweep, the ADR cites the pull
  request's commits, the commit that asks for the merge sets it to `accepted`, and a sweep that has ended and that no
  ADR has recorded is recorded by this round's.
- **The pull request leaves draft with the commit that asks for the merge.**
- **No shard of the weekly job passes 60 per cent of its bound**, 180 minutes, under the regenerated deal
  ([ADR-0179](../../docs/adr/0179-the-bounds-rule-restated.md)); one that does is reported with its job's minutes.
- No pinned number of an earlier round moves, but a whole-image pin with the format.
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-10-13 against `main` at `580d07e`, after ADR-0177 merged. Line numbers move; the symbols and the
quoted sentences are what to re-derive.

1. **The record** (`crates/cortex-homeostasis/src/lib.rs`, `HomeostaticDrivePool`): sixty-four bytes, every one
   named, none reserved. `count_activity`, `close_bin`, `estimate_branching_ratio`, `is_saturated(saturation_per_bin)`
   and `regulate(saturation_per_bin)`, which *"moves the gain by $g \leftarrow g\,(1 - \kappa\,\operatorname{clamp}(\hat
   \sigma - 1, -1, 1))$, the product rounded to nearest and clamped to"* `GAIN_MIN_Q16` and `GAIN_MAX_Q16`, and clears
   the window. A window's spikes are `sum_prev` and `last_activity` together, as `is_saturated` reads them.
2. **The composition** (`runtime/cortex-runtime/src/executor.rs`, `tally`): on the bin's cadence `close_bin`; on the
   window's, `regulate(saturation_ceiling(units))`, then `step_sleep`. `Config::control_step_q0_16`; a step of zero
   leaves the gain.
3. **Parameters of the image** (`runtime/cortex-runtime/src/image.rs`): `FORMAT_VERSION` 21. The modulator section
   carries the inhibitory target period at `[20..24)`, the critic's flag, shift and scale at `[48..51)`, the whole
   punishment's flag at `[51]` and the critic's window at `[52..54)`; the homeostasis record is its own section.
   `Config::istdp_target_period_ticks`, refused outside `ISTDP_PERIOD_MIN_TICKS` and `ISTDP_PERIOD_MAX_TICKS`.
4. **ADR-0036's option 2**, and why it was set aside: *"Option 2 needs a target rate per image with no canonical
   value; $\sigma = 1$ is canonical. A rate ceiling is kept as the guard for the regime the slope cannot read."*
5. **The target's rule** (`runtime/cortex-runtime/tests/inhibition.rs`, `target_period(units, ticks, spikes)`,
   ADR-0128): over the settled image frozen under the reference drive; `TARGET_PERIOD_1024` 58 201 on seed 22, and
   59 510, 59 179, 59 599 and 59 119 on the other networks read. At 1 024 units a period of 58 201 ticks is 2 306
   spikes a window of $2^{17}$ ticks.
6. **H-37's harness** (ADR-0177): `Settling` — the step a lead-in runs under and its image carries, whether the
   settled network's rule reads the gains beside the sums, the bound — with `GAIN_HELD`, `AT_AN_EIGHTH` and
   `under_step(step)`; `derive_under(units, seed, settling, name)`; `lead_in_until`; `settled_in`, `settled_in_both`;
   `SCAN_BOUND` 320; `step_read`; the five weekly tests `the_controller_s_step_of_…_at_1024_units_exhaustive` and
   `the_derivation_at_an_eighth_is_the_controller_s_candidate_at_1024_units_exhaustive`; their pinned tables `SCAN_*`.
   An arm is `arm_on(ground, share, arm)` (ADR-0174).
7. **What H-37 read**: with the gain held, 2 132 to 2 462 spikes a window over ninety-six windows; at one in 256 the
   gain and the rate after windows 1, 16 and 32 at 1.755 and 1.8 Hz, 1.851 and 3.5 Hz, 1.960 and 5.9 Hz; at an eighth
   the first two windows 2 446 and 7 918 spikes at gains of 1.750 and 1.906.
8. **ADR-0177 on the settled network's rule**: at a step of one in 128 or less *"the gain's swing fits inside
   ADR-0055's tolerance, and the rule reads the network as settled."* ADR-0178 holds the gain to its mean instead: the
   mean of sixteen windows within 75 per ten thousand of the mean of the sixteen before.
9. **The weekly job**: twelve shards of 180 minutes, each running its tests two at a time. The cost table holds 120
   tests and 93 020 s, read on a slow run.
10. **The dispatch's scope** ([ADR-0075](../../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)): a round that
    changes a file under `src/` dispatches `scope=both`. The sweep's thirteen jobs start when the shards have ended
    ([ADR-0166](../../docs/adr/0166-the-sweep-after-the-shards.md)).
11. **No sweep is owed** when this brief is written: ADR-0174 records the last that ended.

## Deliverables

- [x] **The rule built, in a new ADR at the next free number (`ls docs/adr`).**
  - *The rule*, in `cortex-homeostasis` beside the estimate's: once a window, the gain moved by
    $g \leftarrow g\,(1 - \kappa\,\operatorname{clamp}(c / c^{*} - 1, -1, 1))$, $c$ the window's spikes and $c^{*}$
    the target's count, with the step, the clamp, the rounding and the gain's bounds the estimate's rule has. The
    estimate is still computed and kept as a reading; the saturation rule is not consulted.
  - *The target*: a period in ticks, a parameter of the image, zero unset; the executor makes the window's count of
    it, the units times the window's ticks over the period. Where it sits, what the loader refuses and what a count
    of zero means are this ADR's to decide.
  - *The format*: `CortexFileHeader::version` moved, the section's or the record's table in whitepaper §5.2, the
    whole-image pins under a masked check.
  - *The tests*:
    - unset, a window is regulated as it was, bit for bit, over the lattice;
    - set, the rule against a hand rule at its edges: a count at its target, at twice it and more, at none, one above
      and one below; the gain at each bound; a step of zero; the rounding;
    - over the lattice of `testkit/prop.rs`, the rule against an oracle in wider integers;
    - the executor: the count made of the period at 256, 1 024 and 2 048 units, and a run under the rule held to the
      rule window by window;
    - the image: the parameter carried, an image of the format before it refused or read as the loader's rule says,
      and unset the bytes it had.
- [x] **H-38's protocol, in an ADR, before any lead-in is run under the rate's rule.**
  - The network, the target and the steps in their order.
  - The settled network's rule under the rule, as an integer rule: the first window at which ADR-0055's criterion
    holds over the excitatory sums and the gain's mean over those sixteen windows is within 75 per ten thousand of
    its mean over the sixteen before, within 320 windows; nothing otherwise.
  - What every later rule returns when nothing passes, and that a step holds when every rule gives a value and the
    frozen block passes both measures.
  - H-38's clauses and constants, restated from ADR-0178 and pinned in the tests.
  - The four predicted readings as rules over the tables: a step of a sixteenth holds; no window of any lead-in
    under the rule holds 32 768 spikes or more; through each arm the gain after every window within 1.6625 and
    1.8375; each arm's mean gain below the gain its image carries.
  - The readings, no clause, as ADR-0178 lists them.
- [x] **The calibration**, before any such run: every pinned number of the tree holds with the target unset, H-37's
  five steps and ADR-0077's candidates among them.
- [x] **The scan**: each of the six steps a weekly `exhaustive` test, run once: its lead-in pinned by window, what
  each rule returned or that it returned nothing, and the frozen block's readings where it was reached.
- [x] **The arms**, where a step holds: H-25's two under the largest step that holds, each a weekly `exhaustive`
  test, run once, their tables pinned per block, the gain's course through the arm and through each reversal among
  them. Where none holds, no arm is run and the ADR says so.
- [x] **The ADR's reading.**
  - H-38's verdict by clause, and for clause 0 what each step returned.
  - The prediction and each of the four predicted readings against what was read.
  - Per step, beside the gain held and beside H-37's steps: the gain's course and band, the count beside its
    target, the steepness of the rate in the gain as the tables give it, and the frozen block's readings.
  - Through each arm: the gain's course and what a reversal does to it, H-25's readings beside H-25's with the gain
    held, and the inhibitory sum's course beside it.
  - **The step of the stopping rule reached, and the next decision it names, not taken.**
- [x] **The gate.** The build's tests, and at most one runtime test for the measurement: the settled network's rule
  with the gain's mean at its edges, clause 0 and the predicted readings' rules over tables written by hand.
- [x] **The evidence**, as "Evidencing a round" says. A weekly dispatched on this round's branch at `scope=both`. Its
  twelve whole-domain shards are green and reproduce every pinned number. The cost table is regenerated from their
  artifacts. The ADR names the dispatch and each shard's job time against its bound. The sweep's outcome is not
  waited for.
- [x] **The documents, in the same pull request.**
  - Whitepaper §5.2 where a record or a section changes; §8.8, where the controller is described; §1.6; §11.1's H-38
    with its verdict and step; §9.
  - The ADR index and `CHANGELOG.md`.
  - `CLAUDE.md` where the standing changes (ADR-0150): the image's format, and what the engine learns under;
    `README.md` and `docs/zh-TW`'s reader's guide as the result requires.
  - The whitepaper's version in both declarations, with its date
    ([ADR-0064](../../docs/adr/0064-the-documentation-gate-and-the-version.md)).
- [x] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned, the round's ADRs set to
  `accepted` by the commit that asks for the merge, and the pull request marked ready for review with it.

## Not empowered

- **No rule of the engine changes but the one ADR-0178 adds.** The estimate, the saturation rule, the bins and the
  window are as they are; unset, the controller is ADR-0036's.
- The rule's form is ADR-0178's: the estimate's rule with the count over its target in the estimate's place. Another
  form of the error, an integral term or a second time constant is not this round's.
- The target is the period H-25's derivation returns with the gain held; the steps are the six; the network is seed
  22 at 1 024 units; the lead-in's bound is 320 windows. None is fitted to the result, and none other is run.
- No rule that derives a constant is changed so that a step holds. The settled network's rule under the rate's rule
  is ADR-0178's.
- H-38's clauses, its constants and its predicted readings do not move after a lead-in is run under the rate's rule.
  There is no second attempt.
- No pinned number of an earlier round moves, but a whole-image pin with the format.
- No change to `.github/workflows/ci.yml`, `scripts/exhaustive-shard.sh` or `scripts/exhaustive-costs.mjs`; the cost
  table changes only by regeneration from this round's dispatch.
- No float anywhere, no dependency, and no `unsafe` beyond ADR-0023's invariant.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- the rule's name and signature, and how it stands beside `regulate`: a second function, or `regulate` given what it
  regulates, with every call of the tree the call it was;
- where the target sits in the image, the record being full, and what the loader refuses;
- how the executor makes the window's count, and what it does where the count would be zero;
- how the derivation takes the rule: a `Settling` that carries the target, with every derivation of the tree the one
  it was;
- how the gain's mean is read as an integer rule, and how the calibration shows H-37's steps unmoved;
- how each step's lead-in and frozen block and each arm are dealt into tests, and whether the local runs go one after
  another or side by side;
- how each reading is computed and pinned;
- whether the round writes one ADR or two (the build and the measurement).

It may not:
- change the estimate's rule, the saturation rule or any other rule of the engine;
- change the rule's form, the target, a step, the bound, the criteria, the network, the task or the schedule;
- move a clause, a constant or a predicted reading after a lead-in is run under the rate's rule;
- reach the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

If the calibration fails, the round stops there as H-38's stopping rule says: the finding is written, and no lead-in
is run under the rate's rule.

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
gh workflow run ci.yml --ref <this round's branch> -f scope=both
node scripts/exhaustive-costs.mjs from <the dispatch's downloaded artifacts> --run <its id> > scripts/exhaustive-costs.tsv
```

Every command exits 0. The dispatched run's twelve whole-domain shards are green and reproduce every pinned number.
In the history, the rule, its tests, the target, the steps, the criteria, H-38's constants and its predicted readings
precede any lead-in run under the rate's rule.

## Report

The closing message states:
- the rule as built, where its target sits, the format, and that unset every run is the run it was;
- the protocol, and when it was committed;
- per step, what each rule returned and the frozen block's measures;
- H-38's verdict by clause, and the prediction and each predicted reading against what was read;
- the gain's course and the count beside its target per step, and the steepness read;
- whether arms ran, what they read beside H-25's, and what the gain did through a reversal;
- the step of the stopping rule reached and the next decision it names, not taken;
- that no pinned number moved but with the format;
- the mutation gate's result on the changed lines;
- the scope ADR-0075 gave the diff, each shard's job time against its bound, the regenerated cost table and the
  sweep's state when the round ended;
- that the pull request is ready for review;
- what was not done and why.
