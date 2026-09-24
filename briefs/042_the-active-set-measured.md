---
status: proposed
date: 2026-09-25
---

# Brief 042: The active set measured — how many units the executor serves per tick on the reference network, how long one message keeps a unit awake, and the budget Appendix A's targets must fit; measured, nothing changed

## Mission

**This brief measures and changes no rule.** The executor serves a unit every tick "from the one it is woken
until it is at rest: every potential zero, no refractory or plateau window, the threshold at or below its base"
([ADR-0023](../docs/adr/0023-executor.md)), and "a unit that is at rest costs nothing". Whether the engine can
meet [Appendix A](../docs/WHITEPAPER.md)'s plan — 43 000 000 units on 64 workers — at a tick of 10 µs therefore
turns on **how many units are not at rest in a tick**, which the tree has never measured. The whitepaper's own
premise is that nervous tissue "is roughly 1–2 % active at any instant" (§1). The reference network's drive
([ADR-0044](../docs/adr/0044-reference-network.md)) sends a message to each unit about every 128 ticks, and one
message of 0.125 leaks to zero in about two thousand, so the arithmetic of the rules says the executor's active set
on that network is close to every unit, every tick. This round measures it.

When the round is done, the tree holds: the engine's own count of the turns it served per tick; the active fraction
on the reference network at 1 024 and 4 096 units under ADR-0044's drive, and at 1 024 units under two sparser
drives; the ticks one message keeps a unit awake, from an oracle of the membrane's leak held to the engine; the
relation between the input's rate and the active fraction, read against the prediction written below; the per-turn,
per-message and per-integration costs the bench reports, as developer-machine figures; and **an ADR that writes the
budget Appendix A's three targets — the unit count, the tick in real time, the input's density — must jointly fit**,
with a numbered finding if the measured active set contradicts §1's premise. **The decision about the integration
model — per tick, or on arrival — is the next ADR's, not this round's.** The maintainers have said they prefer to
keep integrating every awake unit every tick, so that a dynamic with no closed form between two events stays
admissible in the membrane; the budget this round writes must therefore say as fully **what per-tick service can
reach** — the units, the tick in real time and the input's density it can meet together, and which one must give — as
what service on arrival would change.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). This round adopts nothing: no event-driven
  integration, no SIMD, no change of the scheduling, the ownership of units or the turn gate, no new tool, no
  dependency, no version bump of a tool. A counter the executor keeps is the only addition to the engine this round
  may make, and it changes no behaviour.
- Every claim is Implemented, Specified, Target or Hypothesis ([ADR-0010](../docs/adr/0010-measured-or-target.md)).
  **A benchmark figure from a developer machine is not Measured**: it is reported as the protocol in
  `docs/benchmarks/README.md` says, never written into the whitepaper's tables, and any budget computed with it is an
  estimate, labelled so. The active fraction, the ticks to rest and the message rates are counts the tests read from
  the engine, pinned like every other reading, and are Measured in that sense on this tree.
- The repository wins over the document; **a disagreement between §1's premise or Appendix A and what the tree
  measures is a numbered finding** in §11, not a silent edit of either.
- No `f32`/`f64` anywhere, oracles included — the prediction's exponential is computed in integers, as the tree's
  decays are; every operation on a state field saturates or wraps by name
  ([ADR-0029](../docs/adr/0029-structural-enforcement.md)).
- Every loop ends by construction ([ADR-0062](../docs/adr/0062-the-first-complete-sweeps-list.md)).
- **No rule of the engine moves**: not the membrane, the leak, the rest condition, the drive's rule, the scheduling,
  the deque, the barrier, the turn gate. **The determinism pin of [ADR-0030](../docs/adr/0030-verification-governance.md)
  does not move, and no number a prior round pinned moves.** A counter, if added, is read between ticks and written
  where `delivered` is; the mutation gate on the changed lines must pass.
