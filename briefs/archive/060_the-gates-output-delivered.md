---
status: archived
date: 2026-10-07
---

> **Executed 2026-10-07 in pull request #180.** Writes ADR-0144 (the gate's output delivered, built) and ADR-0145 (the
> gate's output delivered, measured); opens and closes F-63, and opens F-64. The gate's output was built as `Hold`, a
> parameter of the task: the selection made before the tick the readout window closes at, and the channel whose net
> output the gate left above zero taking basal messages of negative efficacy on a cadence from there, nothing at a tie.
> The released delivery is `Delivery::Released`, the drawn address with every unit a target. No rule of the engine, no
> record and nothing of the image changed, and a task with no hold runs the trial it ran, which a test holds against
> the trial before the hold written out. The hold was derived by an oracle of the membrane's rule before any network
> ran with one: one message at the bound every 512 ticks from the window's close below tick 4 096. Its frozen check
> passed, the held readout firing 5 spikes within the span against the selected readout's 1 494. The build, the hold,
> the check's rule and H-27's constants were committed and pushed before any rewarded trial of an arm, with the pull
> request opened as a draft; before the arms every one of the weekly job's 81 earlier whole-domain tests passed with no
> task carrying a hold.
>
> **H-27 is no, on clauses 1 and 3, and its stopping rule reached step 5.** With the hold, from the assignment every
> mapping was learned (96, 87, 83 and 87 of each mapping's last 128) and two reversals passed 40 of 64 in 25 and 24
> blocks, past the bound of 23; from the mirrored assignment two mappings were not (117, 72, 95, 75), one reversal never
> passed and one took 30 blocks. Clauses 2 and 4 held in both arms. The control, the release alone, read no on the same
> two clauses, as predicted. The hold silenced the held readout within the span through every mapping, and under it the
> consolidation on the side not selected turned from a cost of 0.52 to 0.63 of the selected side's signal into a gain,
> the run's signal 1.02 to 1.59 of H-25's; the learning was slower all the same, with the couplings fallen as a whole,
> the readouts firing a sixth less and the inhibitory sum at 0.78 of the image's and falling. The next decision, an ADR
> on the attention-gated feedback with these readings as its need, is named and not taken.
>
> Every deliverable below is done and its box ticked. Relative links gained one `../` so that they resolve from
> `archive/`; no word, claim or figure changed.
>
> *The body below describes the tree before execution and is not maintained.*

# Brief 060: The gate's output delivered — ADR-0143's competition built, the channel not selected inhibited from the readout window's close through the pair rule's span, and the address's targets released to every unit; then H-27 run once on H-25's configuration, with the release alone as its control

## Mission

**This brief builds one mechanism of the task and one delivery, and runs one hypothesis once with its control.**
H-25 learned with the reward's address drawn by the engine: its sources come from the critic's window, and its
targets are the readout the selection chose, an efference copy. H-26
([ADR-0142](../../docs/adr/0142-the-readouts-own-competition-measured.md)) read that releasing the targets to every unit
would cost about half the learning signal: outside the readout window, the readout that lost fires as the winner
does.

[ADR-0143](../../docs/adr/0143-the-gates-output-delivered.md) delivers the gate's output to the network:
- the selection is made at the readout window's close;
- the channel not selected is inhibited from there to tick $2^{12}$ of the trial;
- the address's targets are released to every unit, its sources still drawn.

ADR-0143 wrote **H-27** before any run, with the release alone as its control.

When the round is done, the tree holds:
- the gate's output and the released delivery in the task, with their own ADR and tests, and every run without them
  the run it was;
- H-27's two arms and the control's two, run once, as weekly `exhaustive` tests;
- H-27's verdict and **the step of its stopping rule reached**, with the next decision it names and does not take.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). The mechanisms are ADR-0143's two. **Nothing else
  is adopted**:
  - no lateral wiring between the readouts and no feedback tag;
  - no change to the critic, its window, the modulator, STDP, the membrane, the gating rule or the inhibitory rule;
  - no dependency, and no version bump of a tool.
- **Unset, bit for bit.** With the gate's output unset and under the three deliveries the tree has, every run is the
  run it was. Every pinned number holds, and the determinism pin does not move. Nothing of the image or the records
  changes.
- **The selection is what it was.** It is made at the readout window's close from the counts it reads today. With
  the gate's output unset, a run's selections are bit for bit the ones it had.
- **The schedule is derived, not fitted.** The messages a tick and the ticks are the least that silence a unit through
  the span, by an oracle of the membrane's rule. They are checked on a frozen run before any rewarded one, and do not
  move after it.
- **The span is ADR-0143's**: from the readout window's close to tick $2^{12}$ of the trial.
- **H-27's clauses and constants are written before the first rewarded run** and do not move after it. There is no
  second attempt.
- **The engine is read before a description of it is trusted**, this brief's and ADR-0143's included.
- No `f32`/`f64`, oracles included; every operation on a state field saturates or wraps by name
  ([ADR-0029](../../docs/adr/0029-structural-enforcement.md)). Every loop ends by construction
  ([ADR-0062](../../docs/adr/0062-the-first-complete-sweeps-list.md)).
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive`. For the measurement, the runtime's gate grows by
  at most one test ([ADR-0061](../../docs/adr/0061-the-learning-runs-leave-the-gate.md)); the build's own tests are beside
  it. The mutation gate on the changed lines must pass ([ADR-0030](../../docs/adr/0030-verification-governance.md)).
- **No shard of the weekly job passes 60 per cent of its bound** under the regenerated deal.
- No pinned number of an earlier round moves.
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-10-07 against `main` after ADR-0142 and ADR-0143 merged. Line numbers move; the symbols and the
quoted sentences are what to re-derive.

1. **The selection** (`runtime/cortex-runtime/src/task.rs`): `Task::trial` runs the trial's ticks, then counts the
   readouts' spikes over the task's `Window` and calls `Readout::select`. In the harness the window is 100 to 600
   ticks after the trial's first (`WINDOW`, `tests/instrument/harness.rs`).
2. **The gate** (`crates/cortex-basal-ganglia/src/lib.rs`): `compute_gating` writes `gpi_snr_inhibition`, the net
   output, and selects where it falls below zero. The channel not selected holds an output above zero, and at a tie
   both hold zero.
3. **The cancel** (`runtime/cortex-runtime/src/task.rs`, `Cancel`, [ADR-0076](../../docs/adr/0076-two-injections.md)):
   negative basal messages into a set's units, injected between ticks of the trial from an offset. *"The executor
   scales the message by the tick's synaptic gain as it scales a synapse's (F-47), and clamps one message's efficacy at
   −2.0"*.
4. **The membrane** (`crates/cortex-core/src/dynamics/membrane.rs`): `BASAL_LEAK_SHIFT` 9, `SOMA_LEAK_SHIFT` 11,
   `REFRACTORY_TICKS` 200. A compartment leaks toward zero by a fraction of itself, at least one LSB a tick.
5. **The pair rule** (`crates/cortex-core/src/dynamics/synapse.rs`, `step_stdp`): nearest-neighbour, against the
   target's **last** somatic spike, with the factor $(1 - 2^{-11})^{\Delta t}$.
6. **The drawn address** ([ADR-0139](../../docs/adr/0139-the-address-drawn-built.md)): `Executor::address_drawn(targets)`
   and `Delivery::Drawn`.
7. **H-25 and H-26** ([ADR-0140](../../docs/adr/0140-the-address-drawn-measured.md),
   [ADR-0142](../../docs/adr/0142-the-readouts-own-competition-measured.md), `tests/inhibition.rs`):
   - `drawn_arm`, `shadowed_run` and `earned_run_shadowed` on the shared harness;
   - both oracles held at every trial;
   - the readings `Heard`, `Eligible` and `Went`;
   - H-26's cost over the signal, 0.486 to 0.602.
8. **The weekly job**: six shards. A round that changes `src/` dispatches `scope=both`
   ([ADR-0075](../../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)). After ADR-0142 the cost table plans about 39
   per cent of the bound.

## Deliverables

- [x] **The gate's output and the released delivery built, in a new ADR at the next free number (`ls docs/adr`).**
  - *The gate's output*: a parameter of the task, unset by default. With it set:
    - the selection is made at the readout window's close;
    - the schedule's messages go into every unit of the channel not selected, through the span;
    - nothing is delivered at a tie.
  - *The released delivery*: the sources drawn as `Delivery::Drawn` draws them and every unit a target, at a tie too.
    It is refused without the critic's window.
  - *The tests*:
    - unset, a trial is the trial it was, bit for bit, selections included;
    - set, the messages land on exactly the losing channel's units, over exactly the span, and none at a tie;
    - the selection at the window's close equals the selection at the trial's end;
    - the released delivery's sources are the drawn ones and its targets every unit;
    - each refusal.
- [x] **The schedule, derived and checked before any rewarded run.**
  - By an oracle of the membrane's rule: the least messages a tick and the fewest ticks that keep a unit of the settled
    network from firing from the window's close to tick $2^{12}$ under the drive.
  - On a frozen block of 64 trials: the losing channel's spikes within the span are at most a tenth of the selected
    channel's in the same span. A schedule that fails this stops the round as a finding.
- [x] **H-27's protocol, in an ADR, before the first rewarded run.**
  - The arms, from H-25's image: the released delivery with the gate's output, from both assignments; and the control,
    the released delivery alone, from both.
  - H-27's four clauses and constants, restated from ADR-0143 and pinned in the tests: learned, bounded, each reversal
    within 23 blocks, and the excitatory sum outside the four couplings within 0.75 and 1.25 of the image's.
  - The readings, no clause, for each arm:
    - the cost and the signal closed-loop, on the selected side and the side not selected;
    - the losing channel's spikes within the span and outside it, beside the selected channel's;
    - the eligibility at each reward onto each readout, by magnitude and on net, beside ADR-0142's;
    - the messages delivered a trial;
    - the population's rate by class and the inhibitory sum's course;
    - the couplings' separation, the reversal speeds, the value and the troughs, beside H-25's.
- [x] **The calibration**, before any rewarded run: every pinned number holds with the gate's output unset; H-25's
  first block reproduces under the drawn address; the schedule passes its frozen check.
- [x] **The runs**: four weekly `exhaustive` tests, H-27's two arms and the control's two, their tables pinned per
  block.
- [x] **The ADR's reading.**
  - H-27's verdict per clause and arm.
  - The control by the same clauses, with its prediction (no) against the reading.
  - The readings: which of the two effects ADR-0143 names was the larger, the background removed or the window's
    response kept.
  - **The step of the stopping rule reached, and the next decision it names, not taken.**
- [x] **The gate.** The build's tests, and at most one runtime test for the measurement: H-27's clauses at their edges,
  the schedule's rule over values written by hand, and the readings' rules.
- [x] **The evidence.** A weekly dispatched on this round's branch at `scope=both`. It must be green in every job, with
  no survivor of the sweep on the new lines and every pinned number reproduced. The cost table is regenerated from that
  run's artifacts.
- [x] **The documents, in the same pull request.**
  - Whitepaper §5.2.5's status and its composition by the runtime, §8.8's row for striatal action selection, §11.1's
    H-27 with its verdict and step, and §9.
  - The ADR index, `CHANGELOG.md`, `CLAUDE.md`, and `docs/zh-TW`'s reader's guide as the result requires.
  - The whitepaper's version in both declarations, with its date
    ([ADR-0064](../../docs/adr/0064-the-documentation-gate-and-the-version.md)).
- [x] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned.

## Not empowered

- **No rule of the engine changes.** The critic, its window, the modulator, STDP, the membrane, the gating rule and the
  inhibitory rule are untouched. Nothing of the image or the records changes.
- The span is not fitted to the learning, and the schedule does not move after a rewarded run.
- No lateral wiring between the readouts and no feedback tag.
- H-27's clauses and constants do not move after a rewarded run. There is no second attempt.
- No pinned number of an earlier round moves.
- No change to `.github/workflows/ci.yml`, `scripts/exhaustive-shard.sh` or `scripts/exhaustive-costs.mjs`; the cost
  table changes only by regeneration from this round's dispatch.
- No float anywhere, no dependency, and no `unsafe` beyond ADR-0023's invariant.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- the names and the shape of the task's parameter and of the delivery, and how each is refused;
- how the trial is composed around a selection at the window's close;
- the schedule's form, within "the least that silences a unit through the span";
- how the arms are built on the harness, and how they are dealt into tests;
- what the oracles replay, and how each reading is computed and pinned;
- whether the round writes one ADR or two (the build and the measurement).

It may not:
- change a rule of the engine;
- fit the span or the schedule to the learning;
- move a clause or a constant after a rewarded run;
- reach the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

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

Every command exits 0. The dispatched run is green and reproduces every pinned number. In the history, the build, its
tests, the schedule with its frozen check, and H-27's constants precede the first rewarded run.

## Report

The closing message states:
- the gate's output and the released delivery as built, the refusals, and that unset nothing moved;
- the schedule as derived, its frozen check, and when it was committed;
- the calibration;
- H-27's verdict per clause and arm, the control's reading against its prediction, and the readings;
- the step of the stopping rule reached and the next decision it names, not taken;
- that no pinned number moved;
- the scope ADR-0075 gave the diff, the shards' times and the regenerated cost table;
- what was not done and why.
