---
status: archived
date: 2026-09-13
---

> **Executed 2026-09-13 in pull request #54.** Writes ADR-0044 (the reference network) and
> ADR-0045 (the executive clause search); records finding F-33; dispositions H-8 and H-9 at
> 256 and 1 024 units and H-11's symbolic half; image format 13 and the determinism pin
> untouched. Every deliverable is done; notes under the boxes say where the tree departs from
> the text (the gate's form of the H-8 test is one gain, one window and four kicks; the
> proof search selects the leftmost literal; `resolve_literal` was added). The report is in
> the pull request and in `CHANGELOG.md`. The body below describes the tree before execution
> and is not maintained; its relative links gained one `../`.

# Brief 022 — The reference network and its measurements: a seeded anatomical prior, the runtime's synthesis, drive and causal branching-ratio oracle, the executive clause search with a bounded proof search as its readback; hypotheses H-8, H-9 and the symbolic half of H-11 decided at a stated scale

> A self-contained prompt for one round of work. Read `briefs/README.md`, then this file, then
> the files it names. Frozen when executed; the outcome lives in the ADRs that accept it, the
> changelog and the code.

## Mission

Answer an architectural proposal ("Reference-Scale Connectome Synthesis, Autonomous Clause
Search, and Empirical Validation of Cognitive Hypotheses (H-8, H-9, H-11)", received
2026-09-13) with decisions, code, measurements and tests instead of a document that drifts.
The proposal names three pillars: a procedural connectome synthesizer and a traced run that
decide H-8 (the branching-ratio estimator's regime) and H-9 (replay and pattern completion
after a night); an executive clause search that feeds the inductive operators of
[ADR-0041](../../docs/adr/0041-induction-on-the-term-arena.md) and decides H-11 (whether a
rewarded invention is read back); and a runtime lexicon with a cortico-striatal gating
circuit. Five of its premises are not in the tree (the Context says which), so the round
begins by re-deriving them. When the round is done: **one ADR** puts the anatomical prior of a
synthesized network in `cortex-connectome`, a seeded, integer-only rule that yields synapses
(a ring lattice with a local window and a rewired fraction, every $k$-th unit inhibitory with
negative weights, a local delay band and a far one) and depends on nothing; gives the runtime
the synthesis that writes a prior into an executor's arenas, a stationary drive that is a
function of the tick and a seed, and a causal branching-ratio oracle: two forks of one image
under one drive, one kicked, their spike trains differenced and the extra spikes attributed
through the kicked unit's synapses, so that the whitepaper's causal definition
(`update_branching_ratio(descendants, ancestors)`) has a caller with attribution for the first
time; and records what the exit tests hold on that network at 256 units (a pull request's
gate) and at 1 024 (the weekly job): the lag-one slope against the causal ratios at fixed
gains, the controller's trajectory under a step of an eighth, and, after a night of the
stages of [ADR-0037](../../docs/adr/0037-sleep-regulation.md) with two episodes tagged, the
weights among each pattern and whether a cue of half a pattern fires the rest. **One ADR**
gives `cortex-reasoning` the candidate-pair walk intra-construction can take and a bounded
proof search over a clause store (depth-first, backtracking over the clause, a frame slice as
the depth bound and a resolution budget), and gives the runtime the executive search that
walks the pairs, commits an invention whose reward is positive and undoes every other; its
exit test holds that every goal provable before the search is provable after it, with the
invented predicates read back on the proof's path at one step each, and that the store is
shorter by the pinned amount. Finding F-33 records the proposal's premises the tree
contradicted; H-8 and H-9 are dispositioned at the stated scale with the Appendix-A scale a
Target; H-11's symbolic half is decided and its synaptic half stays open with the reason (no
rule maps an id to a pattern of units). The lexicon is the next brief's first item and the
cortico-striatal gating is not adopted, with the reasons. No record in the image changes:
format 13 stays and the determinism pin is untouched. The whitepaper, README, `CLAUDE.md`,
the reader's guide, the ADR index and the changelog say all of this, and this brief is
archived with every check green.

