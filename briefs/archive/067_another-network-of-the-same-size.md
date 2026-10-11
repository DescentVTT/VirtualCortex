---
status: archived
date: 2026-10-11
---

> **Executed 2026-10-11 (UTC) in pull request #198.** Writes ADR-0167 (another network of the same size, measured);
> opens no finding. No file under `src/` changed and no pinned number moved. The harness's constants for the
> instrument's prior became what the rules return for a prior: each function of a run's path that read one gained a
> form that is given it, and the function it was calls that form with seed 22's, so no call of the tree changed.
> `derive` runs the rules on ADR-0044's prior from a seed in ADR-0165's order and stops at the first that returns
> nothing. The seeds, the range each rule scans and H-33's constants were committed and pushed before any rule ran on
> a network of another seed; the tree's 96 other whole-domain tests then passed, among them the rules on seed 22
> returning every constant the tree holds, each table on the way the one its round pinned, and the image H-25's by
> its CRC.
>
> **H-33 is yes, on both networks, in both arms and in all five clauses, and its stopping rule reached step 3: the
> learning configuration is the rules', not one network's.** On seeds 23 and 24 every rule gave a value: the
> rotation 0, the gain 1.75 and the cancel's 6 messages as on seed 22; the settling at the 106th and the 99th window
> where seed 22's settled at the 96th; the cancel's span 8 and 12 ticks against 9; the target period 59 510 and
> 59 179 ticks against 58 201; the critic's window 100 ticks and its shift and scale 9 and 2. The frozen block fired
> each stimulus unit once and saw the stimulus in 64 trials of 64 on each. H-25's two arms learned every mapping,
> 126, 124, 124 and 123, and 122, 115, 118 and 126 of each mapping's last 128 on seed 23, and 117, 123, 120 and 115,
> and 124, 120, 121 and 123 on seed 24, with no coupling past 1.30 of its image's, the highest 1.183 and 1.195,
> every reversal within 12 to 21 blocks of its flip where the bound is 23, and the network outside the couplings
> within two parts in ten thousand of the image's. Five of the eight constants came back the same on all three
> networks, so those rules were tested in returning a value that worked and not in returning another; and no rule
> returned nothing. The next decision, an ADR choosing among another size, the operating regime, a reward delayed
> past a trial and a less reliable reward, is named and not taken.
>
> Every deliverable below is done and its box ticked. Six things differ from the brief's letter, each by its
> empowerment clause or by its directive that the rules are the code, and each recorded in ADR-0167:
> - the cancel's messages are ADR-0076's pick among three candidates, as the tree holds the rule, where the brief's
>   table names `least_cancel` at the extreme standing, the candidate seed 22 picked; the two agree on all three
>   networks;
> - the cancel's span is read from the census of the picked gain's calibration block on the prior's network, as
>   `VOLLEY_TICKS_1024` was, and the candidates' frozen blocks are run on that network, before the settled one;
> - a derivation runs the gain's candidates and the cancel's in order until one passes, and does not run one after
>   the one taken;
> - clause 0's sight is ADR-0165's words, 56 trials of 64, with the harness's `calibrated` and ADR-0072's sign read
>   beside it;
> - an arm on a new network is H-25's run with no shadow beside it, and without H-23's first block reproduced,
>   which reads seed 22's tables;
> - the round adds seven weekly tests, one of them the rules on seed 22, and the gate's one test also reads the
>   pinned tables of the new networks.
>
> Relative links gained one `../` so that they resolve from `archive/`; no word, claim or figure changed.
>
> *The body below describes the tree before execution and is not maintained.*

# Brief 067: Another network of the same size — ADR-0165's question: on two priors of the next two seeds, every constant of the learning configuration derived again by the rule that derived it, then H-25's two arms on each; nothing under `src/` changed; H-33 run once

## Mission

**This brief builds nothing in the engine and runs one hypothesis once.** Every learning result since H-13 was read
on one network, ADR-0044's prior at 1 024 units from seed 22.
[ADR-0165](../../docs/adr/0165-another-network-of-the-same-size.md) asks whether the learning configuration is the rules'
or that network's: on the same generator from seeds 23 and 24, each constant the configuration took from the network
is derived again by its rule, and H-25's two arms are run.

ADR-0165 wrote **H-33** before any run: the rules derive a configuration on each network, and H-25's four clauses
hold there. It predicts no verdict.

When the round is done, the tree holds:
- the harness's constants for seed 22 as what the rules return for a prior, with every pinned number of the tree as it
  was;
- on each of the two networks, the derivations as tests and H-25's two arms as weekly `exhaustive` tests;
- H-33's verdict and **the step of its stopping rule reached**, with the next decision it names and does not take.

**Do not run this round beside brief 065's**: both dispatch the weekly workflow.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). Nothing is adopted: no mechanism, no parameter, no
  dependency, no version bump of a tool. **In particular**:
  - the whole punishment of ADR-0155 and the exploration of ADR-0163 stay unset;
  - two readouts and two stimuli, H-25's task;
  - no change to a rule that derives a constant. A rule is run as it is written, on a network it was not written on.
