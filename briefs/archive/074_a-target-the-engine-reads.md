---
status: archived
date: 2026-10-18
integrity: sha256-d7aed212129343abd23ab60bb23528aa440f733c53a37b01609c3eab0a694dd7
---

<!-- spec-brief:banner -->
> **Executed 2026-10-18 (UTC) in pull request [#216](https://github.com/DescentVTT/VirtualCortex/pull/216).** Writes ADR-0189 (the read built, the image at format 23) and ADR-0190 (H-40 measured); opens no finding. H-40 is yes, as ADR-0188 predicted, at step 3 of its stopping rule: on seeds 22, 23 and 24 at 1 024 units, at a sixteenth, the engine's read makes a target on every network, 1.0 to 2.8 per cent above the derived one, each lead-in settles at window 160, and H-25's two arms on each network hold H-25's four clauses; two of the three predicted readings held and the first missed, seed 22's read target above its derived one, the live count still rising through the read. Every run was a dispatch and none ran on the developer machine: the calibration 39120143892, the trial 39120156976, the derivations 39127001541, the arms 39128573344, and the reproduction 39131009704 at scope=both, its twelve whole-domain shards green, two jobs past 60 per cent of their bound under the old deal by the runners' spread; the cost table is regenerated from it. The next decision, the engine's default for an image no derivation has settled, is named and not taken. The body below describes the tree before execution and is not maintained.
<!-- /spec-brief:banner -->

# Brief 074: A target the engine reads — ADR-0188's question: with the engine holding its gain for 128 windows and taking the mean count of the last sixteen as its target, then running the rate's rule at a sixteenth, on seeds 22, 23 and 24 at 1 024 units, H-25's two arms on each; the read built in the engine at format 23; H-40 run once, its runs dispatched

## Mission

**This brief builds one thing in the engine and runs one hypothesis once.** H-39 read that the engine learns and
revises under the rate's rule at a sixteenth on three networks, each under the target its own derivation returns
([ADR-0183](../../docs/adr/0183-a-sixteenth-on-three-networks-measured.md)). That target was derived offline, from a
frozen copy of a network settled by a criterion the engine does not hold. An image no derivation has settled has
none. [ADR-0188](../../docs/adr/0188-a-target-the-engine-reads.md) gives the engine a read: its gain held for 128
windows from its first, the mean count of the last sixteen as its target, the rule from the next window.

ADR-0188 wrote **H-40** before any run and **predicts yes**, with what would refute it and three predicted readings.

When the round is done, the tree holds:
- the read in the engine, unset bit for bit, its progress part of the image, the image at format 23;
- on each of the three networks, the derivation under the read at a sixteenth, and H-25's two arms under it, as weekly
  `exhaustive` tests;
- H-40's verdict per network, arm and clause, and **the step of its stopping rule reached**, with the next decision
  it names and does not take.

**This is the first round under [ADR-0184](../../docs/adr/0184-the-runs-off-the-developer-machine.md),
[ADR-0185](../../docs/adr/0185-the-spec-tools-upgraded-and-spec-brief.md) and
[ADR-0186](../../docs/adr/0186-merge-commits.md).** No whole-domain test runs on the developer machine. The brief is
archived with `spec-brief`. Commits are signed and the branch is never rebased. `briefs/README.md`'s "Evidencing a
round" and "Archiving one" are the rule where this brief is silent.

**Do not run this round beside brief 065's**: both dispatch the weekly workflow.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4, as [ADR-0187](../../docs/adr/0187-the-upgrade-suite-planned.md)
  scopes it). The round depends on nothing new: no crate, no tool and no version bump. A window's count read and then
  held by a rule is decades old, and the read is the engine's own code.
- **The read is ADR-0188's.**
  - The gain is held for 128 windows from the first window the engine runs with the read set, whatever the step.
  - The counts of windows 113 to 128 are summed, and their mean, to nearest, is the target's count.
  - The image's period becomes the units times $2^{17}$ over that count, to nearest.
  - The rule moves the gain from the 129th window.
  - A read set beside a period already set is refused.
  - The read's progress is part of the image.