---

## Standing directives

- Every claim is Implemented, Specified, Target or Hypothesis. What a test holds on a network
  of 256 or 1 024 units is stated as what it is, with the prior's parameters; what the same
  rules do at Appendix A's scale is a Target with the same generator
  ([ADR-0010](../../docs/adr/0010-measured-or-target.md)). No timing figure enters a document
  from a developer machine (the sizing in the Context is not a measurement). The words
  "validates", "reference-scale" (for anything below Appendix A's counts), "associative
  memory" and "autonomous" describe the proposal, never the tree.
- The repository wins over the document; a disagreement is a numbered finding in whitepaper
  §11, never a silent edit.
- No `f32`/`f64`, in the crates and in the tests; a ratio is Q16.16 in `u32`/`i32`, widened
  to `i64` to multiply; every operation on a state field saturates or wraps by name
  (`clippy::arithmetic_side_effects` is denied everywhere,
  [ADR-0029](../../docs/adr/0029-structural-enforcement.md)); a shift amount is bounded a line
  above the shift.
- 64-byte `#[repr(C, align(64))]` records with compile-time assertions; no heap types, threads
  or `unsafe` in a state crate (`unsafe_code = "forbid"`, ADR-0029); the prior yields synapses
  through an iterator over its own state, never a slice it allocates.
- Every quantity has one owner ([ADR-0016](../../docs/adr/0016-thirty-two-crate-architecture.md)):
  an anatomical prior is `cortex-connectome`'s (§5.2.2 names the priors as its responsibility)
  and, under TC-2, names no record of another crate; the synthesis, the drive, the forks and
  the search over the store are the runtime's ([ADR-0023](../../docs/adr/0023-executor.md)); a
  candidate pair and a proof are rules over terms, so they are `cortex-reasoning`'s. No new
  crate; no new record; the crate count stays 32.
- A change to a record bumps `CortexFileHeader::FORMAT_VERSION` (rule L-6). This round changes
  no record and adds none to the image: the prior is a parameter of a generator, not a section
  (a record the loop never reads is the class finding F-30 named), so the format stays 13.
- State crates declare no dependencies (TC-2); `cortex-executive` therefore cannot hold a
  search over `TermNode`, whatever the proposal's module name.
- Rule L-3: a term holds ids, never strings.
- The engine never amends its own code ([ADR-0031](../../docs/adr/0031-policy-amendment.md)); the
  search's budget is the caller's argument, not a registry entry, this round.
- The determinism pin of [ADR-0030](../../docs/adr/0030-verification-governance.md) moves only
  with a stated reason; the mutation gate on the changed lines must pass; a new rule carries
  a test over the lattice of `testkit/prop.rs`; every pinned number that an arithmetic oracle
  can produce is computed by that oracle before the test that asserts it is written; a pinned
  number that only the engine produces (a spike count, a gain sequence) is pinned from one run
  and stated as the engine's, as the determinism pin is.
- A heavy exit test (a window of $2^{17}$ ticks on a thousand units is 22 s in the debug
  profile on a developer machine) runs in the weekly job as an ignored test whose name
  contains `exhaustive`; the pull request's gate runs the same harness at 256 units.
- No product name enters a crate. Conventional Commits with a real body; never commit on
  `main`; the required checks keep their names.

## Context

Re-derived on 2026-09-13 against `main` at `5fdd8b3`. Line numbers move; the symbols and the
quoted sentences are what to re-derive.

1. **The estimator's names and the causal form's lack of a caller.** The proposal names
   `HomeostaticDrivePool::update_branching_ratio_q16`; `crates/cortex-homeostasis/src/lib.rs`
   has `update_branching_ratio(descendants, ancestors) -> u32` (the causal form, "the spikes
   a window's spikes caused divided by the spikes that caused them"; "the executor's
   estimator is `regulate`, which needs no attribution of a spike to its cause") and
   `estimate_branching_ratio() -> Option<u32>` (the lag-one slope). No caller of the causal
   form exists outside its own tests (`grep -rn update_branching_ratio`). Whitepaper §11.1
   H-8's protocol is "a reference image, a traced run, the estimate against a causal count
   from the trace", and the causal count is what the oracle of this round produces. Part of
   F-33.
2. **The networks the tree holds and the sites of H-8.** `runtime/cortex-runtime/tests/differential.rs`:
   a three-unit ring and `wire_random` of 128 units (the determinism pin,
   `PINNED_ARENA_HASH` `0x1724f3486c1d674e`, 95 spikes).
   `runtime/cortex-runtime/tests/criticality.rs`: `const UNITS: usize = 64`, `wire_recurrent`
   (two blocks per unit, eight synapses to two targets, delays 2 000 to 2 558, weights near
   the rail); its module comment says "a 48-unit network with depleting synapses", a comment
   defect against its own constant (part of F-33); [ADR-0036](../../docs/adr/0036-criticality-control.md)
   "What is not claimed" and §11.1 H-8 say 64. The proposal's "128-unit `wire_random`" as the
   site of H-8 conflates the two files.
3. **What decides a spike, and what a kick does.** `crates/cortex-core/src/dynamics/neuron.rs`:
   `synaptic_efficacy_q16(w, u, r) = (w × u × r) >> 15`, so a synapse at the rail with the
   release fraction at rest (`STP_U` 51 of 255) delivers at most 0.198 of the threshold's 1.0;
   `spike_message` carries an 18-bit two's-complement efficacy, so a negative weight is an
   inhibitory synapse and `FLAG_INHIBITORY` (`membrane.rs`) is defined and read by no rule
   (`grep -rn FLAG_INHIBITORY`: the definition and two re-exports). `membrane.rs::integrate`:
   the soma follows the basal potential with a coupling of $2^{-4}$ per tick, the basal leaks
   with $2^{-9}$, the refractory window is 200 ticks; [ADR-0038](../../docs/adr/0038-episodic-ledger-and-replay.md)
   probed a drive of 2.0 as no spike, 2.25 to 2.875 as one, 3.0 as two. The tests' kick
   (`criticality.rs::kick`, `modulation.rs::fire`) is three messages of 0.99. A unit a drive
   holds near its threshold fires on less and, since the basal potential outlives the
   refractory window, fires again on what is left: a kick is one to several ancestor spikes,
   which the oracle counts.
4. **The executor's inputs, bounds and quiescence.** `runtime/cortex-runtime/src/executor.rs`:
   `Config { workers, units, blocks, deltas, nodes_per_worker, deque_capacity,
   injector_capacity, trace_capacity, amendments, modulation_baseline_q16, control_step_q0_16,
   sleep_shift, episodes }`; `units_mut`, `blocks_mut` (exclusive, between ticks; not both at
   once), `injector`, `reward`, `tag_episode` (refused without `Config::episodes`), `wake`,
   `sleep_stage`, `homeostasis`, `episodes`, `replays`, `depotentiations`, `is_quiescent`,
   `shutdown -> Vec<WorkerReport>` (`spikes: Vec<(u32, u32)>` as `(unit, tick)`, `dropped`).
   A wheel slot holds `CAP` tokens and a full slot aborts (`phase_fan_out`); the tests use
   `Executor<64>` and production `Executor<2048>` (`ProductionExecutor`). `Image::encode`
   refuses a non-quiescent engine (`image.rs`; §8.7), so a fork of a driven network starts
   from the image the network was synthesized into and replays the same drive. `trial.rs`
   forks the same way for an amendment. The replay drive is two messages of 1.25
   (`REPLAY_DRIVE_Q16`, `REPLAY_MESSAGES`); a ripple is every $2^{11}$ ticks
   (`cortex_hippocampus::RIPPLE_SHIFT`); a pattern is at most twelve units (`PATTERN_MAX`);
   the window is $2^{17}$ ticks and the bin $2^{12}$ (`cortex_homeostasis::ACTIVITY_*`); the
   saturation ceiling is one spike per unit per bin (`saturation_ceiling`); the night's
   stages run at `sleep_shift` 5 from a pressure of 1.0 (`tests/sleep.rs::reload_with` patches
   the record through the image).
5. **What TC-2 forbids and where the search belongs.** `scripts/check-deps.mjs` and every
   `crates/*/Cargo.toml`: a state crate declares no dependency. The proposal's
   `ClauseSearchQueue` in `cortex-executive` over `TermNode` slices cannot exist; a rule over
   terms is `cortex-reasoning`'s (`induce.rs`: `intra_construct`, `resolve_definite`,
   `InduceScratch::{mark, restore}`) and the composition with the affect state and the reward
   is the runtime's (`discovery.rs`: `invent`, which refuses `NotPrimed` and `NotInStore`;
   `update_valence` primes the affect state to the length it was given, so a rejected
   invention needs the caller to put the state back). §5.2.30's Specified list: "Clause
   search, standardising apart, a proof store, constraint propagation, type raising and a
   chart". Part of F-33.