- **No file under `src/` and no file of a state crate changes.**
- **One change from H-25**: the prior's seed. The generator's other parameters, the drive's seed and the task's seed
  are H-25's.
- **The seeds are 23 and 24.** No network is looked at before its seed is committed, none replaces one that reads
  badly, and no third seed is run.
- **On seed 22 the rules return the constants the tree holds**, and every pinned number holds. The determinism pin
  does not move.
- **A rule that gives no value on a network, or a frozen block that fails a measure there, is a reading of H-33**
  (its clause 0). It stops that network, not the round, and it is not repaired.
- **H-33's clauses and constants are written before any rule is run on a new network** and do not move after it.
  There is no second attempt.
- **A prediction is not owed.** ADR-0165 predicts no verdict; this round does not add one.
- **The engine and the harness are read before a description of them is trusted**, this brief's and ADR-0165's
  included. The table of rules below is the map; the rules are the code.
- No `f32`/`f64`, oracles included; every operation on a state field saturates or wraps by name
  ([ADR-0029](../../docs/adr/0029-structural-enforcement.md)). Every loop ends by construction
  ([ADR-0062](../../docs/adr/0062-the-first-complete-sweeps-list.md)): a search for the first rotation or the first
  window is a scan over a stated range.
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
- **No shard of the weekly job passes 60 per cent of its bound** under the regenerated deal.
- No pinned number of an earlier round moves.
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-10-11 against `main` after ADR-0163 to ADR-0166 merged. Line numbers move; the symbols and the
quoted sentences are what to re-derive.

1. **The prior** (`runtime/cortex-runtime/tests/instrument/harness.rs`, `prior(units)`): ADR-0044's generator, a fifth
   inhibitory, 32 synapses a unit, a window of eight, a quarter rewired, local delays of 100 to 300 ticks, and
   `seed: 22`. `drive(units)` has `seed: 3`, and the task's `SEED` is 27.
2. **The rules a constant came from**, each with its value on seed 22:

   | Constant | The rule, and where it lives | On seed 22 |
   | :--- | :--- | :--- |
   | the rotation | the first from which the four couplings by the prior's census are `balanced`; `the_geometry_holds_against_the_census_at_both_sizes` holds it and that every smaller one fails | `ROTATION_1024` 0 |
   | the gain | the first of `GAINS` at which the frozen block reads `SEEN_MIN`, 56 of 64 | `GAIN_1024` 1.75 |
   | the settled network | `settled_within`, ADR-0055's criterion over the lead-in's windows; `background_candidate`, `settled_engine`, `settled_image` | held at the 96th window |
   | the cancel | `CANCEL_OFFSET`, the refractory window and one; `CANCEL_TICKS`, the ticks `VOLLEY_TICKS_1024` spreads over; `least_cancel` at the extreme standing | 201, 9 and 6 |
   | the frozen block's measures | `fires_once`, and the sight at `SEEN_MIN` | every unit once; 62 of 64 |
   | the target period | `target_period` over the settled network's spikes (`tests/inhibition.rs`) | `TARGET_PERIOD_1024` 58 201 |
   | the critic's window | `shortest_delay` of the image; the loader refuses another | `WINDOW_TICKS_1024` 100 |
   | the critic | ADR-0132's arithmetic for the stimulus's units | `VALUED_CRITIC`, shift 9 and scale 2 |

3. **What is the configuration's and no network's**: the signed gate, the excitatory baseline at zero, the inhibitory
   baseline at a half, `Delivery::Drawn`, H-20's schedule (`SCHEDULE`), the two starting assignments.
4. **How H-25's image is built** (`tests/inhibition.rs`): link by link from the settled image, each link held by a
   CRC pinned for seed 22, the last `WINDOWED_IMAGE_CRC_1024`.
5. **H-25** ([ADR-0140](../../docs/adr/0140-the-address-drawn-measured.md)): `drawn_arm`, with the composer and the
   network's oracle held at every trial; its verdict `DRAWN_1024`; its clause 3 by `reversals_within` at
   `REVERSAL_BLOCKS_MAX` 23. It read the first mappings passing 40 of 64 in the 6th and 4th block, the reversals in
   19, 21 and 16 and in 20, 15 and 14 blocks, and the highest coupling at 1.191.
6. **Seed 22's pins are tables**, not rules: ADR-0077's lead-in and quiet run, the frozen blocks, each image's CRC.
   On a new network there is no table to hold a derivation to; what holds it is the rule.
7. **The weekly job**: twelve shards. H-25's two arms read 1 079 and 1 782 s in the last dispatch. The bound of a
   shard is 120 minutes.
8. **The dispatch's scope** ([ADR-0075](../../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)): `scope=exhaustive`
   for a round that changes no file under `src/`.

## Deliverables

- [x] **The harness's constants as what the rules return for a prior, in the tests.**
  - Each constant of the table is returned by its rule from a prior, or the rule is written beside the constant and
    held equal to it on seed 22.
  - On seed 22 every pinned number of the tree holds, and each rule returns the constant the tree holds.
