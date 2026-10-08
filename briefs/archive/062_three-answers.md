---
status: archived
date: 2026-10-09
---

> **Executed 2026-10-08 to 2026-10-09 in pull request #188.** Writes ADR-0152 (three answers, built) and ADR-0153
> (three answers, measured); opens and closes F-65. The task was built generic over its channels: `Readout`, `Task`
> and `Outcome` take the number of readouts as a parameter of the type, two unless the type names another and at most
> 64; each channel's indirect drive is the largest of the other channels' counts, so the largest count alone is
> selected; the mapping is an answer for each stimulus, which `Task::flip` moves to the next readout; an answer that
> names no readout is refused. No rule of the engine, no record and nothing of the image changed, and under two
> readouts a trial is the trial it was and a flip the flip it was, which a test holds against the task as ADR-0148
> wrote it, written out. The 92 400 deals' order was committed and pushed before any coupling was read; the deal
> taken is the 104th, readouts of places 3 to 6, of 7, 12, 14 and 18 and of 8, 15, 16 and 19. The protocol and H-29's
> constants were committed and pushed before any rewarded trial of an arm; before the arms every one of the weekly
> job's 87 earlier whole-domain tests passed, in each arm H-25's first block reproduced under two readouts, and a
> frozen block with three readouts saw the stimulus in 62 trials of 64.
>
> **H-29 is no, on clause 1, and its stopping rule reached step 5.** Seven of the eight mappings were learned by each
> stimulus: 118, 87, 82 and 109 of each mapping's last 128 from the assignment, and 119, 99 and 114 from the mirrored
> assignment. The mirrored arm's second was not: 85 of its last 128 were correct, but stimulus A selected its answer in
> 30 of its 64 presentations. No coupling passed 1.30 of its image's, the highest 1.242 and 1.261; the excitatory sum
> outside the six couplings stayed within a part in ten thousand of the image's; and each stimulus's mean value stood
> within 0.21 of the reward of 2p − 1. The first mapping was learned as between two. A reversal took 21 to 31 of a
> mapping's 32 blocks where H-25's took 14 to 21: after a flip the old answer took 62 to 90 per cent of the wrong
> selections, the value fell to −0.58 to −0.95 of the reward, and a punishment of the old answer delivered a third to
> two thirds of the reward. The next decision, an ADR choosing between the selection, an exploration among its
> candidates and the readout's resolution, is named and not taken.
>
> Every deliverable below is done and its box ticked. Relative links gained one `../` so that they resolve from
> `archive/`; no word, claim or figure changed.
>
> *The body below describes the tree before execution and is not maintained.*

# Brief 062: Three answers — ADR-0151's selection among three channels and its mapping built, under two readouts every run the run it was; the readouts dealt by its rule; then H-29 run once on H-25's configuration, stimuli, schedule and arms

## Mission

**This brief builds one generalisation of the task and runs one hypothesis once.** Every learning run from H-13 to
H-28 chose between two readouts. [ADR-0151](../../docs/adr/0151-three-answers.md) asks the named configuration a choice
among three, and changes nothing else: the stimuli, the image, the critic, its window, the address, the schedule, the
seed and the arms are H-25's.

ADR-0151 wrote **H-29** before any run:
- every mapping learned, by each stimulus;
- every coupling bounded;
- the network outside the couplings held;
- the critic holding each stimulus's expected reward.

When the round is done, the tree holds:
- a task that selects among three channels and maps each stimulus to an answer, with its own ADR and tests, and every
  run under two readouts the run it was;
- the three readouts, dealt by ADR-0151's rule, with the calibration that reads whether they still see the stimulus;
- H-29's two arms run once, as weekly `exhaustive` tests;
- H-29's verdict and **the step of its stopping rule reached**, with the next decision it names and does not take.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). The one mechanism is ADR-0151's: a third channel in
  the task's readout and an answer for each stimulus. **Nothing else is adopted**:
  - no change to `cortex-basal-ganglia`'s gate, the critic, its window, the address, the modulator, the signed gate,
    STDP, the membrane or the inhibitory rule;
  - no third stimulus, no noise in the selection, no second schedule;
  - no dependency, and no version bump of a tool.