6. **The language stream's shape against the proposal's third pillar.**
   `crates/cortex-linguistic/src/lib.rs`: `ROLE_SUBJECT`, `ROLE_ACTION`, `ROLE_OBJECT`,
   `ROLE_AFFECT`, not the proposal's Agent, Patient, Action and Goal;
   `crates/cortex-basal-ganglia/src/lib.rs`: one rule, `compute_gating` (a net drive below
   zero selects the channel); `runtime/cortex-runtime/src/language.rs::comprehend` takes a
   category sequence, and §6.9 says "the lexicon Specified". Nothing in the tree routes a
   token to a role, and no measurement asks for a gating circuit. Part of F-33. Whitepaper
   §11.1 H-9 already says the language stream is not on the readout's path.
7. **"Reference scale" has two meanings and the tree uses one.** Whitepaper Appendix A: the
   reference configuration is $N_{\text{neuron}}$ = 43 000 000 and $N_{\text{block}}$ =
   67 108 864; §11.1 H-8 and H-9 say "a reference image" with no size; no test can run
   Appendix A's counts. This round names the network it measures on by its prior and its
   size, and leaves Appendix A's scale a Target with the same generator. Part of F-33.
8. **Sizing, from a developer machine (not admissible, not written anywhere but here).** A
   window of $2^{17}$ ticks on 1 024 units with 32 synapses each, two workers, the production
   wheel and a drive of eight messages per tick runs in about 2.5 s in the release profile
   and 22 s in the debug profile; at 256 units about a quarter of that. CI runs the tests in
   both profiles and the mutation gate reruns the runtime's tests per mutant, so the pull
   request's exit tests run at 256 units for a few windows, and the same harness at 1 024
   units for the full sweep is an ignored `exhaustive` test the weekly job runs.
