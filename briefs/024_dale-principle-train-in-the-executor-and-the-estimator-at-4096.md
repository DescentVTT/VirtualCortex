---
status: proposed
date: 2026-09-14
---

# Brief 024 — Dale's principle in the pair rule (F-36), the spike train inside the executor, and the estimator at 4 096 units

> A self-contained prompt for one round of work. Read `briefs/README.md`, then this file, then
> the files it names. Frozen when executed; the outcome lives in the ADRs that accept it, the
> changelog and the code.

## Mission

Take the three steps the closing report of brief 023 recommended, in its order, as decisions,
code, measurements and tests. **One ADR** conserves a synapse's sign by construction: a
`SynapseBlock`'s polarity is its presynaptic unit's (`FLAG_INHIBITORY`, which the executor
reads at the spike), consolidation moves the eligibility trace into the weight's *magnitude*
within the polarity's half-range, so that an excitatory weight never falls below zero and an
inhibitory one never rises above it whatever the trace holds, and an inhibitory block takes the
symmetric window with a depression per presynaptic spike derived from a stated target rate
(Vogels et al. 2011), selected by the polarity in the same rule; finding F-36 is resolved, and
every exit test that consolidated an inhibitory or a zero-crossing synapse is pinned again from
the run, with the reason stated. **One ADR** gives the executor its own spike train: every
worker's spikes of a tick are merged after the tick in unit order into a bounded ring the
executor owns, bit-identical on every worker count, read between ticks as a sorted slice, so
that the capture rules of [ADR-0048](../docs/adr/0048-episodes-tagged-from-the-train.md) run on
the engine's own run without a fork, a rewarded search tags the coincidence before its reward
from that train, and the composition that searches a clause store, rewards the modulator and
tags the association runs in one call between ticks; the online capture inside the tick is
Implemented for the train, and what would move the store, the affect state and the association
into the image is Specified with its format bump. **One ADR** runs the estimator's measurement
of [ADR-0047](../docs/adr/0047-second-prior-and-the-estimator.md) at 4 096 units on both priors
as the weekly job's `exhaustive` test, with the decision rule written here before the run, and
decides whether the record needs a line for a fine-bin estimate or whether the coarse
estimate's noise stays the bin's at that size too. When the round is done: the whitepaper,
README, `CLAUDE.md`, the reader's guide, the ADR index and the changelog say all of this, the
format stays 13 unless an ADR of this round bumps it, the determinism pin moves only if the
pinned run consolidated a weight across zero (the run says), and this brief is archived with
every check green.

---

## Standing directives

- Every claim is Implemented, Specified, Target or Hypothesis. What a test holds on a network
  of 256, 1 024 or 4 096 units is stated as what it is, with the prior's parameters; what the
  same rules do at Appendix A's scale is a Target with the same generator
  ([ADR-0010](../docs/adr/0010-measured-or-target.md)). No timing figure enters a document from
  a developer machine. A rule is what it does: no "validates", no "autonomous", no
  "reference-scale" for anything below Appendix A's counts.
- The repository wins over the document; a disagreement is a numbered finding in whitepaper
  §11, never a silent edit. An accepted ADR is history: the numbers ADR-0044, ADR-0047 and
  ADR-0048 pinned under the pair rule as it was stay in them with a note; the whitepaper and
  the tests carry the numbers as the tree has them after this round.
- No `f32`/`f64`, in the crates and in the tests; a ratio is Q16.16 in `u32`/`i32`, widened to
  `i64` to multiply; every operation on a state field saturates or wraps by name
  (`clippy::arithmetic_side_effects` is denied everywhere,
  [ADR-0029](../docs/adr/0029-structural-enforcement.md)); a shift amount is bounded a line
  above the shift.
- Every loop ends by construction: a countdown, a range, a scan by `get`, a slice's iterator;
  never by a comparison alone that one operator flip turns into a walk without end.
- 64-byte `#[repr(C, align(64))]` records with compile-time assertions; no heap types, threads
  or `unsafe` in a state crate (`unsafe_code = "forbid"`, ADR-0029). A rule of `cortex-core`
  takes the polarity as an argument; it does not read a unit.
- Every quantity has one owner ([ADR-0016](../docs/adr/0016-thirty-two-crate-architecture.md)):
  the plasticity rule is `cortex-core`'s, the flag that says a unit is inhibitory is the
  unit's, the merge of the workers' spikes and the ring that holds them are the executor's
  ([ADR-0023](../docs/adr/0023-executor.md)), the capture rules stay `cortex-hippocampus`'s and
  the estimator's rules `cortex-homeostasis`'s. No new crate; the crate count stays 32.