- **One change from H-25**: the number of answers. The stimuli are ADR-0065's two, unchanged.
- **Under two readouts, bit for bit.** Every run pinned before this round is the run it was. Every pinned number holds,
  and the determinism pin does not move. Nothing of the image or the records changes.
- **The readouts are dealt by ADR-0151's rule**, and the deal is the first that passes it. The order the deals are
  tried in is committed before any coupling is read.
- **H-29's clauses and constants are written before the first rewarded run** and do not move after it. There is no
  second attempt.
- **Literature is a prior, not a verdict.** ADR-0151 cites the account that predicts a yes and names where the
  engine's premise differs. The measurement decides.
- **The engine is read before a description of it is trusted**, this brief's and ADR-0151's included.
- No `f32`/`f64`, oracles included; every operation on a state field saturates or wraps by name
  ([ADR-0029](../../docs/adr/0029-structural-enforcement.md)). Every loop ends by construction
  ([ADR-0062](../../docs/adr/0062-the-first-complete-sweeps-list.md)).
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive`. For the measurement, the runtime's gate grows by
  at most one test ([ADR-0061](../../docs/adr/0061-the-learning-runs-leave-the-gate.md)); the build's own tests are beside
  it. The mutation gate on the changed lines must pass ([ADR-0030](../../docs/adr/0030-verification-governance.md)).
- **The round is evidenced as `briefs/README.md`'s "Evidencing a round" says**
  ([ADR-0150](../../docs/adr/0150-a-round-waits-for-what-it-checks.md)): the arms run once, the merge waits for the
  dispatch's whole-domain shards and not for its sweep, the ADRs cite the pull request's commits, and the commit that
  asks for the merge sets them to `accepted`.
- **No shard of the weekly job passes 60 per cent of its bound** under the regenerated deal.
- No pinned number of an earlier round moves.
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-10-08 against `main` after ADR-0149, ADR-0150 and ADR-0151 merged. Line numbers move; the symbols
and the quoted sentences are what to re-derive.

1. **The readout** (`runtime/cortex-runtime/src/task.rs`): `Readout` holds `sets: [Set; 2]` and
   `channels: [BasalGangliaChannelState; 2]`. `select(counts: [u32; 2])` gives each channel its own count as
   `striatal_d1_drive` and the other's as `striatal_d2_drive`, calls `compute_gating` on both, and returns the one
   channel selected, *"or none when neither is or both are"*. `Readout::hold` reads `gpi_snr_inhibition` of each
   channel.
2. **The gate** (`crates/cortex-basal-ganglia/src/lib.rs`): `compute_gating` selects when
   `striatal_d2_drive + stn_hyperdirect_drive − striatal_d1_drive` is below zero.
3. **The mapping**: `Task::mirrored`, and `Task::answer(stimulus)`, which is `stimulus ^ 1` when mirrored. A schedule's
   flip negates the flag (`run_on_scheduled` in `tests/instrument/harness.rs`).
4. **The outcome and the critic**: `Outcome::counts: [u32; 2]`; `Critic::expected_q16: [i32; 2]` is per stimulus, and
   there are still two stimuli.
5. **The deliveries**: `Delivery::Drawn` addresses the engine's drawn sources onto
   `self.readout.sets[selected].units()`, and nothing at a tie.
6. **The geometry** (`tests/instrument/harness.rs`): `PERIOD` 20, `A_OFFSET` 0, `B_OFFSET` 11, `R0_MASK` 0xAA2AA,
   `R1_MASK` 0x55554, `ROTATION_1024` 0, `geometry(units, rotation) -> [Set; 4]`, `couplings`, and
   `SYNAPSES_1024` `[[775, 806], [798, 809]]`. A set's period is at most `MAX_PERIOD`, 32.
7. **The prior** (`crates/cortex-connectome/src/prior.rs`): `Prior::target` sends a synapse anywhere with the chance
   `rewire_q0_8` of 256 (a quarter in the harness's prior), and otherwise to *"A place in `1..=2W`"*, each with the
   same chance. `is_inhibitory(unit)` is true when `unit + 1` is a multiple of `inhibitory_every`, 5.
8. **The sight** (`tests/instrument/harness.rs`): `WINDOW` (from 100, 500 ticks), `LEAD_IN`, and `SEEN_MIN` 56 of 64,
   *"trials in which the window after the volley held more readout spikes than the window before the injection"*.
9. **The settled image**: `settled_image`, the image every arm since H-13 decodes; H-25's arm writes its
   configuration into it.
10. **H-25** ([ADR-0140](../../docs/adr/0140-the-address-drawn-measured.md), `tests/inhibition.rs`): `drawn_arm`, with
    both oracles held at every trial; its pinned tables and its verdict `DRAWN_1024`; the first mappings crossing at
    the 6th and 4th block and the reversals at 19, 21, 16 and 20, 15, 14 blocks; the highest coupling 1.191.
11. **The harness's oracles**: the composer replays the pair rule over the four couplings, and the network's oracle
    ([ADR-0137](../../docs/adr/0137-the-reward-unaddressed-measured.md)) replays every excitatory synapse from the train.
    Both were written for two readouts.
12. **The weekly job**: twelve shards since ADR-0150. The cost table holds 87 tests and 44 765 s; H-25's two arms are
    1 817 and 1 837 s of it.

## Deliverables

- [x] **The task for three answers built, in a new ADR at the next free number (`ls docs/adr`).**
  - *The selection*: each channel's direct drive its own count, its indirect drive the largest of the other channels'
    counts, its hyperdirect drive zero, and `compute_gating` on each, unchanged. A channel is selected when its count
    is above every other's, and none when the largest is shared.
  - *The mapping*: an answer for each stimulus in place of the flag. At a flip every stimulus's answer moves to the
    next readout, the last to the first.
  - *The tests*:
    - under two readouts a trial is the trial it was, bit for bit, and a flip is the flip it was;
    - over the lattice of three counts, the selection against a hand rule: the largest alone is selected, a shared
      largest selects nothing, and with two channels it is the rule in place;
    - every refusal of `Task::check` with three readouts, an answer that names no readout among them;
    - under each delivery, what is addressed when the third channel is selected, and at a tie.
- [x] **The readouts dealt, in an ADR, before any rewarded run.**
  - The deals that satisfy ADR-0151's rule: each readout four of the fourteen places within both stimuli's windows
    (3 to 8 and 12 to 19), one of them an inhibitory place (4, 14 or 19).
  - The order they are tried in, committed before any coupling is read.
  - The deal taken: the first whose six stimulus–readout couplings, as summed weights on the settled image, are equal
    within ten per cent with none zero. Its six couplings and the prior's census of the same six are pinned.
  - A gate test holds the constants to the rule, as `the_geometry_holds_against_the_census_at_both_sizes` holds the
    rotation.
- [x] **H-29's protocol, in an ADR, before the first rewarded run.**
  - The arms: H-25's two starting assignments through H-25's three flips, from H-25's image, with the three readouts.
    The image is asserted H-25's by its CRC.
  - H-29's clauses and constants, restated from ADR-0151 and pinned in the tests:
    - at least 80 of each mapping's last 128 correct, and among them each stimulus selecting its answer in more than
      half of its presentations;
    - no coupling above 1.30 of its image's at any block's end;
    - the excitatory sum outside the six couplings within 0.75 and 1.25 of the image's at every block's end;
    - each stimulus's mean value over each mapping's last 128 trials within a quarter of the reward of $(2p - 1)$ of
      it.
  - The predicted readings, restated: a later start, a value below zero first, elimination after a flip.
  - The readings, no clause:
    - the block each mapping passes 40 of 64 in, beside H-25's;
    - per mapping, the selections by readout for each stimulus (the answer, the old answer, the third) and the ties;
    - after each flip, the stimulus that moves onto the other's old answer beside the one that moves onto the free
      readout;
    - the six couplings' courses, and the value beside $(2p - 1)$ with its troughs;
    - where the consolidation went and the inhibitory sum's course, beside H-25's.
- [x] **The calibration**, before any rewarded run:
  - every pinned number of the tree holds, the two-answer runs among them;
  - a deal passes the readouts' rule;
  - on a frozen block of 64 trials from the settled image, the window after the volley holds more readout spikes than
    the window before the injection in at least 56;
  - read beside it, no gate: each readout's count before and after the volley, and the frozen selections by readout.
- [x] **The runs**: H-29's two arms, each a weekly `exhaustive` test, run once, their tables pinned per block.
- [x] **The ADR's reading.**
  - H-29's verdict per clause and arm.
  - Each predicted reading against what was read.
  - What the smaller readouts cost: the frozen block's counts beside the two-answer geometry's.
  - **The step of the stopping rule reached, and the next decision it names, not taken.**
- [x] **The gate.** The build's tests, and at most one runtime test for the measurement: H-29's clauses at their edges
  and the readings' rules over tables written by hand.
- [x] **The evidence**, as "Evidencing a round" says. A weekly dispatched on this round's branch at `scope=both`. Its
  twelve whole-domain shards are green and reproduce every pinned number. The cost table is regenerated from their
  artifacts. The ADR names the dispatch and says where the sweep stood.
- [x] **The documents, in the same pull request.**
  - Whitepaper §6.5 (the loop as the runtime composes it), §11.1's H-29 with its verdict and step, and §9.
  - The ADR index and `CHANGELOG.md`.
  - `CLAUDE.md` only where the standing changes (ADR-0150); `README.md` and `docs/zh-TW`'s reader's guide as the
    result requires.
  - The whitepaper's version in both declarations, with its date
    ([ADR-0064](../../docs/adr/0064-the-documentation-gate-and-the-version.md)).
- [x] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned, and the round's ADRs set to
  `accepted` by the commit that asks for the merge.

## Not empowered

- **No rule of the engine changes.** `compute_gating`, the critic, its window, the address, the modulator, the signed
  gate, STDP, the membrane and the inhibitory rule are untouched. Nothing of the image or the records changes.
- The stimuli are ADR-0065's two. No third stimulus, and no place of a stimulus moves.
- The schedule, the seed and the starting assignments are H-25's, and none is fitted to the result.
- The deal is the first that passes, in the order committed before any coupling was read. It is not chosen for what a
  frozen block or a run reads of it.
- H-29's clauses and constants do not move after a rewarded run. There is no second attempt.
- No pinned number of an earlier round moves.
- No change to `.github/workflows/ci.yml`, `scripts/exhaustive-shard.sh` or `scripts/exhaustive-costs.mjs`; the cost
  table changes only by regeneration from this round's dispatch.
- No float anywhere, no dependency, and no `unsafe` beyond ADR-0023's invariant.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- the task's shape: a readout and a task generic over the number of channels, or a three-channel form beside the
  two-channel one, as long as every two-readout run is the run it was;
- how the answers and the flip are held, and how the harness's schedule drives them;
- the order the deals are enumerated in, as long as it is committed before any coupling is read;
- how the harness's oracles and tallies read six couplings, and which oracle is held at every trial;
- how each reading is computed and pinned, and how the arms are dealt into tests;
- whether the round writes one ADR, two or three (the build, the readouts and the measurement).

It may not:
- change a rule of the engine or of a state crate;
- add a stimulus, move one, or change the schedule;
- change the readouts' rule or take a deal other than the first that passes;
- move a clause or a constant after a rewarded run;
- reach the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

If no deal passes the rule, or the frozen block fails the sight, the round stops there as H-29's stopping rule says:
the build stays, the finding is written, and no arm is run.

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
gh workflow run ci.yml --ref <this round's branch> -f scope=both
node scripts/exhaustive-costs.mjs from <the dispatch's downloaded artifacts> --run <its id> > scripts/exhaustive-costs.tsv
```

Every command exits 0. The dispatched run's twelve whole-domain shards are green and reproduce every pinned number;
the merge does not wait for its sweep. In the history, the build, its tests, the order of the deals, the deal taken
and H-29's constants precede the first rewarded run.

## Report

The closing message states:
- the task as built: the selection among three, the mapping, and that under two readouts nothing moved;
- the readouts: the order of the deals, the deal taken and its six couplings;
- the protocol, and when it was committed;
- the calibration, with the frozen block's counts;
- H-29's verdict per clause and arm, each predicted reading against what was read, and the readings;
- the step of the stopping rule reached and the next decision it names, not taken;
- that no pinned number moved;
- the scope ADR-0075 gave the diff, the shards' times, where the sweep stood at the merge, and the regenerated cost
  table;
- what was not done and why.