- **Unset, every run is the run it was**, bit for bit, at format 23. An image of format 22 reads as unset. The
  determinism pin does not move. Every whole-image pin moves by the format's mask alone.
- **The networks are seeds 22, 23 and 24 at 1 024 units, and the step is a sixteenth**, `0x1000` in Q0.16. No other
  network, size, step or span is run.
- **A read that gives no target, a lead-in that does not settle, a rule that gives no value, or a frozen block that
  fails a measure, on a network, is a reading of H-40** (its clause 0). It stops that network, not the round, and it
  is not repaired.
- **H-40's clauses, its constants and its three predicted readings are written before any lead-in is run with the
  read set**, and do not move after it. There is no second attempt.
- **A run runs once, and on the repository's host.**
  - No whole-domain test runs on the developer machine: not the calibration, not a derivation, not an arm, not a
    trial of the harness.
  - Each is a dispatch: `gh workflow run ci.yml --ref <branch> -f scope=exhaustive -f exclude='<this round's own tests>'`
    for the calibration, and `-f scope=arms -f tests='<test> ...'` for the round's runs and trials.
  - A check added to an arm is tried first, by such a dispatch, on an arm the tree pins.
- **The branch is never rebased or force-pushed once a dispatch has run on it**, and is brought up to date by merging
  `main`. Every commit is signed: the ruleset requires it.
- **The prediction is ADR-0188's**: yes. The round reports it against what was read, and each predicted reading as
  held or missed.
- **The engine and the harness are read before a description of them is trusted**, this brief's and ADR-0188's
  included.
- **No capacity of a run's configuration is reached silently.**
- No `f32`/`f64`, oracles included; every operation on a state field saturates or wraps by name
  ([ADR-0029](../../docs/adr/0029-structural-enforcement.md)). Every loop ends by construction
  ([ADR-0062](../../docs/adr/0062-the-first-complete-sweeps-list.md)). No `unsafe`.