- **The engine is read before a description of it is trusted**, this brief's included: the ~2 000 ticks to rest and
  the ~128 ticks between messages above are arithmetic on `leak`, `at_rest` and ADR-0044's `Drive`, not readings.
  In particular the executor scales an injected message by the tick's gain (F-47), so the magnitude a unit receives is
  not the drive's efficacy.
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive` and runs in the weekly job; the runtime's gate
  grows by at most one test ([ADR-0061](../docs/adr/0061-the-learning-runs-leave-the-gate.md)).
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-09-25 against `main` at `03bd296`, after brief 041's round merged (the image format is 16; the
executor's scheduling, `at_rest`, `leak` and ADR-0044's drive are as below). Line numbers move; the symbols and the
quoted sentences are what to re-derive.

1. **The active set** ([ADR-0023](../docs/adr/0023-executor.md), `runtime/cortex-runtime/src/executor.rs`): phase 1
   of a tick pops units from the local deque and steals when it is empty; per unit `begin_turn`, drain the mailbox,
   sort, sum, `integrate(basal, apical, t)`, `step_stp` if it fired, `end_turn`, and "if the unit is not at rest
   `try_schedule` it onto the worker's next-tick list". `at_rest` (`pub(crate)`) is the rest condition. A message
   wakes a unit at rest. **Nothing the executor exposes counts the turns it serves**; `Executor::delivered()` counts
   the messages delivered, summed over the workers.
2. **The leak** (`crates/cortex-core/src/dynamics/membrane.rs`): `BASAL_LEAK_SHIFT` is 9; `leak(v, shift)` moves `v`
   toward zero by `max(|v| >> shift, 1)` and never past it. From a potential `v` the ticks to zero are about
   `512 × ln(v / 512)` in the proportional phase and then one LSB a tick, so from 0.125 (8 192 LSB) about 1 900 —
   an estimate from the rule, which the round's oracle replaces with the count.
3. **The drive** ([ADR-0044](../docs/adr/0044-reference-network.md); `drive(units)` in `tests/reference.rs` and the
   instrument's harness): "every tick, one message of 0.125 per 128 units into units drawn" uniformly — each unit
   about one message every 128 ticks, about **780 a second** at 10 µs a tick — scaled by the gain on arrival (F-47).
   `Drive` is `{ every, messages, efficacy_q16, units, seed }`, "a function of the tick alone".
4. **The rates the tree has read**: the instrument's regime at the gain 1.75 fires 1.6 to 1.8 Hz per unit and the
   controller's 12 to 41 Hz ([ADR-0077](../docs/adr/0077-the-background-side.md)); the reference network at 4 096 units
   is ADR-0051's configuration.
5. **The prediction's shape.** If one message keeps a unit awake for `D` ticks and messages arrive at a rate `r` per
   tick, independently, the fraction of ticks a unit is not at rest is about `1 − e^{−rD}` (a unit is at rest only if
   no message arrived in the last `D` ticks). Under ADR-0044's drive `rD ≈ 1 900 / 128 ≈ 15`: about **100 %**. A drive
   sixteen times sparser (`rD ≈ 0.93`): about **60 %**. Two hundred and fifty-six times sparser (`rD ≈ 0.058`):
   about **6 %**. The network's own spikes add messages, so these are floors. §1's 1–2 % needs `rD ≈ 0.01–0.02`,
   about one message per unit per second.
6. **The bench** (`benches/cortex-bench/benches/hot_path.rs`): `gate/schedule_begin_end`, `neuron/integrate`,
   `mailbox/push_drain_x16`, `executor/push_to_turn`, among others; the protocol for a Measured figure is
   `docs/benchmarks/README.md`'s, and a developer machine's figure is not admissible.
7. **Appendix A**: `N_neuron` = 43 000 000, workers = 64, the `DendriticSuperNeuron` arena 2.75 GB; the tick is
   `TICK_NS` = 10 000 ([ADR-0033](../docs/adr/0033-tick-duration-in-the-header.md)). Serving every unit every tick
   touches the whole arena every 10 µs.
8. **The weekly shards** ([ADR-0092](../docs/adr/0092-the-shards-dealt-by-cost.md)): a round that adds a whole-domain
   test regenerates `scripts/exhaustive-costs.tsv` from its own dispatch. **This brief names no dispatch scope**: the
   round takes it from its own diff under [ADR-0075](../docs/adr/0075-the-dispatch-scope-follows-the-diff.md) — a
   counter added under `src/` sends it through the whole-tree sweep.

## Deliverables

- [ ] **The count (the next free ADR number; `ls docs/adr`)**. The turns the executor serves, counted by the engine
  where it counts `delivered` — per worker in the turn phase, published between ticks, summed by an accessor — **or**,
  if the round argues it better, a test-side replica of `at_rest` held to the engine; either way the round says which
  and why, and **a counter changes no behaviour**: the determinism pin and every pinned number unchanged, its own test
  (a tick with no unit awake serves none; one message into an armed unit at rest serves that unit on every tick until
  it rests, and not after).
- [ ] **The ticks to rest.** An oracle that steps `leak` alone from a given magnitude, written before the engine is
  run; the engine held to it on an armed unit in a network of no synapse: one message of 0.125 at the gain 1.0 and at
  the instrument's 1.75, the unit's turns counted to rest. The oracle's count and the engine's agree, or the
  disagreement is a finding.
- [ ] **The prediction, written first.** The relation `1 − e^{−rD}` computed in integers for each drive below from the
  measured `D`, committed before the runs, as a floor the network's own spikes may raise.
- [ ] **The active set on the reference network.** Per tick: the turns served, the messages delivered (drive and
  synaptic), the spikes; read over a lead-in of one window and four windows of $2^{17}$ ticks, pinned per window:
  (a) 1 024 units, ADR-0044's drive, the instrument's configuration (the prior at seed 22, the gain 1.75, the controller
  off, the baseline as the instrument's calibration has it); (b) 1 024 units, the same with the drive's messages
  sixteen times fewer; (c) the same, two hundred and fifty-six times fewer; (d) 4 096 units, ADR-0044's drive, ADR-0051's
  configuration. Each reading beside the prediction. Heavy runs are weekly `exhaustive` tests.
- [ ] **The costs, as the bench reports them.** `cargo bench -p cortex-bench --bench hot_path` for the gate, the
  integration, the mailbox and the push-to-turn path, reported as developer-machine figures under
  `docs/benchmarks/README.md`'s protocol — not admissible, not written into the whitepaper's tables.
- [ ] **The budget, in the ADR.** Appendix A's three targets written as one relation — the units, the fraction of them
  served per tick, the cost of a turn, the workers, the tick — with the measured fractions in it and the bench's
  figures as a labelled estimate; beside it the same relation for serving a unit only when a message arrives, so the
  next decision has both sides; for per-tick service, the largest unit count that meets a 10 µs tick in real time at
  each measured input density, and the slowdown against real time at Appendix A's count; and what the arena's size
  alone implies for serving every unit every tick. **No integration model is chosen.**
- [ ] **The finding, if it applies.** If the measured active fraction under ADR-0044's drive contradicts §1's premise
  of 1–2 %, or the relation shows Appendix A's targets cannot be met together by the executor as built, a numbered
  finding in §11 stating the disagreement and its evidence; Appendix A's line and §1's sentence are not edited to fit.
- [ ] **The gate.** At most one runtime test: the counter's edges and a few hundred ticks of (a) held to the first rows
  of its table.
- [ ] **The evidence.** A weekly dispatched on this round's branch at the scope ADR-0075 gives this round's own diff,
  the clause that applied stated in the ADR; green in every job it runs, every pinned number reproduced, the sweep's
  survivors if any dispositioned; the cost table regenerated from that run (ADR-0092).
- [ ] **The documents, in the same pull request.** Whitepaper §11 (the finding, if any), §9, Appendix A's section a
  pointer to the ADR's budget; the ADR index; `CHANGELOG.md`; `CLAUDE.md` and `docs/zh-TW`'s reader's guide as the
  result requires. The whitepaper's version moves in both declarations with its date
  ([ADR-0064](../docs/adr/0064-the-documentation-gate-and-the-version.md)), in the same commit as any edit to it.
- [ ] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned.

## Not empowered

- **No rule of the engine changes**, and no integration model is chosen, built or prototyped: not event-driven
  integration, not a closed-form leak, not SIMD, not fixed ownership, not a change to the turn gate or the deque.
- **The drive's rule does not change**; the sparser drives are `Drive` values with fewer messages, in tests only.
- No pinned number moves; the determinism pin does not move; the image format does not move.
- No developer-machine figure is written into the whitepaper's tables or called Measured.
- §1's premise and Appendix A's plan are not edited to agree with the reading; a disagreement is a finding.
- No new crate, no dependency, no `unsafe` beyond ADR-0023's, no float anywhere.
- No change to `.github/workflows/ci.yml`, `scripts/exhaustive-shard.sh` or `scripts/exhaustive-costs.mjs`; the cost
  table changes only by regeneration from this round's dispatch.
- The learning line (H-18 and after) is not touched.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in the ADR: whether the
count is an engine counter or a test-side replica, and how the counter is published, so long as it changes no
behaviour and the determinism pin stands; the drives' exact sparser densities, so long as one is near `rD ≈ 1` and one
well below it; the windows' number and length, so long as each configuration is read after a lead-in and over at
least four windows; whether 4 096 units are read under ADR-0051's configuration or the instrument's; how the
prediction's exponential is computed in integers; whether the round writes one ADR or two; and which figures the ADR
restates. It may not reach the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`, and
it may not choose the integration model.