9. **Where the round's numbers go.** The next ADR is 0044 (`ls docs/adr`); the next finding
   F-33 (whitepaper §11); H-8 and H-9 gain dispositions in §11.1 and H-11 a partial one; the
   next reference 75 (Appendix D); the whitepaper moves 4.9.0 → 4.10.0; the property kit is
   `testkit/prop.rs`; the mutation gate's exclusions are in `.cargo/mutants.toml`; the
   determinism pin is `PINNED_ARENA_HASH` in `runtime/cortex-runtime/tests/differential.rs`.


## Deliverables

- [x] **The reference-network ADR (the next free number)** (`docs/adr/0044-reference-network.md`,
  `depends-on: ADR-0036`, ADR-0038 named). In `cortex-connectome`, module `prior`: `Lcg`
  (Knuth's MMIX generator, the property kit's, with `below` and `between`), `Prior { units,
  inhibitory_every, synapses_per_unit, window, rewire_q0_8, delay_min, delay_max,
  far_delay_min, far_delay_max, weight_min, weight_max, inhibitory_gain_q4_4, apical_q0_8,
  seed }` with `is_well_formed`, `is_inhibitory(unit)` (every $k$-th unit), `synapses()` (an
  iterator yielding `Synapse { source, target, weight_q1_15, delay_ticks, apical }` unit by
  unit in slot order from one seeded walk: a target within $\pm W$ on the ring with a delay
  from the local band, or, for a `rewire_q0_8` fraction, anywhere on the ring with a delay
  from the far band, never the source itself; an inhibitory source's weight the drawn
  excitatory weight times the gain in sixteenths, saturating at the width, negated),
  `census()` (`Census { inhibitory_units, synapses, inhibitory_synapses, long_range,
  apical }`), `ring_distance`. In the runtime, module `synthesis`: `synthesize(units, blocks,
  prior) -> Result<Census, SynthesisError>` (every unit at its base threshold with the
  short-term factors at rest and `FLAG_INHIBITORY` set where the prior says, its blocks
  chained from `unit × blocks_per_unit`; refused for a malformed prior, a unit arena of another
  size, too few blocks, a delay in either band at the horizon), `blocks_per_unit`,
  `blocks_for`, `mix64` (SplitMix64's finaliser), `Drive { every, messages, efficacy_q16,
  units, seed }` with `is_due`, `unit_at` and `step` (a function of the tick and the seed: two
  forks under one drive receive the same messages); `Executor::arenas_mut`. Module
  `branching`: `Perturbation { unit, tick, messages, efficacy_q16 }`, `run_driven`, `trace`
  (refused when a worker dropped a spike), `fork(image, config, drive, kicks, ticks)`,
  `cascade(baseline, perturbed, unit, at, synapses, latency, advance) -> Cascade { ancestor,
  ancestors, first, advanced, extra, missing }` (an extra spike of a unit the kicked unit
  reaches within `latency` ticks after an ancestor spike plus that synapse's delay is a
  first-generation descendant; one the baseline holds as a spike of that unit within `advance`
  ticks after it that the perturbed fork lacks is advanced, not added; the kicked unit's own
  later extra spikes are ancestors), `Attribution::of(cascades)` with `branching_ratio_q16`
  (through `HomeostaticDrivePool::update_branching_ratio`) and `net_branching_ratio_q16`.
  Exit tests in `runtime/cortex-runtime/tests/reference.rs`: the prior written and read back
  through the image whole (the loader accepts every delay and every chain); the driven run
  bit-identical on one and four workers; at 256 units, the prior of the ADR, a drive of two
  messages per tick of 0.125 and a step of zero, the lag-one estimate per window at three
  fixed gains and the attribution of eight kicks at each, every number pinned; under a step of
  an eighth from a gain of 1.0, the gain and the estimate per window for ten windows, pinned;
  the plastic network (baseline 1.0) awake for three bins with an experience (a cue that fires
  a local cluster of twelve excitatory units together) in the third, a cluster and a random
  pattern tagged, a night from a pressure of 1.0 at shift 5 to the wake, the synapses among
  each pattern before and after (count and sum, pinned), and the readout on forks: a cue of
  six units on the image before the night and on the image after, the units of the rest that
  fire, for both patterns, pinned. The same at 1 024 units with sixteen kicks per gain and
  twelve windows of closed loop as `#[ignore]` tests named `..._exhaustive`. The ADR records
  the numbers of both, labelled, and what they decide.
  **Departure:** the gate's form is one fixed gain (2.0), one window and four kicks, and a loop of four windows from 2.0, so that the runtime's tests stay seconds long under the mutation gate; the three-gain, two-window, eight-kick sweep and the loop from 1.0 at 256 units run with the 1 024-unit tests as `exhaustive`. The prior's delay bands are two (local 100 to 300, far 1 400 to 2 559) because completion needs local delays within a basal time constant. The oracle attributes through the synapses (a latency of 128 ticks, an advance of 512), not by a time window.
