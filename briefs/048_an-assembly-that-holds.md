---
status: proposed
date: 2026-09-27
---

# Brief 048: An assembly that holds — on the learning line's settled network, every weight frozen, a grid of assemblies wired among its excitatory units: does one hold its activity after a kick, stay quiet without one, and let go on a signal; measured, nothing changed

## Mission

**This brief measures and changes no rule.** [ADR-0111](../docs/adr/0111-a-rule-held-by-the-network.md) took a
representation of the rule in force as the learning line's next question. H-20 read no learning set
([ADR-0110](../docs/adr/0110-a-schedule-of-reversals-measured.md)): every reversal was a relearning, 13 to 23 blocks
each. The prefrontal account is a context held as persistent activity, under which each mapping is kept, so that a
reversal becomes a switch.

ADR-0111's first round asks whether the substrate exists: **can an assembly of this network hold its activity once
kicked, stay quiet when not, and let go on a signal?** The rules give reasons to doubt it. Short-term plasticity is
depression-dominated (release 0.2, facilitation $\tau_f \approx 164$ ms, depression $\tau_d \approx 328$ ms), the
threshold adapts by 0.02 a spike over about 41 ms, and the prior was not built to hold anything.

When the round is done, the tree holds:
- a grid of assemblies, sizes and recurrent weights, written before any run, each wired as the test's own network on
  ADR-0077's settled image with every weight frozen;
- for each cell, whether it **holds**, **ignites** without a kick, **lets go** on a signal, and **spills** into the rest
  of the network, by rules committed before any run;
- a new ADR that records the region where an assembly is usable, if one exists, and names the next decision
  **without taking it**.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). This round adopts nothing:
  - no rule of the engine changes: not the membrane, short-term plasticity, STDP, the modulator, the executor, the
    drive, the task, the critic;
  - no dependency, and no version bump of a tool.

  An assembly is the test's own wiring, synapses added to the network in the test, as ADR-0097's controls were wired
  to nothing.
- **Every weight frozen.** The excitatory baseline is zero, the inhibitory baseline unset, the signed gate unset, and no
  reward is delivered, so no synapse consolidates. The round shows it: every weight at the end of each cell's run
  equals its value at the start.
- **The grid, the measures and their thresholds are written before any run**, and none moves after one. The grid may
  be widened before the first run, not narrowed.
- **No prediction is required for the region, but the rules' arithmetic is written first.** Before any run, the round
  writes:
  - the steady efficacy of an assembly synapse at a sustained 20 Hz under `step_stp`'s rule, as a fraction of its
    efficacy at rest;
  - what one kicked unit's spike delivers to its targets at each weight of the grid, against the threshold.