- A change to a record's or a section's bytes bumps the image's version, updates its table in whitepaper §5.2, and
  gets a changelog entry, in the same pull request.
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive`. The runtime's gate grows by at most one test
  ([ADR-0061](../../docs/adr/0061-the-learning-runs-leave-the-gate.md)). A new rule carries a test over the lattice of
  `testkit/prop.rs`, and the pull request's mutation gate catches every mutant in the lines it changes
  ([ADR-0030](../../docs/adr/0030-verification-governance.md)).
- **The round is evidenced as `briefs/README.md`'s "Evidencing a round" says.** The runs are dispatches; the
  reproduction's scope follows the diff, which here is `both`; the merge waits for the reproduction's whole-domain
  shards and not the sweep; the ADRs cite the round's commits, which a merge commit keeps; and the commit that asks for
  the merge sets them `accepted`.
- **The pull request leaves draft with the commit that asks for the merge.**
- **No shard of the weekly job passes 60 per cent of its bound**, 180 minutes, under the regenerated deal
  ([ADR-0179](../../docs/adr/0179-the-bounds-rule-restated.md)). One that does is reported with its job's minutes.
- No pinned number of an earlier round moves, but the whole-image pins by the format's mask.
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-10-18 against `main` at `256902a`, after ADR-0183 to ADR-0187 merged. Line numbers move; the
symbols and the quoted sentences are what to re-derive.

1. **The rule** (`crates/cortex-homeostasis/src/lib.rs`, ADR-0180):
   - `HomeostaticDrivePool::regulate_rate(target_per_window)` moves the gain by the window's count over its target,
     through `move_gain`, then clears the window. `regulate` is the estimate's rule. The step `kappa` serves both.
   - A window is 32 bins of $2^{12}$ ticks, $2^{17}$ ticks in all.
   - The record has no byte free.
2. **The executor** (`runtime/cortex-runtime/src/executor.rs`):
   - `tally` sums the workers' spikes into `count_activity`, closes a bin at `BIN_CADENCE`, and at `WINDOW_CADENCE`
     calls `regulate_rate(target)` when `rate_target` is `Some`, `regulate(saturation_ceiling(..))` otherwise, then
     `step_sleep`. *"The cadences are masks on the tick, so a loaded engine continues the window it was written in."*
   - `Config::rate_target_period_ticks`, zero unset (the default); `rate_target_of(units, period)` returns the count,
     or `RateTargetOutOfRange`.
   - The engine has no notion of a settled network, a lead-in or a calibration.
3. **The image** (`runtime/cortex-runtime/src/image.rs`): the modulator section carries the rate's target period at
   `RATE_TARGET` (`56..60`) and the inhibitory rule's at `[20..24)`. `MODULATOR_RESERVED` is `[26..28, 39..40, 54..56,
   60..64]`, nine bytes. `FORMAT_VERSION` is 22. ADR-0180's format bump moved every whole-image pin by one mask.
4. **The derivation** (`runtime/cortex-runtime/tests/inhibition.rs`, `instrument/harness.rs`):
   - `derive(seed)` → `derive_under(units, seed, &GAIN_HELD, ..)`. The lead-in runs from a gain of 1.75 under
     ADR-0044's drive, one window at a time, until ADR-0055's criterion over the excitatory sums holds:
     `SETTLED_WINDOWS` (16), each within 0.75 per cent of the sum sixteen windows before. The bound is
     `LEAD_IN_EXTENDED` (160).
   - Under the rule, `mean_settled_in` adds the gain's mean over sixteen windows against the sixteen before, and the
     bound is `RATE_BOUND` (320).
   - The target: `target_lead_in_at` runs the frozen image, modulation's baseline zero, for `TARGET_LEAD_IN_TICKS`
     ($2^{20}$, eight windows), and `target_period` is the units times $2^{20}$ over the spikes. Seed 22's eight read
     18 449 spikes, giving 58 201. The same period is the inhibitory target (`targeted_image`).
5. **H-39's runs**, pinned:
   - the derivations `the_rate_s_rule_at_a_step_of_a_sixteenth_at_1024_units_exhaustive` (seed 22) and
     `the_rate_s_rule_at_a_sixteenth_on_seed_{23,24}_at_1024_units_exhaustive`;
   - the arms `the_address_drawn_at_a_sixteenth_on_seed_{22,23,24}_from_the_{assignment,mirrored_assignment}_at_1024_units_exhaustive`,
     each re-running its network's derivation first;
   - targets `SIXTEENTH_TARGETS`; the gain-held lead-ins `HELD_RATE_READ` and `SEEDED_HELD_RATE_READ`;
   - every mapping 119 to 126 of its last 128, every reversal in 11 to 19 blocks, and each arm's last sixteen
     windows within 0.14 per cent of its target.
6. **What the live network reads with the gain held** (ADR-0183): seeds 22, 23 and 24 settle at windows 96, 106 and
   99, and their last sixteen windows stand 2.7 per cent below, 0.47 above and 0.60 above their derived targets of
   2 306, 2 255 and 2 268 a window.
7. **The runs on the host** (ADR-0184):
   - `scope=arms` runs each named test in a job of its own on `ubuntu-24.04-arm`, through `scripts/arm-run.sh`. The
     log, the seconds and the listing are kept as `arm-<n>`, numbered in the names' sorted order: read `arm.tsv` for
     the name.
   - `scope=exhaustive -f exclude='..'` skips the named tests in the shards, and fails before any test runs if a name
     is not listed.
   - On ARM, H-38's arms ran in 999 and 1 041 s and the step of an eighth in 128 s.
8. **The weekly job**: twelve shards of 180 minutes, each running its tests two at a time. The cost table holds 136
   tests and 107 093 s, and the slowest shard read 94.1 minutes, 52 per cent (ADR-0183). This round adds three
   derivations and six arms, about 11 500 s.
9. **The dispatch's scope** ([ADR-0075](../../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)): a file under `src/`
   changes, so the reproduction is `scope=both`. With the sweep's thirteen jobs that is twenty-five against the
   account's twenty at once.
10. **No sweep is unrecorded**: the last, run 38417864748's, ADR-0183 recorded. This round's own sweep is read when it
    ends, by the first ADR that merges after (ADR-0158).
11. **Archiving** (ADR-0185): `npx --no-install @descent-vtt/spec-brief archive 074 --pr <n> --summary "<one
    paragraph>"`, after every deliverable is ticked or dispositioned. It writes the banner and the integrity hash;
    `npm run spec` fails an archived brief changed since.

