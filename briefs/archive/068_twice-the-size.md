---
status: archived
date: 2026-10-12
integrity: sha256-f118c0fa309c53c76d45b213dac6bc919654566df44610fa2a41f7cb32932f8e
---

> **Executed 2026-10-11 and 2026-10-12 (UTC) in pull request #200.** Writes ADR-0169 (twice the size, measured);
> opens no finding. No file under `src/` changed, and no pinned number moved but the one assertion ADR-0168 names,
> a stimulus of 65 units in H-33's gate. The derivation was given its size: `derive_at(units, seed, name)`, which
> `derive` calls with 1 024. The critic's rule was written as ADR-0168 gives it, the shift the lesser of the sum less
> ADR-0132's scale and 9 and the scale the rest. The size, the seeds, the range each rule scans and H-34's constants
> were committed and pushed before any rule ran at 2 048 units; the tree's 102 other whole-domain tests then passed,
> the rules at 1 024 units returning the constants the tree holds on seeds 22, 23 and 24 among them.
>
> **H-34 is yes, on both networks, in both arms and in all five clauses, and its stopping rule reached step 3: the
> learning configuration is its rules' at twice the size as at the first.** On seeds 22 and 23 at 2 048 units every
> rule gave a value: the rotation 0, the gain 1.75, the cancel's 6 messages and the critic's window of 100 ticks as
> on all three networks at 1 024; the critic's shift and scale 9 and 3, the extension's for a stimulus of 102 units;
> the settling at the 100th and the 105th window; the cancel's span 10 and 21 ticks; the target period 59 599 and
> 59 119 ticks. The frozen block fired each stimulus unit once and saw the stimulus in 64 trials of 64 on each.
> H-25's two arms learned every mapping, 121, 122, 123 and 125, and 124, 125, 127 and 123 of each mapping's last 128
> on seed 22, and 126, 127, 126 and 125, and 121, 120, 127 and 126 on seed 23, with no coupling past 1.30 of its
> image's, the highest 1.169 and 1.151, every reversal within 9 to 15 blocks of its flip where the bound is 23, and
> the network outside the couplings within two parts in ten thousand of the image's. The critic's constants gave
> the step ADR-0132's gave, 253 to 255 parts in ten thousand of the error a presentation. The rotation's rule and
> the gain's returned what they returned at 1 024 units, so they are still not read returning another value. The
> next decision, an ADR choosing among a larger size still, which needs the weekly job's bound decided first, the
> operating regime, a reward delayed past a trial and a less reliable reward, is named and not taken.
>
> Every deliverable below is done and its box ticked. Seven things differ from the brief's letter or add to it, each
> by its empowerment clause or by its directive that the rules are the code, and each recorded in ADR-0169:
> - four functions beneath `derive` wrote the size too, which the Context does not list: `target_lead_in` and
>   `fed_run_on` gained forms given it, `probe_task_on` builds its probe at the prior's own size, and the assertion
>   that a rewarded run is at 1 024 units moved from `earned_run_held_on` to `earned_run_held`, which passes the
>   constants pinned at that size;
> - **the injector ring is sized by the units**, 8 192 entries at 2 048 where every run had 4 096, decided before any
>   run from the rules' ranges: the widest cancel the pick can try is 6 544 messages before one tick there. No run
>   needed it; the widest tried was 628;
> - a network is a `Site`, and H-33's six tests at 1 024 units run through the same `site_derivation` and
>   `site_arm` as H-34's at 2 048;
> - the critic's step is read beside every arm's run and held to the engine's own value at every trial, and it is
>   read and pinned on H-33's four arms at 1 024 units too (`SEEDED_CRITIC`), a table added and none moved;
> - the calibration ran H-33's six tests each in a process of its own beside the other ninety-six under cargo, so
>   that their dumps were kept;
> - with the pins, `config` gained an assertion that its ring holds the widest injection at its size and the gate
>   holds `TWICE_PREDICTED` at run time, for the MSRV toolchain's reading of a use inside a compile-time assertion;
> - H-34's tables are named `TWICE_*`: `SIZED_*` is H-31's.
>
> Relative links gained one `../` so that they resolve from `archive/`; no word, claim or figure changed.
>
> *The body below describes the tree before execution and is not maintained.*

