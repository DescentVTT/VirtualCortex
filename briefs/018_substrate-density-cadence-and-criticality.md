---
status: proposed
date: 2026-09-12
---

# Brief 018 — The substrate re-examined: synaptic density, a multirate cadence, and closed-loop criticality control

> A self-contained prompt for one round of work. Read `briefs/README.md`, then this file, then
> the files it names. Frozen when executed; the outcome lives in the ADRs that accept it, the
> changelog and the code.

## Mission

Answer an architectural proposal ("Foundational Systems Substrate, Multirate Synchronization &
Dynamic Non-Linear Equilibrium", received 2026-09-12) with decisions, code and measurements
instead of a document that drifts. The proposal names three frontiers: an extreme-density
synaptic layout (eight-bit logarithmic weights, eight synapses per block, a far-memory synapse
tier), a multirate execution engine (hierarchical timing wheels, epoch dividers, conservative
parallel discrete-event simulation with axonal lookahead), and a multi-timescale stability
framework (a closed loop driving the branching ratio to 1, multiplicative synaptic scaling
without an $O(N)$ sweep, columnar annealing, an immune quench). Three of its stated baseline
findings are not in the tree (the Context below says which), so the round begins by
re-deriving them. When the round is done: **one ADR** evaluates the density proposals against
the block's byte accounting, keeps the four-synapse block, and names the density levers with
the measurement that admits each; **one ADR** makes multirate stepping a *cadence* (a
power-of-two period and a phase, a mask on the tick, as the wheel's slot is), adds the
population spike tally the executor lacked and a benchmark of the barrier cost per tick
against the worker count, and leaves conservative lookahead as an open question decided by
that measurement; **one ADR** implements criticality control as an integer rule in
`cortex-homeostasis`: the branching ratio estimated by lag-one regression over bins of
population activity (Wilting and Priesemann 2018), a global synaptic gain as the actuator,
multiplicative and bounded, moved once per window by a bounded step, composed by the executor
and persisted in the image, with the reference dynamics unchanged at the default step of zero
(the modulator's precedent, [ADR-0032](../docs/adr/0032-three-factor-plasticity.md)). Per-unit
multiplicative scaling (Turrigiano) is written down as Specified with its closed-form rule and
the bytes it will take; the annealing and the quench are not adopted, with the reasons. The
whitepaper, README, `CLAUDE.md`, the ADR index and the changelog say all of this, two document
defects found on the way are corrected, and this brief is archived with every check green.

## Standing directives

- Every claim is Implemented, Specified, Target or Hypothesis. A number is Measured only from an
  admissible run ([ADR-0010](../docs/adr/0010-measured-or-target.md)); the proposal's "severe
  barrier skew" and "4.29 GB halved" are not measurements and MUST NOT be written as findings.
- The repository wins over the document; a disagreement is a numbered finding in whitepaper
  §11, never a silent edit.
- No `f32`/`f64`; Q16.16 in `i32`/`u32`, widened to `i64` (or `i128` where a product needs it)
  to multiply; every operation on a state field saturates or wraps by name
  (`clippy::arithmetic_side_effects` is denied everywhere, [ADR-0029](../docs/adr/0029-structural-enforcement.md)).
- 64-byte `#[repr(C, align(64))]` records with compile-time assertions; no heap types, threads
  or `unsafe` in a state crate; the runtime's arena access is the one `unsafe`
  ([ADR-0023](../docs/adr/0023-executor.md)).
- A change to a record bumps `CortexFileHeader::FORMAT_VERSION`, updates its §5.2 table and
  gets a changelog entry (rule L-6).
- A run is `(image, seed, input trace)` (§8.3): anything that changes what a run does is in
  the image, not in a configuration or a constant a caller can vary.
- The determinism pin of [ADR-0030](../docs/adr/0030-verification-governance.md) moves only
  with a stated reason; the mutation gate on the changed lines must pass.
- Conventional Commits with a real body; never commit on `main`; the required checks keep
  their names.
- Latest ≠ Newest (§2.1): a technique is admissible with a stable specification, years of
  third-party use and documented failure modes. Lag-one regression for the branching ratio
  (Wilting and Priesemann 2018) and min-delay lookahead (Chandy and Misra 1979; Morrison et
  al. 2005, NEST) pass; a memory tier the OS does not yet page transparently does not.

## Context

Re-derived on 2026-09-12 against `main` at `3595208`.

- `crates/cortex-core/src/dynamics/neuron.rs`: `SynapseBlock` is 64 bytes for four synapses:
  `target_neuron_ids` `[0..16)`, `weights_q1_15` `[16..24)`, `delays_ticks` `[24..32)`, `chain`
  `[32..36)`, `last_spike_tick` `[36..40)`, `last_release_q16` `[40..56)`, `eligibility_q1_15`
  `[56..64)`. Per synapse: 4 bytes of target, 2 of weight, 2 of delay, 4 of stored release, 2 of
  trace, and 2 of the block's chain and stamp. **The weight is 2 of the 16 bytes**, so an
  eight-bit weight saves one sixteenth of the arena, not half; eight synapses per block would
  need the delay at 8 bits (a 2.55 ms horizon, below every coarse-ring delay), no stored release
  (the delivering worker has neither the source's STP factors nor its weight at the spike,
  [ADR-0022](../docs/adr/0022-synapse-fan-out-and-stdp.md)) and an 8-bit trace, which
  [ADR-0032](../docs/adr/0032-three-factor-plasticity.md) rejected (0.0100 is 1.3 LSB of Q0.7).
- `crates/cortex-core/src/dynamics/synapse.rs`: `STDP_A_PLUS_Q1_15` 328, `STDP_A_MINUS_Q1_15`
  344 (the proposal's 1.05 ratio is right); the chain word's 28 bits, `CHAIN_MASK`; the STP
  factors are the *unit's* (`stp_u_rel`, `stp_r_ves` in `neuron.rs`, `step_stp` in
  `plasticity.rs`), not per synapse, and the proposal's "membrane.rs" holds integration, not STP.
- `runtime/cortex-runtime/src/barrier.rs`: a sense-reversing spin barrier; `executor.rs`
  `tick()` waits at it four times per tick. The loop runs units and blocks and nothing else:
  no homeostasis, thalamus, immune or metabolic rule is composed, so no "slow node
  synchronising at every tick" exists in the tree. `docs/benchmarks/README.md` says what is not
  measured: "the loop across several workers (the barriers' cost with contention)".
  [ADR-0023](../docs/adr/0023-executor.md)'s consequences already name the four waits as "the
  dominant cost of an idle tick and a T-3 measurement subject".
- `runtime/cortex-runtime/src/executor.rs`: units are not owned by workers; a work-stealing
  deque moves a unit to whichever worker pops it, so a region a worker could step ahead of
  the others does not exist. `Worker::spiked` holds the units that fired this tick; nothing
  sums its length across workers. `Config` has ten fields; `tests/differential.rs` and
  `tests/contention.rs` spell every one out in a literal.
- `crates/cortex-homeostasis/src/lib.rs`: `HomeostaticDrivePool` has `energy_level` `[0..4)`,
  `sensory_fatigue` `[4..8)`, `curiosity_drive` `[8..12)` (kept by ADR-0016's table),
  `thermal_stress` `[12..16)`, `circadian_phase` `[16..20)` (a 16-bit counter in a `u32`),
  `sleep_mode_active` `[20..24)` (0 or 1 in a `u32`), `branching_ratio_q16` `[24..28)`,
  `target_threshold_bias` `[28..32)` (read by nothing), `_reserved: [u8; 32]`.
  `update_branching_ratio(descendants, ancestors)` divides; nothing tallies spikes into it
  (whitepaper §5.2.16: "the spike tally that feeds $\sigma$: Specified"). The crate's own
  comment says hardware vitals are `cortex-autonomic`'s, whose record holds
  `core_temperature_milli_c` and `thermal_limit_milli_c`: `thermal_stress` has two owners.
- `docs/WHITEPAPER.md` §8.8: "Self-organised criticality | `cortex-homeostasis` | Rescale
  weights by $1 - \kappa(\sigma - 1)$ during sleep | Specified", and the equation
  $W_{ij}(t+1) = W_{ij}(t)[1 - \kappa(\sigma - 1)]$ in the reference block: multiplicative on
  every weight, an $O(S)$ sweep, which is why it was confined to sleep. A common factor on every
  weight arriving at a unit is a gain on that unit's input sums, applied once per turn.
- `docs/WHITEPAPER.md` §10.2 and Appendix A: T-2 is 20 GB and the plan totals 15.8 GB of Tier 1
  with the block arena at 4.29 GB (row 3, the most a token can name, finding F-23); nothing
  in the tree measures the arena as the binding constraint.
- `docs/WHITEPAPER.md` Appendix D: reference 31 is two entries (Izhikevich 2007 and
  Rosenthal 1986), a leftover of the numbering fix in PR #44; the document-control table
  says "Version | 4.4.0" beside front matter that says 4.5.0.
- `crates/cortex-core/src/dispatch/wheel.rs`: `HORIZON_TICKS` 2 560, `TICK_NS` 10 000; the
  fine and coarse rings are powers of two "so that slot selection is a mask" and "the wrap is
  exact". A cadence with a power-of-two period has the same two properties on the `u64` clock.
- Preconditions, checked by `spec-guard` until this brief is archived:

<!-- @assert-absence target="crates/cortex-core" symbol="Cadence" word="true" reason="brief 018 precondition: no cadence type exists yet" -->

<!-- @assert-absence target="crates/cortex-homeostasis" symbol="synaptic_gain_q16" reason="brief 018 precondition: the homeostasis record has no gain yet" -->

<!-- @assert-count target="crates/cortex-homeostasis" symbol="thermal_stress" min="1" reason="brief 018 precondition: the pool still holds a hardware vital that cortex-autonomic owns" -->

## Deliverables

1. [ ] **Density** (one ADR, no code). The four-synapse block stands. The ADR carries the
   byte accounting of the Context, rejects eight-bit logarithmic weights (a pairing of 1 % is
   below one level of a 256-level logarithmic range; [ADR-0012](../docs/adr/0012-synaptic-weight-q1-15.md)
   rejected Q8.8 for the same resolution reason, and stochastic rounding is a change to the
   numeric model), eight synapses per block (the delay, the release and the trace above) and
   a far-memory synapse tier built into the engine (a block is touched only when its unit
   fires, so cold blocks are already cold pages; placement is the OS's, and a hint at load is
   the `mmap` round's), and names the density levers in order with the measurement that
   admits each: the stored release as the block's `(u, r)` at the spike (2 bytes for 16,
   widening [ADR-0022](../docs/adr/0022-synapse-fan-out-and-stdp.md)'s overwrite defect); a
   column-relative target id; the token width (finding F-23). Whitepaper §11.1 gains the
   hypothesis that the block arena is the binding constraint, with T-2 on a reference image
   as the test.
2. [ ] **Cadence** (`cortex-core`, part of the second ADR). `Cadence::new(period_shift, phase)
   -> Option<Cadence>` (refused for a shift of 64 or more and for a phase at or beyond the
   period), `is_due(tick: u64) -> bool` as `tick & (period − 1) == phase`, `period()`,
   `phase()`. A power-of-two period so that the wrap of the clock is exact and the test is a
   mask. Tests at both bounds, across the wrap, and the property walk of `testkit/prop.rs`
   (every tick is due on exactly one phase of a period).
3. [ ] **The tally and the barrier benchmark** (`cortex-runtime`, `cortex-bench`; the second
   ADR). Every worker publishes the number of units that fired in its turns phase; the
   coordinator sums them between ticks (a sum of integers, so the count is the same on any
   worker count). `benches/cortex-bench` gains `executor/idle_tick` on one, two and four
   workers (an executor of units at rest; one `tick` per iteration: the four barrier waits and
   nothing else), and the benchmarks README a row. Whitepaper §11.1 gains the open question of
   conservative lookahead (synchronising per minimum delay instead of per tick, which needs
   units owned by workers) with the decision rule: the idle tick at the reference platform's
   worker count, from an admissible run, against the tick's budget.
4. [ ] **The estimator and the controller** (`cortex-homeostasis`, the third ADR).
   `HomeostaticDrivePool` becomes: `energy_level` `[0..4)`, `sensory_fatigue` `[4..8)`,
   `curiosity_drive` `[8..12)`, `bin_activity: u32` `[12..16)` (spikes counted in the open bin;
   `thermal_stress` removed, finding), `circadian_phase: u16` `[16..18)`, `sleep_mode_active:
   u8` `[18..19)`, `window_bins: u8` `[19..20)`, `control_step_q0_16: u16` `[20..22)` ($\kappa$,
   Q0.16), `_reserved: u16` `[22..24)`, `branching_ratio_q16` `[24..28)`, `synaptic_gain_q16:
   u32` `[28..32)` (`target_threshold_bias` removed: the gain is the actuator), `sum_prev: u64`
   `[32..40)`, `sum_prev_sq: u64` `[40..48)`, `sum_pair: u64` `[48..56)`, `last_activity: u32`
   `[56..60)`, `first_activity: u32` `[60..64)`. Rules: `count_activity(spikes)` (saturating at
   `ACTIVITY_COUNT_MAX`, $2^{24} - 1$, so the window's sums fit `u64` and their products
   `i64`); `close_bin()` (the closed bin pairs with the previous one into the three sums, the
   first bin of a window is remembered, the bin count advances); `estimate_branching_ratio()
   -> Option<u32>`: the lag-one least-squares slope with intercept over the window's pairs,
   $\hat\sigma = (n \sum ab - \sum a \sum b) / (n \sum a^2 - (\sum a)^2)$ with
   $\sum b = \sum a - a_0 + a_n$, in Q16.16 clamped to $[0, 16]$; a window without a spike is
   $\hat\sigma = 0$ (no descendants), a window whose activity never varied is no estimate;
   `regulate() -> Option<u32>`: with an estimate, $\sigma$ is stored and the gain moves by
   $g \leftarrow g\,(1 - \kappa\,\operatorname{clamp}(\hat\sigma - 1, -1, 1))$, rounded to
   nearest, clamped to $[0.25, 4.0]$; with or without one the window resets; `is_well_formed()`
   (the gain in its bounds, $\kappa \le 0.5$, the bins below the window, the counts at or
   below the cap, the sums within what the cap allows, the reserved bytes zero, the sleep flag
   0 or 1); `encode`/`decode` of the 64 bytes; `new()` at gain 1.0, equal to `Default`.
   Constants: `ACTIVITY_BIN_SHIFT` (12: 4 096 ticks, above the wheel's horizon, so every
   direct descendant of a bin's spikes lands in that bin or the next; below the horizon the
   lag-one slope would read a delayed network as sub-critical and the controller would raise
   the gain without bound), `ACTIVITY_WINDOW_SHIFT` (5: 32 bins), `GAIN_ONE_Q16`,
   `GAIN_MIN_Q16`, `GAIN_MAX_Q16`, `CONTROL_STEP_MAX_Q0_16`, `SIGMA_MAX_Q16`.
   `update_branching_ratio` and `update_circadian_tick` stay (the widths change, not the
   rules). Tests: a synthetic AR(1) series recovers its slope within rounding, the silent and
   the constant windows, both rails of the gain, $\kappa = 0$ leaves the gain bit for bit, the
   bytes, and the property walk. F-28 (the two fields with two owners or none).
5. [ ] **The composition** (`cortex-runtime`, the third ADR). `Config::control_step_q0_16`
   (default 0; refused above `CONTROL_STEP_MAX_Q0_16`); one `HomeostaticDrivePool` per engine,
   `homeostasis()` between ticks; the gain published to the workers before every tick and
   applied by the turn holder to the basal and the apical sums before `integrate` (rounded
   to nearest, exact at 1.0); the tally after every tick into `count_activity`; the bin closed
   when `Cadence::new(ACTIVITY_BIN_SHIFT, 0)` is due at the tick count, the window regulated
   when the window's cadence is; a compile-time assertion that the bin is at least the wheel's
   horizon. Section kind 43 (`SECTION_HOMEOSTASIS`, one 64-byte record, the pool's bytes)
   always written and required (`MissingSection`), refused when its count is not one
   (`Directory`), when the record is not well formed (a new `ImageError` variant) and when
   $\kappa$ exceeds its bound (`Config`); the image's $\kappa$ and gain outrank the
   configuration's. `FORMAT_VERSION` 12, with the reason in its list. The pin's `Config`
   literal gains the field, and the pin holds: with $\kappa = 0$ the gain is 1.0 and the
   turn's product is exact.
6. [ ] **The exit test** (`runtime/cortex-runtime/tests/`). With $\kappa > 0$: a network wired
   super-critical settles with $\hat\sigma$ within a stated distance of 1 and the gain below
   1, its activity bounded; a network wired sub-critical under a steady drive ends with the
   gain above 1 and activity that persists; a silent engine's gain climbs to the ceiling and
   stops there exactly; with $\kappa = 0$ nothing moves and every pinned value holds; the same
   run is bit-identical on one and four workers (the tally is a sum); the homeostasis state
   written mid-window round-trips through the image and a loaded engine continues alike; the
   loader's refusals, each on its own.
7. [ ] **Specified and not adopted.** Whitepaper §8.8 gains a Specified row for per-unit
   multiplicative scaling (Turrigiano): a gain per unit in `[52..54)` of
   `DendriticSuperNeuron`, lowered by a step at each spike and relaxing toward its ceiling
   between spikes by $(1 - 2^{-k})^{\Delta t}$ at the next event (the STP pattern of
   [ADR-0019](../docs/adr/0019-short-term-plasticity.md)), so a silent unit's gain rises and
   a busy unit's falls with no sweep. The third ADR says why columnar annealing (a seeded
   jitter changes the run's definition and nothing in the tree exhibits the deadlock it would
   escape) and an immune quench (the branching ratio has one owner; a second actuator in
   `cortex-immune` gives it two, against [ADR-0016](../docs/adr/0016-thirty-two-crate-architecture.md)'s
   rule) are not adopted.
8. [ ] **Documents.** Whitepaper §1.6 (the two Logic cells), §5.2.1 (the cadence and the
   gain in the turn), §5.2.2 (format 12, section 43), §5.2.16 (the table, the API, the status,
   the rule with executable assertions), §6.1 and R-12 (the tally), §8.3 (the tally is a sum),
   §8.4 (the cadence row), §8.7 (the section), §8.8 (the criticality rows and the new
   Specified row; the equation block), §9, §10.2 (the benchmark), §11 (F-28), §11.1 (the two
   new questions), Appendix A (row 43), Appendix C (M5), Appendix D (Wilting and Priesemann
   2018; Turrigiano 2008; Chandy and Misra 1979; Morrison et al. 2005; reference 31 made
   unique) and the document-control version cell; `docs/benchmarks/README.md`; the ADR
   index; `README.md`, `CLAUDE.md`; `CHANGELOG.md`; this brief archived with every box
   dispositioned.

## Not empowered

- To change the tick, the wheel's geometry, or the phase structure of
  [ADR-0023](../docs/adr/0023-executor.md): no new barrier, no static assignment of units to
  workers, no phase pipelining. Conservative lookahead is an open question this round measures
  for, not one it decides.
- To change the weight format ([ADR-0012](../docs/adr/0012-synaptic-weight-q1-15.md)) or the
  block's layout, or to add a synapse tier: the density ADR decides what is *not* done and
  what would admit it.
- To let the controller touch weights (an $O(S)$ sweep in the loop), or to put $\kappa$, the
  gain or the bin geometry in the amendment registry of
  [ADR-0031](../docs/adr/0031-policy-amendment.md): they change what the engine does.
- To keep the pool out of the image or to write its section only sometimes: the gain changes
  results, so the section is always written and required (the review finding of PR #44).
- To add a state crate, a dependency, `#[allow]`, a feature flag or a nightly attribute; to
  add `unsafe` outside `arena.rs`.
- To move a pin without the ADR's reason and the test that shows the boundary.
- To decide the `mmap` path, core pinning, the broker or the lexicon.

## Architectural empowerment

- The executing round may choose the estimator's exact form (the slope through the origin,
  a multistep regression) and the bin and window shifts, the gain's bounds and the step's
  bound, with the reason in the ADR and a test at each boundary; the bin MUST stay at or above
  the wheel's horizon.
- It may choose a threshold bias as the actuator instead of a gain if it shows a stated input
  on which the gain fails and the bias does not; the whitepaper's equation is multiplicative,
  which is why the gain is the default.
- It may repurpose different bytes of the pool if it shows a field's owner differently, and
  may keep `thermal_stress` if it finds a rule that reads it; every removed field is a finding.
- It may place the cadence type in the runtime instead of `cortex-core` if it names why a
  state crate must not carry it.
- It may leave the benchmark out if `criterion` cannot run a multi-worker executor in smoke
  mode on the CI runner, and must then say so and name the measurement's next home.

## Verification

```bash
cargo check --workspace --all-targets --locked
cargo test --workspace --locked
cargo test --workspace --release --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked
cargo bench -p cortex-bench --bench hot_path --locked -- --test
cargo +1.85 check --workspace --all-targets --locked
cargo +1.85 test --workspace --locked
npm run spec
git diff main...HEAD > target/pr.diff && cargo mutants --workspace --in-diff target/pr.diff
```

Every command exits 0; the last one reports no survivor in CI (a local Windows run marks
mutants whose build failed as unviable and is not the gate's truth). `npm run spec:deps` still
reports no dependency in a state crate. The determinism pin holds at `0x1724f3486c1d674e`
with 95 spikes, or the ADR says why it moved. The three precondition directives above are gone
with the archived brief.

## Report

The closing message states: which of the proposal's premises the tree contradicted and where
each correction went; the byte accounting and the density levers; the cadence, the tally and
the benchmark, with the developer-machine figure for the idle tick marked not admissible; the
record's new layout, the estimator, the controller's constants and its bounds; the exit test's
outcome with the numbers it reached; the format version; whether the pin moved and why; what
the mutation gate found on the changed lines and how each survivor was answered; what was not
done (per-unit scaling, the lookahead, the annealing, the quench, the other three fields of
the pool) and why.