- No `f32`/`f64`, oracles included; every operation on a state field saturates or wraps by name
  ([ADR-0029](../docs/adr/0029-structural-enforcement.md)). Every loop ends by construction
  ([ADR-0062](../docs/adr/0062-the-first-complete-sweeps-list.md)).
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive`; the runtime's gate grows by at most one test
  ([ADR-0061](../docs/adr/0061-the-learning-runs-leave-the-gate.md)). The mutation gate on the changed lines must pass
  ([ADR-0030](../docs/adr/0030-verification-governance.md)).
- **The engine is read before a description of it is trusted**, this brief's and ADR-0111's included. The executor
  scales an injected message by the gain (F-47), so a kick's shape is read on the engine before it is used.
- No pinned number of an earlier round moves.
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-09-27 against `main` after ADR-0110 merged. Line numbers move; the symbols and the quoted sentences
are what to re-derive.

1. **The network** (`runtime/cortex-runtime/tests/instrument/harness.rs`). `prior(units)` is ADR-0044's prior at seed
   22: a fifth of the units inhibitory at the rail, 32 synapses a unit, a window of eight, a quarter rewired, local
   delays of 1 to 3 ms and far ones of 14 to 25.6 ms, excitatory weights in $[6\,000, 12\,000]$ of Q1.15. ADR-0077's
   settled candidate and its image are what the learning line starts from; `config(units, workers, baseline)` sizes
   the arena with `blocks_for(&prior(units))` blocks.
2. **The wiring.** `SynapseBlock::set_synapse`, `SynapseBlock::link` and `DendriticSuperNeuron::set_first_block`
   (`crates/cortex-core/src/dynamics/synapse.rs`) write a unit's fan-out as a chain of blocks. An assembly's added
   synapses need blocks beyond the prior's, chained after each unit's own.
3. **Short-term plasticity** (`crates/cortex-core/src/dynamics/plasticity.rs`, `step_stp`):
   - `STP_U` is 51/256;
   - `STP_TAU_F_SHIFT` is 14 and `STP_TAU_D_SHIFT` is 15;
   - at each presynaptic spike, $u$ facilitates by $U(1-u)$ and $R$ is depleted by $uR$, both relaxing toward rest
     between spikes.
4. **The membrane** (`membrane.rs`): the threshold at 1.0 and adapting by `THRESHOLD_STEP` (0.02) with
   `THRESHOLD_DECAY_SHIFT` 12; `REFRACTORY_TICKS` 200.
5. **The kick and the cancel** ([ADR-0076](../docs/adr/0076-two-injections.md)): a stimulus shape that fires each unit
   of a set once under the drive, and a cancel of negative basal messages derived from the refractory window. Both are
   in the harness, `SHAPE_F46` and `CANCEL_AT_THE_EXTREME` among them.
6. **The rates** (ADR-0097): about 1.76 Hz a unit on the reference network under ADR-0044's drive.
7. **The task's sets** (the harness's stimulus and readout sets), which an assembly must not share a unit with, so that
   a later round can put the task and the context in the same network.
8. **The weekly job** ([ADR-0092](../docs/adr/0092-the-shards-dealt-by-cost.md),
   [ADR-0075](../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)): a round that changes only tests dispatches
   `scope=exhaustive`. The cost table is regenerated from its own dispatch.

## Deliverables

- [ ] **The grid and the protocol, in a new ADR at the next free number (`ls docs/adr`), before any run.** At least:
  - *Sizes and weights:* assemblies of **16, 32 and 64** excitatory units, disjoint from the task's sets, with added
    recurrent synapses among them at **four weights**, 0.25, 0.5, 0.75 and 1.0 of Q1.15's range. Each unit targets
    the assembly's other units up to 32 of them, with delays from the local band.
  - *The protocol:*
    - a lead-in under the drive;
    - epochs of $2^{14}$ ticks (one trial's length) without a kick;
    - epochs with a kick at the start;
    - epochs with a kick and a release signal part way.
  - *The release signal:* one, chosen and written before any run (ADR-0076's cancel into the assembly, or a volley into
    nearby inhibitory units).
  - *The measures and their thresholds*, committed as rules:
    - the assembly's background rate;
    - **holds**: the rate over the last half of a kicked epoch at least five times the background, in at least 7 of 8
      kicked epochs;
    - **ignites**: an unkicked epoch whose last half reaches that rate;
    - **lets go**: the rate over the tail after the release at most twice the background, in at least 7 of 8;
    - **spills**: the rest of the network's rate while the assembly holds more than twice its background;
    - a cell is **usable** when it holds, lets go, ignites in at most 1 of 8 unkicked epochs, and does not spill.
- [ ] **The arithmetic, before any run**: `step_stp`'s steady efficacy at 20 Hz as a fraction of the efficacy at rest,
  and what one spike delivers to a target at each weight against the threshold. The kick read on the engine: each unit
  of each assembly fires once, or the shape is derived again until it does, before any cell is run.
- [ ] **The runs.** Every cell of the grid, each an `exhaustive` test or a few cells to a test, the weights shown
  unchanged at each run's end. Tables pinned per epoch: the assembly's spikes by window, the rest of the network's,
  the ignitions, and the assembly's short-term state (the mean $u$ and $R$) at each window's end.
- [ ] **The ADR's reading.** The grid table: holds, ignites, lets go, spills, usable, per cell. The usable region if one
  exists. The arithmetic beside what the runs read. If no cell is usable, what failed in each: never holding, running
  away, or not letting go. **The next decision named and not taken**:
  - a readout gated by the context, if a region is usable;
  - otherwise a mechanism of persistence (synapses whose facilitation outlasts their depression for the context's units,
    or a slower current), with this round's readings as its need.
- [ ] **The gate.** At most one runtime test: the rules at their edges over tables written by hand, and one assembly
  wired and kicked for a few hundred ticks.
- [ ] **The evidence.** A weekly dispatched on this round's branch at the scope ADR-0075 gives the diff (`exhaustive`
  if only tests change), the clause stated in the ADR. It must be green in every job, and every pinned number
  reproduced. The cost table is regenerated from that run's artifacts.
- [ ] **The documents, in the same pull request.** Whitepaper §11.1's question on a rule held by the network, with this
  round's reading, and §9; the ADR index; `CHANGELOG.md`; `CLAUDE.md`; `docs/zh-TW`'s reader's guide as the result
  requires. The whitepaper's version in both declarations, with its date
  ([ADR-0064](../docs/adr/0064-the-documentation-gate-and-the-version.md)).
- [ ] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned.

## Not empowered

- **No rule of the engine changes**, and no weight moves during a run. The assembly's synapses are test wiring, not a
  change to the prior or to the image format.
- No context, no gated readout, no switch on errors, no reward. Those are ADR-0111's later rounds.
- The grid, the protocol and the thresholds do not move after a run, and the grid is not narrowed.
- No pinned number of an earlier round moves.
- No change to `.github/workflows/ci.yml`, `scripts/exhaustive-shard.sh` or `scripts/exhaustive-costs.mjs`; the cost
  table changes only by regeneration from this round's dispatch.
- No float anywhere, no dependency, no `unsafe`.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- where the assemblies sit in the ring and how their synapses are chained;
- the delays drawn;
- the epochs' number and windows, within the measures' definitions;
- the release signal;
- whether the grid is widened, for example a size of 128 or a weight between two of the four;
- a reading of the same grid on a network drained as a learning run leaves it;
- where the tests live;
- whether the round writes one ADR or two.

It may not narrow the grid, move a threshold after a run, change a rule of the engine, or reach the standing
directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

## Verification

```bash
cargo check --workspace --all-targets --locked
cargo test --workspace --locked
cargo test --workspace --release --locked
cargo test --workspace --release --locked -- --ignored exhaustive --list
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked
cargo +1.85 check --workspace --all-targets --locked
cargo +1.85 test --workspace --locked
npm ci
npm run spec
git diff main...HEAD > target/pr.diff && cargo mutants --workspace --in-diff target/pr.diff
gh workflow run ci.yml --ref <this round's branch> -f scope=<what ADR-0075 gives this diff>
node scripts/exhaustive-costs.mjs from <the dispatch's downloaded artifacts> --run <its id> > scripts/exhaustive-costs.tsv
```

Every command exits 0. The dispatched run is green and reproduces every pinned number. The grid, the protocol and the
thresholds precede the first run in the history.

## Report

The closing message states:
- the grid, the protocol, the release signal and the thresholds, and when they were committed;
- the arithmetic beside the readings;
- the kick read on the engine;
- the grid table: holds, ignites, lets go, spills, usable;
- the usable region, or what failed in each cell;
- that no weight moved and no pinned number moved;
- the scope ADR-0075 gave the diff, the shards' times and the regenerated cost table;
- what was not done and why;
- the next decision named, not taken.