- No new record and no field in a record unless an ADR of this round decides one with its
  format bump; the reserved bytes of every record stay zero. The clause store, the affect state
  and the association stay the caller's this round ([ADR-0043](../docs/adr/0043-discovery-path.md)).
- Rule L-3 and §1.5: no word, no string and no language name enters a crate.
- The determinism pin of [ADR-0030](../docs/adr/0030-verification-governance.md) moves only
  with a stated reason; the mutation gate on the changed lines must pass; a new rule carries a
  test over the lattice of `testkit/prop.rs`; every pinned number an arithmetic oracle can
  produce is computed by that oracle before the test that asserts it is written (the
  inhibitory rule's amounts, the merge's order); a number only the engine produces is pinned
  from one run and stated as the engine's.
- The engine never amends its own code ([ADR-0031](../docs/adr/0031-policy-amendment.md)); the
  inhibitory rule's target rate is a constant this round, stated with its derivation, and the
  round that tunes it makes it a `REGISTRY` entry by its own ADR.
- A heavy exit test runs in the weekly job as an ignored test whose name contains
  `exhaustive`; the pull request's gate runs at most what it runs today plus the train's
  tests at 256 units.
- No product name enters a crate. Conventional Commits with a real body; never commit on
  `main`; the required checks keep their names.

## Context

Re-derived on 2026-09-14 against `main` at `c9a87a7`. Line numbers move; the symbols and the
quoted sentences are what to re-derive.