# Brief 068: Twice the size — ADR-0168's question: ADR-0044's generator at 2 048 units from seeds 22 and 23, every constant of the learning configuration derived by its rule at that size, the critic's rule extended as ADR-0168 writes it, then H-25's two arms on each network; nothing under `src/` changed; H-34 run once

## Mission

**This brief builds nothing in the engine and runs one hypothesis once.** Every learning result of H-25's
configuration was read at 1 024 units, on three networks.
[ADR-0168](../../docs/adr/0168-twice-the-size.md) asks whether the configuration is its rules' at another size: on
ADR-0044's generator at 2 048 units from seeds 22 and 23, each constant is derived by its rule with the size given
where 1 024 was written, and H-25's two arms are run.

ADR-0168 wrote **H-34** before any run: the rules derive a configuration on each network, and H-25's four clauses
hold there. It predicts no verdict. It also decides, before any run, how the critic's rule reads for a stimulus of
more than 64 units, where the rule as written returns nothing.

When the round is done, the tree holds:
- the derivation taking its size, with every pinned number at 1 024 units as it was, but for the one assertion
  ADR-0168 names;
- on each of the two networks at 2 048 units, the derivations as tests and H-25's two arms as weekly `exhaustive`
  tests;
- H-34's verdict and **the step of its stopping rule reached**, with the next decision it names and does not take.

**Do not run this round beside brief 065's**: both dispatch the weekly workflow.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). Nothing is adopted: no mechanism, no parameter, no
  dependency, no version bump of a tool. **In particular**:
  - the whole punishment of ADR-0155 and the exploration of ADR-0163 stay unset;
  - two readouts and two stimuli, H-25's task;
  - no change to a rule that derives a constant, **but the critic's, and that one exactly as ADR-0168 writes it**. Every
    other rule is run as it is written, at a size it was not written at.
- **No file under `src/` and no file of a state crate changes.**
- **One change from H-33**: the size, 2 048 units. The generator's other parameters, the drive's rule and seed, the
  task's seed and the schedule are H-25's.
- **The networks are seeds 22 and 23 at 2 048 units.** Neither is looked at before the protocol is committed, none
  replaces one that reads badly, and no third network and no other size is run.
- **At 1 024 units the rules return the constants the tree holds**, on seeds 22, 23 and 24, and every pinned number
  holds. One assertion moves, by ADR-0168: a stimulus of 65 units, which the gate pins as no critic, reads a shift of
  9 and a scale of 3. The determinism pin does not move.
- **A rule that gives no value on a network, or a frozen block that fails a measure there, is a reading of H-34**
  (its clause 0). It stops that network, not the round, and it is not repaired.
- **H-34's clauses and constants are written before any rule is run at the new size** and do not move after it.
  There is no second attempt.
- **A prediction is not owed.** ADR-0168 predicts no verdict; this round does not add one.
- **The engine and the harness are read before a description of them is trusted**, this brief's and ADR-0168's
  included. The Context below is the map; the rules are the code.
- **No capacity of a run's configuration is reached silently.** At the new size the round shows that no injection was
  refused, no worker dropped a spike and the train held every window read from it.