## Verification

```bash
cargo check --workspace --all-targets --locked
cargo test --workspace --locked
cargo test --workspace --release --locked
cargo test --workspace --release --locked -- --ignored exhaustive --list
cargo test -p cortex-runtime --release --locked -- --ignored exhaustive
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked
cargo bench -p cortex-bench --bench hot_path --locked -- --test
cargo bench -p cortex-bench --bench hot_path --locked
cargo +1.85 check --workspace --all-targets --locked
cargo +1.85 test --workspace --locked
npm ci
npm run spec
git diff main...HEAD > target/pr.diff && cargo mutants --workspace --in-diff target/pr.diff
gh workflow run ci.yml --ref <this round's branch> -f scope=<what ADR-0075 gives this diff>
node scripts/exhaustive-costs.mjs from <the dispatch's downloaded artifacts> --run <its id> > scripts/exhaustive-costs.tsv
```

Every command exits 0; the in-diff gate reports no survivor in CI; the dispatched run is green and reproduces every
pinned number; the determinism pin and the image format are unchanged.

## Report

The closing message states: how the turns were counted and that no behaviour changed; the ticks to rest from one
message at the two gains, the oracle's and the engine's; the active fraction, the messages and the spikes per unit per
second in each configuration, beside the prediction; the bench's figures, labelled as a developer machine's; the budget
relation and what it says about Appendix A's three targets, for per-tick service and for service on arrival; the
finding, if one was opened; the scope ADR-0075 gave this diff, the shards' times and the regenerated cost table; what
the mutation gate and any sweep found; what was not done and why; and what the next decision — the integration model —
should weigh.