1. **The pair rule and the sign (F-36).** `crates/cortex-core/src/dynamics/synapse.rs`:
   `step_stdp(slot, pre_now_tick, post_last_tick)` puts the nearest-neighbour amounts into the
   slot's eligibility trace (the target's spike after the previous presynaptic one is
   `+A_+ (1 - 2^{-11})^{q-p}`, the target's spike before this one is
   `-A_- (1 - 2^{-11})^{t-q}`; `STDP_A_PLUS_Q1_15` 328, `STDP_A_MINUS_Q1_15` 344,
   `STDP_TAU_SHIFT` 11); `consolidate(slot, modulation_q16)` moves `round(trace × m)` into the
   weight "saturating in $[-1, 1)$" and takes what the weight absorbed out of the trace;
   `step_stdp_all`, `consolidate_all`; `window(amplitude, delta)`; the tests `pair`,
   `one_synapse`, and `a_weight_saturates_at_both_ends_...` (which drives a weight of −32 760
   to `i16::MIN` by depression on a block that has no polarity). Nothing in the crate reads a
   sign: `FLAG_INHIBITORY` (`membrane.rs`: "the unit is inhibitory (its fan-out weights are
   negative); read by the runtime, not by this method") is set by `synthesize`
   (`runtime/cortex-runtime/src/synthesis.rs`) from `Prior::is_inhibitory`, and
   `crates/cortex-connectome/src/prior.rs` draws an inhibitory synapse as the excitatory
   weight times `inhibitory_gain_q4_4` sixteenths, saturating at the width (−32 767 at the
   reference prior's 255). The executor's `phase_fan_out` (`executor.rs`) reads `pre`, the
   presynaptic unit, and calls `block.step_stdp_all(now, posts)` then
   `block.consolidate_all(modulation)` with no polarity. Whitepaper §11 F-36 and the §11.1
   item after it name the two candidate answers. Which pinned runs consolidate: the nights and
   the capture nights of `tests/reference.rs` (`config(units, 2, 0, MODULATION_ONE_Q16)`,
   baseline 1.0); the estimator's runs and the determinism-on-workers test are at baseline 0
   and consolidate nothing, so ADR-0047's table is untouched by a plasticity change. Which
   networks hold inhibitory units: `tests/reference.rs` alone (`grep -rn FLAG_INHIBITORY
   runtime/cortex-runtime/tests`); the differential pin's network (`tests/differential.rs`)
   sets no flag, so the pin moves only if that run depressed an excitatory weight below zero,
   which the run decides. At 256 units the capture night depressed six of unit 0's eleven
   synapses "to the negative rail" (excitatory synapses driven negative by depression, the
   same class as F-36 in the other direction). Other callers of the two rules: the oracle
   blocks of `tests/sleep.rs` and `tests/modulation.rs`, `benches/cortex-bench/benches/hot_path.rs`
   (`step_stdp`).
2. **The workers' spikes and the train.** `executor.rs`: `Worker { spiked: Vec<(u32, u8, u8)>,
   spike_trace: Vec<(u32, u32)>, .. }`; `turn` pushes a spike into `spiked` and, up to
   `Config::trace_capacity`, into `spike_trace`; `phase_turns` stores `spiked.len()` into
   `shared.spikes[id]` for the tally; `phase_fan_out` clears `spiked`; `report()` hands
   `spike_trace` over at `shutdown`. Work stealing (`steal`) moves units between workers, so
   which worker runs a unit is not a function of the run, and a merge that concatenated the
   workers' lists would differ by worker count: the merge must sort by unit within the tick.
   `tick()` runs the three phases and then `rehydrate_pending()` and `tally()` between ticks.
   `branching.rs`: `trace(exec)` consumes the executor and sorts the workers' traces; `fork`
   runs a fresh executor from an image for a train. `episode.rs`: `tag_from_trace`,
   `tag_burst`, `tag_discovery` take a caller's train; `tests/reference.rs::capture_night`
   forks the start image under the same drive and cues to obtain the run's train, then tags
   through the reward. `discovery.rs`: `search(store, len, scratch, affect, budget, out:
   &mut [Discovery]) -> Result<SearchReport, DiscoveryError>` with `reward_total_q16`;
   `capture_night` passes the total to `Executor::reward` once and tags for the first
   discovery. `Config` is spelled out in full in `tests/differential.rs` and
   `tests/contention.rs`: a new field is added there. `Executor::new` allocates every buffer
   once; nothing allocates in the loop (`tests/no_alloc.rs`). [ADR-0048](../docs/adr/0048-episodes-tagged-from-the-train.md)'s
   "Not adopted": "a per-tick merge of every worker's spikes in unit order, bit-identical on
   every worker count, and a cadence for the capture, taken by the round that also runs
   discoveries inside the loop".
3. **The estimator at 4 096.** `tests/reference.rs`: `criticality(p, gains, w, kicks, from,
   loop_windows)`, `GAINS` `[1.75, 2.0, 2.25]`, `readings`, `fine_slopes` at `FINE` 256 and
   `LAGS` `[1, 2, 4, 8, 16]`, `coarse_slopes`; the 1 024-unit `exhaustive` test runs both
   priors at three gains, two windows, sixteen kicks and a twelve-window loop. `drive(units)`
   injects `units / 128` messages per tick; `saturation_ceiling(units)` is one spike per unit
   per bin. `blocks_for(&prior(4096))` is 32 768 blocks (2 MiB). Sizing from a developer
   machine (not admissible, not written anywhere but here): the three 1 024-unit `exhaustive`
   tests take about 140 s in the release profile; a 4 096-unit run costs about four times
   per tick. The weekly job has 300 minutes and runs the whole-tree mutation run after the
   `exhaustive` tests. [ADR-0047](../docs/adr/0047-second-prior-and-the-estimator.md)'s decision:
   "the size at which the fine slope reads the gross ratio is not yet run"; whitepaper §11.1's
   resolution: "at 1 024 units within 0.15 of the gross ratio, so the size that would justify
   a line is the next one, 4 096, run before any line is added". ADR-0047's option 3 is the
   line: "the sums for lags 1 to $K$ at a finer bin, or an exponential average of the coarse
   estimate, format 14, and the controller reading it".
4. **The decision rule for the line, written before the run.** A line of the record is
   justified when, at 4 096 units on both priors at every gain below the ceiling, the lag-one
   slope at the fine bin is within 0.15 (Q16.16 9 830) of the gross causal ratio the oracle
   attributes, and the coarse estimate's two windows differ from each other by more than
   0.15. Then the line is Specified (its fields, its bump to format 14, the controller's read)
   for the round that makes it, and the sweep is the reading it rests on. Otherwise no line is
   added, the reading is recorded as the third size, and the causal count inside the loop
   stays the Specified alternative. Either way the sweep's numbers are pinned in the
   `exhaustive` test and stated in the ADR's table beside the two smaller sizes.
5. **The image and the store.** `image.rs` writes five sections (neuron, synapse, modulator,
   homeostasis, hippocampus; `tests/modulation.rs` asserts `section_count` 5) and refuses a
   non-quiescent engine; the clause store is a caller's `TermNode` arena, the affect state a
   caller's `InteroceptiveState`, the association a caller's `Association`
   ([ADR-0043](../docs/adr/0043-discovery-path.md), ADR-0048). Moving the three into the image is
   three sections and format 14, with the loader's checks and every fixed offset of
   `tests/image.rs` moved: its own round, not this one. A capture on a cadence inside `tick()`
   needs the store inside the executor for the reward that triggers it; this round's trigger
   is the caller's call between ticks, on the executor's own train.
6. **Where the round's numbers go.** The next ADR is 0049 (`ls docs/adr`); F-36 gains a
   Resolved disposition in whitepaper §11 and the §11.1 item after it is closed; the
   estimator's resolved item in §11.1 gains the 4 096-unit reading; H-9's disposition gains
   the re-pinned numbers where they moved; the whitepaper moves 4.11.0 → 4.12.0; reference 78
   (Vogels et al. 2011) exists in Appendix D; the property kit is `testkit/prop.rs`; the
   mutation gate's exclusions are in `.cargo/mutants.toml`; the determinism pin is
   `PINNED_ARENA_HASH` in `tests/differential.rs`; the weekly job runs `cargo test --workspace
   --release --locked -- --ignored exhaustive`.
7. **What is not in the tree that a reader might assume.** No rule reads a unit's polarity
   today; no worker's spikes are visible to another before shutdown; no test runs above 1 024
   units; no section of the image holds a term.

<!-- @assert-absence target="crates/cortex-core/src/dynamics/synapse.rs" symbol="Polarity" word="true" reason="brief 024 precondition: no rule of cortex-core reads a polarity yet" -->
<!-- @assert-absence target="runtime/cortex-runtime/src/executor.rs" symbol="train_capacity" reason="brief 024 precondition: the executor holds no train of its own yet" -->
<!-- @assert-absence target="runtime/cortex-runtime/tests/reference.rs" symbol="4096" reason="brief 024 precondition: no exit test runs at 4 096 units yet" -->

## Deliverables

- [ ] **The polarity ADR (the next free number)** (`docs/adr/0049-*.md`, `depends-on:
  ADR-0032`, ADR-0022 and ADR-0044 named). In `cortex-core` (`synapse.rs`): `Polarity {
  Excitatory, Inhibitory }` with `Polarity::of_flags(flags: u8)` from `FLAG_INHIBITORY`;
  `step_stdp(slot, pre_now_tick, post_last_tick, polarity)` and `step_stdp_all(now_tick,
  post_last_ticks, polarity)`: for an excitatory block the rule of ADR-0022 as it is; for an
  inhibitory block the symmetric rule of Vogels et al. 2011 on the magnitude: the trace gains
  `ISTDP_A_Q1_15 (1 - 2^{-11})^{|Δt|}` for the target's spike after the previous presynaptic
  one and for the target's spike before this one, and loses `ISTDP_ALPHA_Q1_15` at every
  presynaptic spike, with the constants stated (`ISTDP_A_Q1_15` equal to $A_+$;
  `ISTDP_TARGET_PERIOD_TICKS` 20 000, a target rate of 5 Hz at the 10 µs tick;
  `ISTDP_ALPHA_Q1_15` = `2 A τ / period`, tied by a `const` assertion). `consolidate(slot,
  modulation_q16, polarity)` and `consolidate_all(modulation_q16, polarity)`: the trace is the
  magnitude's change; the magnitude (the weight for an excitatory block, its negation for an
  inhibitory one, a weight on the wrong side of zero read as a magnitude of zero) gains
  `round(trace × m)` clamped to `[0, i16::MAX]`, the weight is the magnitude with the
  polarity's sign, and the trace keeps what the rail did not absorb, as today. Tests, with
  every amount from an oracle computed before the test: an excitatory weight at zero
  depressed stays at zero and keeps the depression pending; an excitatory weight of one
  depressed by more ends at zero; an inhibitory weight of −1 depressed ends at zero and one
  at −32 767 potentiated stays there with the rest pending; a weight on the wrong side of
  its polarity is brought to zero by the first consolidation with the trace untouched; the
  inhibitory amounts at the same deltas as the excitatory test's table, both orders
  potentiating and α at every spike; the mirror property over the lattice (an excitatory and
  an inhibitory block fed the same history: the inhibitory magnitude moves by the symmetric
  amounts and never crosses zero); the sum of weight and trace conserved in magnitude;
  `two_blocks_given_the_same_history_stay_identical` for both polarities. The executor passes
  `Polarity::of_flags(pre.flags)`; the oracle blocks of `tests/sleep.rs` and
  `tests/modulation.rs` and the benchmark pass `Polarity::Excitatory`. The four night tests of
  `tests/reference.rs` are pinned again from the run, each number that moved stated in the
  ADR with the one that stood, and the 1 024-unit capture night's own pattern holds its
  inhibitory synapses at or below zero after the night. The ADR states why the magnitude
  (potentiation strengthens a synapse whatever its sign; Dale's principle held by the rule,
  not by review), why the symmetric rule for inhibition (the tree's open item named it;
  fifteen years of use in the network simulators; a rate target as its documented failure
  mode), what the target rate is and that it is a constant until a `REGISTRY` entry, and what
  it does to the reference network's inhibitory weights over the waking bins of the exit
  tests. F-36 Resolved.
- [ ] **The train ADR (the number after it)** (`docs/adr/0050-*.md`, `depends-on: ADR-0048`,
  ADR-0023 and ADR-0036 named). In the executor: `Config::train_capacity` (the spikes the
  executor's ring keeps; 0 keeps none and adds nothing to the tick); a shared buffer of one
  slot per unit with an atomic cursor that every worker appends its spikes to in `turn` (a
  unit fires at most once per tick, so the cursor stays below the unit count); after the
  tick's last barrier the coordinator takes the tick's spikes, sorts them by unit and appends
  `(tick, unit)` to a ring of `train_capacity`, letting the oldest go when it is full and
  counting them; `Executor::train(&mut self) -> &[(u32, u32)]` (the ring made contiguous, in
  tick order and unit order within a tick, between ticks) and `Executor::train_overwritten()`.
  In `episode.rs`: `tag_recent(exec, from, window, priority)`, `tag_burst_recent(exec,
  ticks_back, coincidence, priority)` and `tag_discovery_recent(exec, window, coincidence,
  discovery, priority)` over the executor's own train (the existing three over a caller's
  train stay); `discover(exec, store, len, scratch, affect, budget, window, coincidence,
  priority, discoveries, associations) -> Result<(SearchReport, Option<(u32, Burst)>),
  DiscoverError>`: the search of ADR-0045, the committed rewards' total into the modulator
  when positive, the densest coincidence of the window before now tagged once for the
  rewarded search, and one association per commit to that episode written to `associations`
  (`OutFull` when there is no room, before the search). Tests: the ring's order at one and
  four workers equal, and equal to `trace(exec)` of the same run, in
  `tests/reference.rs`'s determinism test; the ring's overwrite (a capacity of four and six
  spikes keep the last four, two counted), a capacity of zero (an empty train, nothing
  counted, no allocation in the loop still holds in `tests/no_alloc.rs`); `capture_night`
  reads the executor's train and tags through `discover`, with the same association as the
  fork gave, and no fork for the train remains in that helper. The ADR states the merge's
  cost (one atomic append per spike, one sort of the tick's spikes on the coordinator), why
  the ring is the executor's and not a worker's, what is Implemented (the online capture on
  the engine's own train, triggered between ticks) and what is Specified (the store, the
  affect state and the association in the image: three sections and format 14, taken by the
  round that gives the image a term arena, after which a cadence inside `tick()` can run the
  search on a rewarded moment).
- [ ] **The 4 096-unit ADR (the number after that)** (`docs/adr/0051-*.md`, `depends-on:
  ADR-0047`, ADR-0044 named). In `tests/reference.rs`: an `exhaustive` test at 4 096 units on
  both priors, three gains, two windows, thirty-two kicks and a twelve-window loop, the
  readings pinned from the run as at 1 024. The ADR's table holds, per prior and gain, the
  coarse estimate per window, the spikes, the gross and net causal ratios and the fine-bin
  slopes, beside the two smaller sizes' rows of ADR-0047; the decision by the Context's rule
  (item 4), stated as the rule was written before the run; what a line would hold if it is
  justified, Specified with its bump; the ceiling's role at this size; and what the loop did
  in twelve windows.
- [ ] **F-36 Resolved** in whitepaper §11 with the disposition (the polarity from the unit,
  the magnitude within the half-range, the symmetric rule, the re-pinned nights); the §11.1
  item after it closed; the estimator's resolved item extended with the 4 096-unit reading and
  the line's decision; H-9's disposition carrying the numbers as the tree has them after the
  round, and H-11's where they moved.
- [ ] **The documents.** Whitepaper 4.12.0: the executive summary's sentence on what exists
  (the online capture on the executor's train; a polarity read by the rule); §1.6 rows
  (`cortex-core`: the polarity; the date); §5.2.1 (Public API: `Polarity`, the two rules'
  signatures; the fan-out and STDP paragraph and the three-factor paragraph: the magnitude,
  the half-range, the inhibitory rule; a directive `Polarity`); §5.2.15 (the online capture
  Implemented on the executor's train; a directive `fn tag_discovery_recent`); §5.2.16 (the
  4 096-unit row); §6.1 step 6 (the polarity); §6.6 and §6.10 (the executor's train, the
  rewarded search's tag); §8.8 rows (STDP: the polarity and the inhibitory rule; complementary
  learning: the train; criticality: the third size); §9 three rows; §11 F-36; §11.1;
  Appendix C (M5's clause); the glossary (Polarity, Train). README (the Implemented rows),
  `CLAUDE.md` (the sentence on what exists and what does not), `docs/zh-TW/README.md` (§6 and
  §11 rows), `docs/adr/README.md` (three rows), `CHANGELOG.md` (one entry under Unreleased in
  the shape of brief 023's).
- [ ] **Not adopted, with the reason in the ADR that is closest:** a per-synapse sign bit in
  the block (the polarity ADR: the polarity is the unit's, and a bit would be a second owner
  of it); the target rate as a `REGISTRY` entry (the same ADR: its own ADR when a round tunes
  it); the store, the affect state and the association in the image, and a capture on a
  cadence inside `tick()` (the train ADR: format 14 and three sections, the round that gives
  the image a term arena); a night or a capture at 4 096 units (the 4 096-unit ADR: the
  estimator's question is the one that size answers; H-9's transfer stays open with its
  protocol); a change to `estimate_branching_ratio`, `regulate`, the bin, the window or the
  ceiling on the strength of the sweep (the same ADR: a line is Specified, never made, this
  round).
- [ ] **This brief archived** under `briefs/archive/` with the frozen banner, every box
  dispositioned, the precondition directives removed and the links rebased.

## Not empowered

- No image section, no format bump and no field in any record unless the ADR that decides it
  is written first with the bump; the reserved bytes stay zero; the ledger's record is
  untouched.
- No new crate; no dependency in a state crate; no `unsafe` outside the runtime's arena.
- No float anywhere, including the tests and the oracle.
- No change to the excitatory rule's amounts, the window, the eligibility time constant, the
  modulator, `estimate_branching_ratio`, `regulate`, the bin, the window, the ceiling, the
  ripple, the replay drive or the sleep constants: the polarity ADR adds the inhibitory rule
  and the half-range; the sweep measures.
- No renaming of the CI jobs the ruleset requires; no move of the determinism pin without
  the run's reason.
- No test in the pull request's gate above the nights at 256 units the gate holds today plus
  the train's tests; the 4 096-unit sweep is the weekly job's.
- No claim that a rule at 4 096 units says what the same rule does at Appendix A's scale.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in
the ADR that owns it: the inhibitory rule's form (symmetric, or asymmetric on the magnitude)
and its constants, the treatment of a weight on the wrong side of its polarity, the shape of
the executor's ring and how a caller reads it, the merge's place in the tick, the name and
form of the discovery composition and what it returns, the kicks and windows of the
4 096-unit sweep and the threshold of the decision rule (stated before the run, whichever it
is), and which numbers of the re-pinned nights the ADR restates. It may not reach the
standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

## Verification

```bash
cargo check --workspace --all-targets --locked
cargo test --workspace --locked
cargo test --workspace --release --locked
cargo test -p cortex-runtime --release --locked -- --ignored exhaustive
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
with 95 spikes unless the pinned run consolidated a weight across zero, in which case the new
value is taken from the run and the polarity ADR says so. The three precondition directives
above are gone with the archived brief.

## Report

The closing message states: the polarity rule and the inhibitory rule with their constants
and the oracle that produced their amounts; which pinned numbers of the nights moved, by how
much, and which stood, at 256 and 1 024 units, and what the 1 024-unit own pattern's
inhibitory synapses do now after a night; whether the determinism pin moved and why; the
executor's train, its cost, its worker-count independence and what `capture_night` looks like
without the fork; the 4 096-unit readings on both priors beside the two smaller sizes, the
decision rule as written before the run and what it decided about the line; what the mutation
gate found on the changed lines and how each survivor was answered; what was not done (the
image sections, the cadence inside the tick, the registry entry, a night at 4 096) and why;
and what the re-examination after the round recommends next.