- No `f32`/`f64`, oracles included; every operation on a state field saturates or wraps by name
  ([ADR-0029](../../docs/adr/0029-structural-enforcement.md)). Every loop ends by construction
  ([ADR-0062](../../docs/adr/0062-the-first-complete-sweeps-list.md)).
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive`. The runtime's gate grows by at most one test
  ([ADR-0061](../../docs/adr/0061-the-learning-runs-leave-the-gate.md)). The mutation gate on the changed lines must pass
  ([ADR-0030](../../docs/adr/0030-verification-governance.md)); with no line under `src/` changed it has nothing to
  mutate, and says so.
- **The round is evidenced as `briefs/README.md`'s "Evidencing a round" says**
  ([ADR-0150](../../docs/adr/0150-a-round-waits-for-what-it-checks.md),
  [ADR-0158](../../docs/adr/0158-the-sweep-off-the-next-decisions-path.md)): the arms run once, the dispatch's scope
  follows the diff, the merge waits for the whole-domain shards, the ADR cites the pull request's commits, the commit
  that asks for the merge sets it to `accepted`, and a sweep that has ended and that no ADR has recorded is recorded by
  this round's.
- **The weekly job's cost is reported, and its bound is not this round's.** The regenerated deal is planned against
  60 per cent of a shard's bound as before. An arm at this size is expected at about half of the bound by itself, so
  where a shard reads past 60 per cent because of one arm, the round reports the arm's seconds and the shard's and
  goes on; ADR-0168 names the bound as the workflow's next decision.
- No pinned number of an earlier round moves, but the one named above.
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-10-12 against `main` at `08008c5`, after ADR-0167 merged. Line numbers move; the symbols and the
quoted sentences are what to re-derive.

1. **The derivation** (`runtime/cortex-runtime/tests/inhibition.rs`, `derive(seed, name)`): it begins
   `let p = prior_seeded(1024, seed);` and writes `1024` at every step after it: `config(1024, …)`,
   `geometry(1024, rotation)`, `compose_on_over(…, 1024, BLOCK)`, `candidate_from(&p, 1024, …)`,
   `frozen_from(&zero, 1024)`. It returns a `Derivation` whose `rules` is a `Rules { rotation, gain, settled, span,
   least, cancel, frozen, period, window, critic }`, and stops at the first rule that returns nothing.
2. **What the rules returned at 1 024 units**, pinned as `RULES_1024` and `RULES_SEEDED`:

   | Rule | Seed 22 | Seed 23 | Seed 24 |
   | :--- | ---: | ---: | ---: |
   | the rotation, `first_balanced` over `ROTATIONS` | 0 | 0 | 0 |
   | the gain, `first_calibrated` over `GAINS` | 1.75 | 1.75 | 1.75 |
   | the settled network, `settled_within` over `LEAD_IN_EXTENDED` windows | the 96th | the 106th | the 99th |
   | the cancel's span, from the census | 9 | 8 | 12 |
   | the cancel's messages, the first of the three candidates that fires once | 6 | 6 | 6 |
   | the frozen block's two measures, `frozen_measures` | both | both | both |
   | the target period, `target_period` | 58 201 | 59 510 | 59 179 |
   | the critic's window, `shortest_delay` | 100 | 100 | 100 |
   | the critic, `critic_for` | 9 and 2 | 9 and 2 | 9 and 2 |

3. **The critic's rule** (`critic_for(units)`): the least sum with `units × 32 ≤ 2^sum`; the scale held at
   `SPLIT_SCALE`, 2; nothing where the shift's dead band is not under `DEAD_BAND_PER_CENT` of the reward. The gate
   holds `critic_for(51)`, `(50, 33, 64)`, `(32)`, `(4)` and **`(critic_for(65), critic_for(0)) == (None, None)`**.
   ADR-0168 decides its extension: the shift is the lesser of the sum less 2 and 9, the scale is the sum less the
   shift, and nothing is returned where no sum fits, where the sum is below 2 or where `ValueCritic::is_valid`
   refuses. That reads 9 and 3 for 65 to 128 units and 9 and 4 for 129 to 256, and what it read everywhere it read a
   value.
4. **The geometry** (`harness.rs`, `geometry(units, rotation)`): `(units − rotation) / PERIOD` whole periods of twenty
   places. At 2 048 units that is 102 periods from a rotation of at most eight and 101 from a later one, so a
   stimulus is 102 or 101 units and a readout nine times that.
5. **What is written for two sizes only**: `rotation(units)` and `gain(units)` panic for a size other than 256 and
   1 024; the pinned tables are named `_256` and `_1024`.
6. **The engine's type and capacities** (`harness.rs`): `Engine = Executor<2048>`, where `CAP` is the tokens one wheel
   slot holds (`src/executor.rs`, "2 048 in production, ADR-0013"), not the units. `config(units, workers, baseline)`
   sizes `train_capacity` by the units and holds `injector_capacity` at `1 << 12` and `nodes_per_worker` at
   `1 << 16`. A full injector ring is `InjectError::Full`; a worker out of nodes aborts; a train that lets a spike go
   counts it in `train_overwritten`; a trace that drops one counts it in `WorkerReport::dropped`.
7. **The drive** (`drive(units)`): `units / 128` messages a tick, seed 3. The task's `SEED` is 27.
8. **What has run at another size**: `tests/reference.rs` runs the prior under the drive at 256, 1 024 and 4 096
   units. The task has run at 256 and 1 024, and H-25's configuration at 1 024 only.
9. **H-33** ([ADR-0167](../../docs/adr/0167-another-network-of-the-same-size-measured.md)): `SEEDS`, `SEEDED_ARMS`,
   `fed_run_on`, `drawn_of`, `SEEDED_1024`. Its twelve reversals took 12 to 21 blocks where the bound is 23, its
   highest couplings stood at 1.182 to 1.195 of their image's, and each mapping read 115 to 126 of its last 128.
10. **The weekly job**: twelve shards of 120 minutes, each running its tests two at a time. The cost table holds 102
    tests and 66 471 s. H-33's four arms read 1 815 to 1 842 s and its two derivations 238 and 245 s. A new test is
    costed at 900 s until a dispatch has measured it.
11. **The expectation of the cost**: twice. A developer machine ran six windows of the settling at each size, six
    pairs one after another: five read 1.8 to 2.0 times the seconds at 2 048 units and the first 3.3. That is not a
    measurement; the dispatch's seconds are.
12. **The engine is deterministic**, so the four arms read the same tables whether they are run one after another or
    side by side; only the seconds differ.
13. **The dispatch's scope** ([ADR-0075](../../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)): `scope=exhaustive`
    for a round that changes no file under `src/`.

## Deliverables

- [x] **The derivation given its size, in the tests.**
  - `derive` and what it calls take the size where they wrote 1 024.
  - At 1 024 units, on seeds 22, 23 and 24, each rule returns what the tree pins and every pinned table holds.
- [x] **The critic's rule as ADR-0168 writes it.**
  - The rule's values at its edges in the gate: 4, 32, 33, 51 and 64 units as they were; 65, 102 and 128 units as 9
    and 3; 129 units as 9 and 4; no unit as nothing; and the least stimulus at which the loader's bound returns
    nothing.
  - The one pinned assertion that moves is the one ADR-0168 names.
- [x] **H-34's protocol, in a new ADR at the next free number (`ls docs/adr`), before any rule is run at the new
  size.**
  - The size, the seeds, 22 and 23, and the order of the derivations, ADR-0165's.
  - For each rule, the range it scans at the new size and what it returns when nothing in the range passes.
  - What holds a derivation at the new size, where 1 024 had pinned tables: the rule itself, and the first run's
    table pinned from the run that the dispatch then reproduces.
  - Which capacity of `config` each run at the new size could reach, and how the round shows it was not reached.
  - H-34's clauses and constants, restated from ADR-0168 and pinned in the tests: clause 0, and H-25's four by H-25's
    rules.
  - The readings, no clause, as ADR-0168 lists them.
- [x] **The calibration**, before any rule is run at the new size: every pinned number of the tree holds, and at
  1 024 units the rules return the constants the tree holds on all three seeds.
- [x] **The derivations on each network at 2 048 units**, each a test: what each rule returned, or that it returned
  nothing, and the frozen block's two measures.
- [x] **The runs**: on each network whose clause 0 holds, H-25's two arms, each a weekly `exhaustive` test, run once,
  their tables pinned per block.
- [x] **The ADR's reading.**
  - H-34's verdict per network, arm and clause.
  - Each derived constant on each network beside the three networks' at 1 024.
  - The readings ADR-0168 lists, among them the critic's: the value beside $2p - 1$, what a presentation moves it by,
    and the units the critic counts a trial beside the volley's.
  - An arm's seconds in the dispatch beside twice an arm's at 1 024.
  - **The step of the stopping rule reached, and the next decision it names, not taken.**
- [x] **The gate.** At most one runtime test: the critic's rule at its edges, H-34's clauses at their edges and the
  readings' rules over tables written by hand.
- [x] **The evidence**, as "Evidencing a round" says. A weekly dispatched on this round's branch at
  `scope=exhaustive`. Its twelve whole-domain shards are green and reproduce every pinned number. The cost table is
  regenerated from their artifacts. The ADR names the dispatch, each shard's wall time against its bound, and any
  shard past 60 per cent with the arm that put it there.
- [x] **The documents, in the same pull request.**
  - Whitepaper §11.1's H-34 with its verdict and step, and §9.
  - The ADR index and `CHANGELOG.md`.
  - `CLAUDE.md` only where the standing changes (ADR-0150); `README.md` and `docs/zh-TW`'s reader's guide as the
    result requires.
  - The whitepaper's version in both declarations, with its date
    ([ADR-0064](../../docs/adr/0064-the-documentation-gate-and-the-version.md)).
- [x] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned, and the round's ADR set to
  `accepted` by the commit that asks for the merge.

## Not empowered

- **No file under `src/` and no file of a state crate changes.** No rule, no record, no format.
- No rule that derives a constant is changed, widened or given a fallback so that it returns a value at the new size,
  **but the critic's, exactly as ADR-0168 writes it**. Another split of the critic's sum is not this round's.
- No constant of a network at 1 024 units is carried to 2 048 where a rule derives it.
- The networks are seeds 22 and 23 at 2 048 units. No other seed and no other size is run, before or after.
- The task is H-25's: two stimuli, two readouts, H-20's schedule, the task's seed 27.
- H-34's clauses and constants do not move after a rule is run at the new size. There is no second attempt.
- No pinned number of an earlier round moves, but the one ADR-0168 names.
- No change to `.github/workflows/ci.yml`, `scripts/exhaustive-shard.sh` or `scripts/exhaustive-costs.mjs`; the cost
  table changes only by regeneration from this round's dispatch. A shard's bound is not this round's.
- No float anywhere, no dependency, and no `unsafe`.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- how the derivation takes its size: a parameter of `derive`, or a form given the size that `derive` calls with 1 024;
- how the functions and tables written for 256 and 1 024 units are reached at another size without moving theirs;
- how the critic's rule is written so that it returns ADR-0168's values, and how its constants are named;
- the range each rule scans at the new size where the tree does not state one, written before the rule is run there;
- a capacity of `config` that a run at 2 048 units would reach: sized by the units, as the train is, with 1 024's as
  it was, and said in the ADR;
- how a derivation with no pinned table is held: what is asserted by rule, and what is pinned from the one run;
- how each network's derivations and arms are dealt into tests, whether a derivation is run once and its image kept
  for both arms, and whether the local runs go one after another or side by side;
- how each reading is computed and pinned.

It may not:
- change a file under `src/` or of a state crate;
- change a rule other than the critic's, a seed, the size, the task or the schedule;
- move a clause or a constant after a rule is run at the new size;
- reach the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

If the calibration fails, the round stops there as H-34's stopping rule says: the finding is written, and no rule is
run at the new size.

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
git diff main...HEAD > target/pr.diff && cargo mutants --workspace --in-diff target/pr.diff
gh workflow run ci.yml --ref <this round's branch> -f scope=exhaustive
node scripts/exhaustive-costs.mjs from <the dispatch's downloaded artifacts> --run <its id> > scripts/exhaustive-costs.tsv
```

Every command exits 0. The dispatched run's twelve whole-domain shards are green and reproduce every pinned number.
In the history, the size, the seeds, the critic's rule and H-34's constants precede any rule run at the new size.

## Report

The closing message states:
- how the derivation took its size, and that at 1 024 units nothing moved but the one assertion ADR-0168 names;
- the critic's rule as built, and its values at its edges;
- the protocol, and when it was committed;
- on each network at 2 048 units, what each rule returned beside the three networks' at 1 024, and the frozen block's
  measures;
- H-34's verdict per network, arm and clause, and the readings;
- the step of the stopping rule reached and the next decision it names, not taken;
- that no file under `src/` changed, and which capacity of the configuration, if any, was sized by the units;
- the scope ADR-0075 gave the diff, each shard's time against its bound, an arm's seconds beside twice an arm's at
  1 024, and the regenerated cost table;
- what was not done and why.