- [x] **H-33's protocol, in a new ADR at the next free number (`ls docs/adr`), before any rule is run on a new
  network.**
  - The seeds, 23 and 24, and the order of the derivations: the rotation, the gain, the settled network, the cancel
    and the frozen block's two measures, the target period, the critic's window and the critic's arithmetic.
  - For each rule, the range it scans and what it returns when nothing in the range passes.
  - What holds a derivation on a new network where seed 22 had a pinned table: the rule itself, and the first run's
    table pinned from the run that the dispatch then reproduces.
  - H-33's clauses and constants, restated from ADR-0165 and pinned in the tests: clause 0, and H-25's four by H-25's
    rules.
  - The readings, no clause:
    - each derived constant beside seed 22's;
    - the frozen block: the readouts' counts before and after the volley, the selections by readout and the ties;
    - the block each mapping passes 40 of 64 in, beside H-25's;
    - the couplings' courses and their highest; the value beside $2p - 1$ with its troughs;
    - the inhibitory sum's course.
- [x] **The calibration**, before any rule is run on a new network: every pinned number of the tree holds, and on seed
  22 the rules return the constants the tree holds.
- [x] **The derivations on each network**, each a test: what each rule returned, or that it returned nothing, and the
  frozen block's two measures.
- [x] **The runs**: on each network whose clause 0 holds, H-25's two arms, each a weekly `exhaustive` test, run once,
  their tables pinned per block.
- [x] **The ADR's reading.**
  - H-33's verdict per network, arm and clause.
  - Each derived constant on each network beside seed 22's, and what differs between the networks.
  - **The step of the stopping rule reached, and the next decision it names, not taken.**
- [x] **The gate.** At most one runtime test: the rules on seed 22 against the tree's constants where that is cheap
  enough for the gate, H-33's clauses at their edges and the readings' rules over tables written by hand.
- [x] **The evidence**, as "Evidencing a round" says. A weekly dispatched on this round's branch at
  `scope=exhaustive`. Its twelve whole-domain shards are green and reproduce every pinned number. The cost table is
  regenerated from their artifacts. The ADR names the dispatch.
- [x] **The documents, in the same pull request.**
  - Whitepaper §11.1's H-33 with its verdict and step, and §9.
  - The ADR index and `CHANGELOG.md`.
  - `CLAUDE.md` only where the standing changes (ADR-0150); `README.md` and `docs/zh-TW`'s reader's guide as the
    result requires.
  - The whitepaper's version in both declarations, with its date
    ([ADR-0064](../../docs/adr/0064-the-documentation-gate-and-the-version.md)).
- [x] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned, and the round's ADR set to
  `accepted` by the commit that asks for the merge.

## Not empowered

- **No file under `src/` and no file of a state crate changes.** No rule, no record, no format.
- No rule that derives a constant is changed, widened or given a fallback so that it returns a value on a new network.
- No constant of seed 22 is carried to another network where a rule derives it.
- The seeds are 23 and 24. No other is run, before or after.
- The task is H-25's: two stimuli, two readouts, H-20's schedule, the task's seed 27.
- H-33's clauses and constants do not move after a rule is run on a new network. There is no second attempt.
- No pinned number of an earlier round moves.
- No change to `.github/workflows/ci.yml`, `scripts/exhaustive-shard.sh` or `scripts/exhaustive-costs.mjs`; the cost
  table changes only by regeneration from this round's dispatch.
- No float anywhere, no dependency, and no `unsafe`.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- how a constant becomes what a rule returns for a prior: a function of the prior, a table keyed by the seed with the
  rule beside it, or readers written for the new networks beside the ones seed 22 has;
- the range each rule scans where the tree does not state one, written before the rule is run on a new network;
- how the critic's arithmetic is restated where a rotation leaves a stimulus of other than 51 units;
- how a derivation with no pinned table is held: what is asserted by rule, and what is pinned from the one run;
- how each network's derivations and arms are dealt into tests, and whether a derivation is run once and its image
  kept for both arms;
- how each reading is computed and pinned.

It may not:
- change a file under `src/` or of a state crate;
- change a rule, a seed, the task or the schedule;
- move a clause or a constant after a rule is run on a new network;
- reach the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

If the calibration fails, the round stops there as H-33's stopping rule says: the finding is written, and no rule is
run on a new network.

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
In the history, the seeds, the order of the derivations and H-33's constants precede any rule run on a new network.

## Report

The closing message states:
- how the harness's constants became what the rules return, and that on seed 22 nothing moved;
- the protocol, and when it was committed;
- on each network, what each rule returned beside seed 22's, and the frozen block's measures;
- H-33's verdict per network, arm and clause, and the readings;
- the step of the stopping rule reached and the next decision it names, not taken;
- that no pinned number moved and no file under `src/` changed;
- the scope ADR-0075 gave the diff, the shards' times and the regenerated cost table;
- what was not done and why.