## Deliverables

- [x] **The read, built in the engine, in a new ADR at the next free number (`ls docs/adr`).**
  - ADR-0188's read: the gain held for 128 windows from the first, windows 113 to 128 summed, their mean the target's
    count, the period set, the rule from the 129th.
  - Its span and progress in the modulator section's reserved bytes; the image at format 23, its table in §5.2.
  - Refused: a read beside a period already set, and a count `rate_target_of` would refuse.
  - Unset, every run the run it was, bit for bit; a format-22 image read as unset.
  - Its tests:
    - by hand at the read's edges;
    - over the lattice against an oracle of the read;
    - an image written during the read reading back to the same run;
    - the gain held through the read whatever the step.
  - The whole-image pins moved by the format's mask alone.
- [x] **H-40's protocol, in a new ADR at the next free number, before any lead-in is run with the read set.**
  - The networks, the span and the step.
  - What holds a derivation under the read, where none has a pin.
  - H-40's clauses and constants, restated from ADR-0188 and pinned in the tests.
  - The three predicted readings as rules over the tables:
    - each read target within five per cent of its derived one, seed 22's below it;
    - each arm's mean count a window within one per cent of its read target;
    - through each arm the gain after every window within 1.6625 and 1.8375.
  - The readings, no clause, as ADR-0188 lists them.
  - How a check added to an arm was tried, by dispatch, before the runs.
- [x] **The calibration**, before any such run, by a dispatch at `scope=exhaustive` that excludes this round's own
  tests. Every pinned number of the tree holds at format 23 with the read unset, H-39's derivations and six arms among
  them.
- [x] **The derivations under the read on the three networks**, each a weekly `exhaustive` test, run once by a
  dispatch at `scope=arms`. Pinned from that run:
  - the read's sixteen windows and the target;
  - the lead-in by window;
  - what each rule returned, or that it returned nothing;
  - the frozen block's readings.
- [x] **The arms**: on each network whose clause 0 holds, H-25's two, each a weekly `exhaustive` test, run once by a
  dispatch at `scope=arms`, their tables pinned per block, the controller's reading among them.
- [x] **The ADR's reading.**
  - H-40's verdict per network, arm and clause.
  - The prediction and each of the three predicted readings against what was read.
  - Each network beside its own arms under H-39.
  - Over the three networks: the mappings learned of twenty-four, and the reversals' blocks beside H-39's.
  - **The step of the stopping rule reached, and the next decision it names, not taken.**
- [x] **The gate.** At most one runtime test: H-40's clauses and its predicted readings' rules at their edges, over
  tables written by hand.
- [x] **The evidence**, as "Evidencing a round" says. The ADR names, each with its run id and commit:
  - the calibration's dispatch;
  - each dispatch of the round's runs;
  - the reproduction, a weekly dispatched on this round's branch at `scope=both` once the tables are pinned. Its twelve
    whole-domain shards are green and reproduce every pinned number, and the cost table is regenerated from their
    artifacts;
  - each shard's job time against its bound;
  - that the sweep had not ended, if it had not.
- [x] **The documents, in the same pull request.**
  - Whitepaper: §5.2's modulator table at format 23, §11.1's H-40 with its verdict and step, and §9.
  - The ADR index and `CHANGELOG.md`.
  - `CLAUDE.md` where the standing changes: the image's format, and the result.
  - `README.md` and `docs/zh-TW`'s reader's guide as the result requires.
  - The whitepaper's version in both declarations, with its date.
- [x] **The brief archived** with `spec-brief archive`, every deliverable dispositioned. The round's ADRs are set to
  `accepted` by the commit that asks for the merge, and the pull request is marked ready for review with it.

## Not empowered

- **The round's scope**, which [ADR-0187](../../docs/adr/0187-the-upgrade-suite-planned.md) puts here and not among the
  standing directives:
  - the rate's rule is ADR-0180's, run as built, at a sixteenth;
  - the inhibitory rule's target stays the derivation's;
  - no change to the estimate's rule, the inhibitory rule, the critic, the address, the signed gate or the schedule;
  - the whole punishment of ADR-0155 and the exploration of ADR-0163 stay unset, and the feedback is the true one.