- [x] **The clause-search ADR (the number after it)** (`docs/adr/0045-clause-search.md`,
  `depends-on: ADR-0043`, ADR-0041 named). In `cortex-reasoning`'s `induce.rs`:
  `next_pair(store, after, scratch)` (the next `(i, j)`, `i < j`, in index order whose
  clauses both have two literals or more and heads of one shape through the bindings),
  `Frame { goal, next, mark }`, `Proof { steps, resolutions }`, `prove(goal, store, frames,
  budget, scratch)` (depth-first over the store in index order, each step `resolve_definite`
  with the next clause whose head unifies with a literal of the goal, backtracking over the
  clause; the frame slice the depth bound, `Budget` past the resolution budget; the scratch
  left as it was, proof or not; no renaming between uses, which the ADR states as the
  Specified item it is), `InduceError::Budget`, `InduceMark: Default`. In the runtime's
  `discovery.rs`: `search(store, len, scratch, affect, budget, out) -> Result<SearchReport,
  DiscoveryError>` (the pairs in order, each through `invent`; a positive reward is committed,
  the two inputs replaced by the common clause and the first definition and the second
  definition appended, the walk restarted; a reward that is not positive is undone with the
  affect state put back; the operator's own refusals of a pair are rejections; `StoreFull`
  and `OutFull` with the invention undone), `SearchReport { attempts, commits, rejections,
  length_before, length_after, reward_total_q16 }`. Exit test in
  `runtime/cortex-runtime/tests/discovery.rs`: a store of three clauses of one head sharing
  four literals of arity one and differing in a fifth, one clause of another head, and the
  facts of three constants; the search commits two inventions (46 nodes become 40, two
  rewards of three quarters); every goal of the form `p(k)` provable before the search is
  provable after it and no other is; the proof's steps go from the hand-counted 6 to 8, 8 and 7
  (the invented predicates read back on the path, one step each); the committed rewards passed
  to `Executor::reward` consolidate a pending trace in the two-unit network of `modulation.rs`.
  **Departure:** the proof search selects the leftmost literal (SLD's selection) through a new `resolve_literal(goal, rule, index)`, since resolving whichever literal a clause's head unifies with searched a goal of five literals in every order and spent a budget of 4 096; the exit test's store holds the facts of four constants, the fourth unprovable, and the modulator check reuses the file's own helpers.
- [x] **Finding F-33** in whitepaper §11: the causal form with no caller and the proposal's
  name for it; `criticality.rs`'s comment of 48 against its constant of 64; a clause search
  placed in a crate TC-2 forbids it; the frame's roles misnamed; "reference scale" used for a
  size the tree does not name; `FLAG_INHIBITORY` defined and read by no rule. Disposition:
  resolved by the two ADRs and the corrected comment; the flag written by the synthesis as the
  unit's annotation and read by the exit test's pattern choice, the dynamics reading the
  weight's sign.
- [x] **H-8 and H-9 dispositioned** in §11.1 at the stated scale with the numbers, the
  Appendix-A scale a Target; **H-11's symbolic half decided** (the store shorter, every proof
  kept, the invented predicates read back) and its synaptic half open with the reason; an
  open question on the estimator's one-window noise and the ceiling as the controller's fixed
  point at this scale (a smoothed estimate needs a second record line).