- **No change under `src/` but the read** and what carries it: the executor, the image and, if needed, a method on
  the pool that changes none of its bytes. No rule's form, no record of a state crate's layout and no default changes;
  `Config`'s default leaves the read unset.
- The span is 128 windows with the last sixteen read, the step is a sixteenth, and the networks are the three. None is
  fitted to the result, and none other is run.
- No rule that derives a constant is changed so that a network's read or step holds.
- H-40's clauses, its constants and its predicted readings do not move after such a run. There is no second attempt,
  and a run is not repeated for a check of the harness.
- No whole-domain test runs on the developer machine.
- No pinned number of an earlier round moves, but the whole-image pins by the format's mask.
- No change to `.github/workflows/ci.yml`, `scripts/arm-run.sh`, `scripts/exhaustive-shard.sh` or
  `scripts/exhaustive-costs.mjs`. The cost table changes only by regeneration from the reproduction.
- No float anywhere, no dependency, no `unsafe`, and no rebase of the branch once a dispatch has run on it.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- where the read's state lives and how its bytes are encoded: which reserved bytes, the width of the sum, how the
  windows read so far are counted, and whether the executor or a method on the pool closes a held window;
- the error a refused read raises, and where it is raised;
- how the harness sets the read, and how an arm's ground carries a network, a read and a settling, every call of the
  tree the call it was;
- how a derivation under the read on a network with no pin is held: what is asserted by rule, and what is pinned from
  the one run;
- how the three derivations and six arms are dealt into tests and into dispatches, and whether the derivations are
  dispatched before the arms;
- how each reading is computed and pinned;
- how a check added to an arm is tried, by dispatch, before the runs.

It may not:
- change the span, the step, a network, the task or the schedule;
- let the read set the inhibitory rule's target;
- move a clause, a constant or a predicted reading after such a run;
- run a whole-domain test on the developer machine;
- reach the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

If the calibration fails, the round stops there as H-40's stopping rule says: the finding is written, and no such run
is made.

## Verification

Before each push, on the developer machine:

```bash
cargo check --workspace --all-targets --locked
cargo test --workspace --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked
npm ci
npm run spec
```

The pull request's gate runs those and the rest: the release tests, the benchmarks once, the MSRV, AArch64 and the
mutation gate on the changed lines. Every check passes on the last commit.

By dispatch, never on the developer machine:

```bash
gh workflow run ci.yml --ref <this round's branch> -f scope=exhaustive -f exclude='<this round's own tests>'
gh workflow run ci.yml --ref <this round's branch> -f scope=arms -f tests='<the derivations, then the arms>'
gh workflow run ci.yml --ref <this round's branch> -f scope=both
node scripts/exhaustive-costs.mjs from <the reproduction's downloaded artifacts> --run <its id> > scripts/exhaustive-costs.tsv
```

The calibration's shards are green with the round's tests excluded. Each run's job exits 0 with one test passed and
its log kept. The reproduction's twelve whole-domain shards are green and reproduce every pinned number. In the
history, the span, H-40's constants and its predicted readings precede every dispatch that runs a lead-in with the
read set, and every commit a dispatch ran on is an ancestor of the pull request's head.

## Report

The closing message states:
- the read as built: where its state lives, the format, and that every run with it unset is the run it was;
- the protocol, and when it was committed;
- per network, the read's sixteen windows and target beside the derived one, the lead-in's settling, what each rule
  returned and the frozen block's measures;
- H-40's verdict per network, arm and clause, and the prediction and each predicted reading against what was read;
- each network beside its own arms under H-39;
- the step of the stopping rule reached and the next decision it names, not taken;
- each dispatch with its run id and commit: the calibration, the runs and the reproduction;
- that no whole-domain test ran on the developer machine, that each run ran once, and that no pinned number moved
  but by the format's mask;
- each shard's job time against its bound and the regenerated cost table;
- that the pull request is ready for review;
- what was not done and why.