- [x] **The documents.** Whitepaper 4.10.0: the executive summary's sentence on what exists;
  §1.6 rows (`cortex-connectome`, `cortex-reasoning`, the date); §5.1 (the runtime's
  modules); §5.2.2 (Public API, Status: the prior Implemented, the laminar sheet Specified);
  §5.2.16 (the causal form's caller); §5.2.30 (Public API, Status, a "Clause search"
  paragraph with directives `fn next_pair`, `fn prove`); §6.6 and §6.10 (what the runtime
  composes); §8.8 rows (criticality: the oracle; complementary learning: the completion;
  compression progress: the search); §9 two rows; §11 F-33; §11.1 H-8, H-9, H-11 and the
  open question; Appendix C (M5 and M8 clauses); Appendix D references 75 onward (Watts and
  Strogatz 1998; Brunel 2000; Steele, Lea and Flood 2014; Knuth 1997 if not present); the
  glossary (Prior, Cascade, Attribution, Proof search). README (the Implemented row's clause;
  the connectome and reasoning rows), `CLAUDE.md` (the sentence on what exists),
  `docs/zh-TW/README.md` (§6 and §11 rows), `docs/adr/README.md` (two rows), `CHANGELOG.md`
  (one entry under Unreleased in the shape of brief 021's).
- [x] **Not adopted, with the reason in the ADR that is closest:** the runtime lexicon (the
  next brief's first item: a lookup from a token id to a category term is the whole of it
  and it is not on the path of any hypothesis this round decides; the clause-search ADR);
  the cortico-striatal gating circuit (no measurement asks for it, `compute_gating` is one
  rule with no caller, and the frame's roles are bound by the reduction already; the
  clause-search ADR); a prior in the image (a record the loop never reads; the
  reference-network ADR); a change to the estimator's rule, the ceiling, the bin or the
  sleep constants on the strength of one network (the reference-network ADR names what a
  second prior must show first); the multistep regression (the same ADR: the one-window
  slope's noise is recorded and the record has no line for more sums); the synaptic half of
  H-11 (the clause-search ADR: no rule maps an id to a pattern of units, and the ledger's
  `Episode` is the record such a rule would write).
- [x] **This brief archived** under `briefs/archive/` with the frozen banner, every box
  dispositioned, the precondition directives removed and the links rebased.

## Not empowered

- No executor field, no image section, no format bump: the prior is the generator's argument,
  the drive the caller's, the forks the harness's.
- No new crate and no new record; no field in `DendriticSuperNeuron` or `SynapseBlock`.
- No float anywhere, including the tests and the probes that size them.
- No change to `estimate_branching_ratio`, `regulate`, the bin, the window, the ceiling, the
  ripple, the replay drive or the sleep thresholds: this round measures them and says what it
  saw.
- No renaming of the CI jobs the ruleset requires; no move of the determinism pin.
- No test in the pull request's gate above a few windows at 256 units; the full sweep is the
  weekly job's.
- No word of the proposal's vocabulary in a document's claims: a rule is what it does.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in
the ADR that owns it: the prior's shape and parameters (the window, the rewired fraction, the
two bands, the inhibitory rule and gain, the synapse count), the drive's form, the kick's
strength, the oracle's `latency` and `advance`, the sizes and the gains the exit tests use,
the commit rule of the search, the proof search's selection rule, the exit test's store. It
may not reach the standing directives, the whitepaper's invariants or the constraints in
`CLAUDE.md`.

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
with 95 spikes, untouched. The three precondition directives above are gone with the archived
brief.

## Report

The closing message states: which of the proposal's premises the tree contradicted and where
each correction went; the prior's rule and the census of the network it makes at each size;
what the oracle attributes and how a kick's second spike and an advanced spike are counted;
the estimate against the gross and net causal ratios at each gain, the controller's
trajectory and where it settled, and what that decides for H-8 at this scale; the synapses
among each pattern before and after the night and the readout's counts for both patterns on
both images, and what that decides for H-9; the search's commits, the store's lengths and the
proof lengths before and after, and what that decides for H-11's symbolic half and why the
synaptic half stays open; that the format and the pin are untouched and why; what the
mutation gate found on the changed lines and how each survivor was answered; what was not
done (the lexicon, the gating circuit, the prior in the image, the tuning, the multistep
regression, the synaptic readback) and why; and what the re-examination after the round
recommends next.
