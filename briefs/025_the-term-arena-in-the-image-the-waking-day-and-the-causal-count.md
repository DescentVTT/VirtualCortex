---
status: proposed
date: 2026-09-14
---

# Brief 025 — The term arena in the image and the discovery loop inside the tick, the waking day and the inhibitory target as an image parameter, and the causal count inside the loop

> A self-contained prompt for one round of work. Read `briefs/README.md`, then this file, then
> the files it names. Frozen when executed; the outcome lives in the ADRs that accept it, the
> changelog and the code.

## Mission

Take the three steps the closing report of brief 024 recommended, in its order, as decisions,
code, measurements and tests. **One ADR** gives the image a term arena and moves the discovery
loop inside the tick: the clause store, the term nodes it names, the counters that number the
arena's variables and its inventions, the affect state and the association between an invented
predicate and its episode are the executor's and the image's (format 14), the search of
[ADR-0045](../docs/adr/0045-clause-search.md) runs on a cadence of the tick while the engine is
awake with a bounded budget from a cursor it keeps, its reward goes into the modulator and the
coincidence before the reward is tagged from the executor's own train and bound to the invented
predicate without a caller, and a caller may still trigger the same loop between ticks; a
committed invention's outputs are instantiated through the bindings at the commit so that the
store in the image is nodes alone and the binding table stays the scratch it is. **One ADR**
runs a waking day on the reference network: the population rate, the drift of the inhibitory
and the excitatory weights, the gain and the estimate per window over the engine's own day (the
pressure's time constant at shift 5, a night included), at 256 and 1 024 units, under the
target rate [ADR-0049](../docs/adr/0049-dale-principle-in-plasticity.md) chose and under one that
matches the regime; decides where that target lives (the behaviour gate of
[ADR-0031](../docs/adr/0031-policy-amendment.md) admits no parameter that changes what the engine
does, so the registry cannot hold it; the image can), makes the rule take the depression per
presynaptic spike as an argument, and either tunes a sleep placeholder under a criterion stated
before the run or keeps it with the reading. **One ADR** puts the causal count inside the loop:
a message carries one bit that says it is a synapse's, a unit keeps the tick a synapse's message
last reached it, a spike within the oracle's latency of that tick is a descendant, the executor
tallies descendants beside spikes, the ratio is measured against the oracle's gross and net
ratios on both priors at 256 and 1 024 units under a decision rule written here before the run,
and the record takes the count and the controller reads it only if the rule holds. When the
round is done: the whitepaper, README, `CLAUDE.md`, the reader's guide, the ADR index and the
changelog say all of this, the format is 14, the determinism pin moves for the one stated
reason (a field of the unit record is written by the loop) and is taken from the run, and this
brief is archived with every check green.

---

## Standing directives

- Every claim is Implemented, Specified, Target or Hypothesis. What a test holds on a network
  of 256, 1 024 or 4 096 units is stated as what it is, with the prior's parameters; what the
  same rules do at Appendix A's scale is a Target with the same generator
  ([ADR-0010](../docs/adr/0010-measured-or-target.md)). No timing figure enters a document from
  a developer machine. A rule is what it does: no "validates", no "autonomous", no
  "reference-scale" for anything below Appendix A's counts; "a day" is the engine's own (the
  pressure's time constant), stated in windows and in simulated seconds, never a clock's.
- The repository wins over the document; a disagreement is a numbered finding in whitepaper
  §11, never a silent edit. An accepted ADR is history: the numbers ADR-0044 to ADR-0051 pinned
  stay in them with a note where this round moves them; the whitepaper and the tests carry
  the numbers as the tree has them after this round.
- No `f32`/`f64`, in the crates and in the tests; a ratio is Q16.16 in `u32`/`i32`, widened to
  `i64` to multiply; every operation on a state field saturates or wraps by name
  (`clippy::arithmetic_side_effects` is denied everywhere,
  [ADR-0029](../docs/adr/0029-structural-enforcement.md)); a shift amount is bounded a line
  above the shift.
- Every loop ends by construction: a countdown, a range, a scan by `get`, a slice's iterator,
  a recursion whose depth argument falls to a stated bound; never by a comparison alone that
  one operator flip turns into a walk without end. Every walk over the arena visits at most
  `WALK_LIMIT` nodes.
- 64-byte `#[repr(C, align(64))]` records with compile-time assertions; no heap types, threads
  or `unsafe` in a state crate (`unsafe_code = "forbid"`, ADR-0029). A rule of `cortex-core`
  takes what it needs as an argument; it does not read a unit's flag or a policy.
- Every quantity has one owner ([ADR-0016](../docs/adr/0016-thirty-two-crate-architecture.md)):
  the term arena's record and rules are `cortex-reasoning`'s, the affect record
  `cortex-affect`'s, the episode's binding to a symbol `cortex-hippocampus`'s, the descendant
  rule and the message bit `cortex-core`'s, the tally and the ratio `cortex-homeostasis`'s, the
  composition of all of them the executor's ([ADR-0023](../docs/adr/0023-executor.md)). No new
  crate; the crate count stays 32. A new record in a crate is decided by an ADR of this round
  that names the gap it fills and no existing record owning the quantity.
- A change to a record is an ADR of this round with the format bump (13 → 14, one bump for the
  round), the record's table in whitepaper §5.2 moved, every fixed offset of the image tests
  moved, and a changelog entry; the reserved bytes of every record stay zero except where the
  ADR names the field that takes them.
- Rule L-3 and §1.5: no word, no string and no language name enters a crate; a clause holds
  ids.
- The determinism pin of [ADR-0030](../docs/adr/0030-verification-governance.md) moves only
  with a stated reason (this round has one: the unit record gains a field the loop writes);
  the mutation gate on the changed lines must pass; a new rule carries a test over the lattice
  of `testkit/prop.rs`; every pinned number an arithmetic oracle can produce is computed by that
  oracle before the test that asserts it is written (the description lengths, the encodings,
  the depression per spike at each period); a number only the engine produces is pinned from
  one run and stated as the engine's.
- A decision rule for a measurement is written in this brief before the run and applied as
  written; what the numbers say beyond it is recorded as a reading, never folded into the rule
  after the fact (brief 024's lesson, ADR-0051).
- The engine never amends its own code ([ADR-0031](../docs/adr/0031-policy-amendment.md)); a
  parameter that changes what a run does is in the image or in the trace (§8.3), never in a
  configuration alone.
- A heavy exit test runs in the weekly job as an ignored test whose name contains
  `exhaustive`; the pull request's gate runs at most what it runs today plus this round's tests
  at 256 units, sized so that the gate's runtime tests stay within their present order.
- No product name enters a crate. Conventional Commits with a real body; never commit on
  `main`; the required checks keep their names.

## Context

Re-derived on 2026-09-14 against `main` at `9e31fc4`. Line numbers move; the symbols and the
quoted sentences are what to re-derive.

1. **What the image holds and what it does not.** `runtime/cortex-runtime/src/image.rs`
   writes the neuron, synapse, delta (when any), amendment (when any), modulator, homeostasis,
   hippocampus and episode (when any) sections; `tests/modulation.rs` asserts `section_count`
   5 for a network without deltas, amendments or episodes; `tests/image.rs` builds images by
   hand (`small_image`, `small_image_with_modulator`, `patch_section`, `patch_header`,
   `patch_entry`) and holds every clause of the loader's checks
   (`every_clause_of_the_loader_s_checks_refuses_on_its_own`). `cortex-connectome` names
   `SECTION_TERM` (40, "the loader's support is Specified") and kinds 41 to 45; kind 46 is
   the hypervector body's (Appendix A row 9, Specified); the kinds are Appendix A row numbers.
   `CortexFileHeader::FORMAT_VERSION` is 13 with the version history in its doc comment and in
   whitepaper §5.2.2 (a directive `FORMAT_VERSION: u32 = 13` at §5.2.2). The modulator
   section is one 64-byte record: the modulator's 16 bytes, the baseline at `[16..20)`, 44
   reserved bytes ("the baseline changes what a run does, so it is in the image, not in a
   configuration (§8.3)").
2. **The store, the scratch and the bindings.** `runtime/cortex-runtime/src/discovery.rs`:
   `search(store, len, scratch, affect, budget, out)` walks `next_pair` from the first pair,
   restarts from the first pair after every commit, and ends when no pair is left or the
   budget is spent; `invent` measures the store "through the bindings" and `intra_construct`'s
   outputs "are instances of their inputs, and the bindings the matching made stay in the
   caller's table" (`crates/cortex-reasoning/src/induce.rs`, its module doc). So a committed
   store reads through the binding table: `tests/reference.rs::search_and_tag` builds a fresh
   table per call and drops it after, and a second search over the same store reads the
   clauses without the bindings the first left. `InduceScratch { arena, free, bindings, trail,
   trail_len, stack, pairs, next_variable, next_invented }`; `mark` and `restore` (which zeroes
   the nodes appended since the mark and undoes the bindings). `next_pair(store, after, s)`
   resumes after a pair. `Discovery`, `SearchReport { attempts, commits, rejections,
   length_before, length_after, reward_total_q16 }`. `prime(affect, length)` writes the store's
   length into `free_energy_prev_q16`; `search` refuses `NotPrimed`. `TermNode` (64 bytes:
   `kind`, `arity`, `_pad`, `functor`, `children` as index + 1, 24 reserved) has constructors
   and no `encode`, `decode` or `is_well_formed`; `TermNode::compound` takes the children's
   indices, so a host builds bottom-up; `lgg` and `Body::alloc` allocate children before the
   parent. `InteroceptiveState` (`crates/cortex-affect/src/lib.rs`, 64 bytes, 20 reserved) has
   no `encode`, `decode` or `is_well_formed`. `Episode` (`crates/cortex-hippocampus/src/lib.rs`)
   has `_reserved: [u8; 8]` at `[56..64)`; `HippocampalAttractorState` keeps the ledger's
   length and hand. The runtime's `Association { predicate, episode, pattern, len, burst }`
   (`episode.rs`) is "the caller's table, as the clause store is (ADR-0043): no record holds
   it"; `discover(exec, ClauseSearch { store, len, scratch, affect, budget }, Tagging {
   window, coincidence, priority }, discoveries, associations)` runs the loop between ticks
   over a caller's store ([ADR-0050](../docs/adr/0050-the-train-inside-the-executor.md)), and
   its option 3 ("a cadence inside `tick()` that runs the search over an executor-owned store
   on every ripple while awake") was not adopted because "the store is the caller's arena".
   `Executor::tick` (`executor.rs`) runs the three phases, `merge_spikes`, the increment,
   `rehydrate_pending` and `tally`; `ripple()` runs before the first barrier on
   `RIPPLE_CADENCE` in sleep; `Cadence::new(shift, phase)` (`cortex-core`) is `None` at a
   shift of the width. `Config` is spelled out in full in `tests/differential.rs` and
   `tests/contention.rs`. Nothing allocates in the loop (`tests/no_alloc.rs`). The reference
   tests tag with a priority of 200, a window of one ripple (`RIPPLE`, $2^{11}$ ticks) and a
   coincidence of `COINCIDENCE` 512 ticks ("a basal time constant (ADR-0018)";
   `BASAL_LEAK_SHIFT` is 9).
3. **The inhibitory rule's target and the registry.** `crates/cortex-core/src/dynamics/synapse.rs`:
   `ISTDP_TARGET_PERIOD_TICKS` 20 000, `ISTDP_ALPHA_Q1_15` 67 tied by a `const` assertion to
   `2 A_+ 2^{τ} / period`; `step_stdp(slot, pre_now_tick, post_last_tick, polarity)` subtracts
   the constant at every presynaptic spike of an inhibitory block; callers: `phase_fan_out`,
   the oracle blocks of `tests/sleep.rs` and `tests/modulation.rs`, `benches/cortex-bench`.
   Whitepaper §11.1's item after F-36 says "the round that runs a waking day on the reference
   network measures both and makes the rate a `REGISTRY` entry". But the registry's third gate
   (`PolicyAmendment::record_trial`, [ADR-0031](../docs/adr/0031-policy-amendment.md): "the
   candidate fork's behaviour hash equals the baseline's, or the amendment is rejected as
   `REJECT_BEHAVIOUR_CHANGED`... *the engine may change what it costs, never what it does*")
   admits no parameter whose change moves a weight, so the sentence prescribed what the
   registry cannot hold: a finding of this round (F-37). The reference prior draws every
   inhibitory weight at the negative rail (`inhibitory_gain_q4_4` 255, −32 767), so under the
   rule a rail weight can only lose magnitude, α per presynaptic spike, and regain it by
   pairings: onto a target that fires, it stays near the rail; onto a silent one it decays to
   zero in about 489 presynaptic spikes. The reference units fire at 8 to 30 Hz under the
   drive at a gain of 2.0 (H-9's disposition); the target of 5 Hz is a period of 20 000
   ticks, and a period of 5 000 ticks is 20 Hz.
4. **The day and the sleep placeholders.** `HomeostaticDrivePool::step_sleep`
   (`crates/cortex-homeostasis/src/lib.rs`): with the shift $k$ the pressure rises by
   $(1 - S) 2^{-k}$ per window awake and falls by $S 2^{-(k-2)}$ asleep; at shift 5 the rise's
   time constant is 32 windows (42 s at the fine tick) and the nights of `tests/reference.rs`
   last seven windows from a pressure of 1.0; `SLEEP_ONSET_DAY_Q16` 0.875, `WAKE_DAY_Q16`
   0.375, the night quarter's 0.5 and 0.125, `SWS_WINDOWS` 4, `REM_WINDOWS` 2 are "a Target to
   tune"; whitepaper §11.1 lists them as placeholders "the round that measures H-9 tunes". The
   harness: `windows(exec, drive, n)` runs `n` windows under the drive and returns `(gain,
   estimate, spikes)` per window; `among(exec, pattern)`; `reload_with` patches the
   homeostasis section; `night`/`sleep` run the stages from a pressure of 1.0. A window is
   $2^{17}$ ticks, 1.31 s; sixty-four windows are 84 s of simulated time and hold one rise to
   the onset and one night at shift 5.
5. **The message, the unit and the tally.** `spike_message(efficacy, apical)` puts the
   efficacy in bits 0–17 and the compartment in bit 18; "bits 19–31 are zero"; messages sort
   by value in the turn's batch (§8.3). The executor makes a synapse's message in
   `phase_fan_out` (a zero-delay synapse) and `phase_deliveries` (a token due); the injector's
   messages (the drive, the kicks, the cues) and the replay's `REPLAY_MESSAGE` are not a
   synapse's. `DendriticSuperNeuron` has `mailbox_reserved: u64` at `[16..24)` ("Reserved; MUST
   be zero"; the ABA tag until ADR-0017) and `_reserved: u16` at `[52..54)`, which §8.8 names
   for the synaptic scaling gain (Specified). `turn` drains the batch, integrates, and on a
   spike pushes to `spiked`, the trace and `shared.fired`; `phase_turns` stores
   `spiked.len()` into `shared.spikes[id]`; `tally` sums them into `count_activity`. The
   oracle's `LATENCY` is 128 ticks ("a descendant fires within this many ticks of its message's
   arrival") and `ADVANCE` 512 (a baseline spike within it after a descendant is the same
   spike, advanced; the net ratio subtracts these). `HomeostaticDrivePool` has no reserved
   byte: `energy_level` (`[0..4)`, "Energy reserve") is written by no rule in the workspace
   (`grep -rn energy_level crates runtime`), like F-29's `sensory_fatigue` was, and `sum_prev`
   (`[32..40)`, `u64`) is bounded by the cap and the window at $31 (2^{24} - 1) < 2^{29}$,
   which fits `u32`; `regulate(saturation_per_bin)` reads the lag-one regression and the
   ceiling. ADR-0051: "a controller that reads the branching ratio below the ceiling needs the
   causal count inside the loop (an ancestor stamp carried per delivery), which stays
   Specified". The determinism pin `PINNED_ARENA_HASH` (`tests/differential.rs`) hashes every
   unit's image bytes, so a field the loop writes moves it.
6. **The decision rules, written before the runs.**
   - *The causal count.* The in-loop ratio (descendants over spikes, per window) is the
     controller's reading when, at 256 and 1 024 units on both priors at every gain of
     `GAINS` below the ceiling, it is within 0.15 (Q16.16 9 830) of the oracle's gross causal
     ratio of the same run, and it rises with the gain on both priors at both sizes. Then the
     record takes the two counts (the open bin's descendants where `energy_level` was, the
     window's where the upper half of `sum_prev` was; F-38 for the field written by nothing),
     `regulate` reads descendants over spikes below the ceiling and the regression stays the
     reading beside it, and the loop's twelve windows are pinned under the new reading.
     Otherwise the count stays the executor's reading (its counters, not the record's), the
     controller is untouched, the record keeps its layout, and the reading is recorded with
     the difference stated. Either way the per-window in-loop ratios are pinned beside the
     oracle's on both priors at both sizes.
   - *The sleep placeholders.* A constant moves only when the day's reading shows a stage the
     constant makes unobservable at shift 5 (a night without a ripple, a slow-wave stage
     without a replay, a day without a window awake); it moves to the smallest value that
     makes the stage observable, and the ADR says so. Otherwise the placeholders stay, the
     reading attached to the §11.1 item.
   - *The target rate.* No rule decides a value: the period is a parameter of the image with
     20 000 as its default and the reading at the two periods says what each does to the
     rates and the weights over the day; the ADR states the reading and names what a round
     that chooses a value would need (an objective over the rate).
7. **Where the round's numbers go.** The next ADR is the next free number (`ls docs/adr`);
   the whitepaper moves 4.12.0 → 4.13.0; findings F-37 (the §11.1 sentence against the
   behaviour gate) and F-38 (a homeostasis field written by nothing, if the causal rule holds)
   are the next numbers of §11; §11.1's items after F-36 and on the sleep placeholders gain
   their dispositions; the estimator's item gains the in-loop reading; H-11's disposition
   gains the association in the image; Appendix A gains rows for the engine's affect and
   induction records and the clause store; the property kit is `testkit/prop.rs`; the
   mutation gate's exclusions are in `.cargo/mutants.toml`; the weekly job runs `cargo test
   --workspace --release --locked -- --ignored exhaustive`.
8. **What is not in the tree that a reader might assume.** No section of the image holds a
   term, a clause, an affect state or an association; no search runs without a caller; no
   message says whether it is a synapse's; no unit knows when a synapse's message last reached
   it; no count of descendants exists inside the loop; no rule takes the depression per
   presynaptic spike as an argument; nothing writes `energy_level`.

## Deliverables

- [ ] **The term-arena ADR (the next free number)** (`depends-on: ADR-0050`; ADR-0025,
  ADR-0041, ADR-0043 and ADR-0045 named). In `cortex-reasoning`: `TermNode::{encode, decode,
  is_well_formed}` (a kind the constants name, an arity of zero for a constant or a variable
  and at most eight for a compound, the slots beyond the arity zero, the pad and the reserved
  bytes zero); `instantiate(term, s) -> Result<u32, InduceError>`: a copy of `term` through
  the bindings with every bound variable replaced by its binding's instance, children
  allocated before their parent, the same index returned when no variable beneath is bound,
  the depth bounded by a stated constant and every walk by `WALK_LIMIT`;
  `InduceScratch::unbind(&mark)`: the bindings made since the mark undone and the trail cut,
  the nodes kept; a third record `InductionState` (64 bytes, `Copy + Default + Eq`,
  `encode`, `decode`, `is_well_formed`): the arena's `free`, `next_variable`,
  `next_invented`, the store's `clauses`, the search's cursor as the pair after which the
  next search resumes (each as index + 1, zero the start), `search_budget`, `search_shift`
  (0 never searches) and `tag` (the REM ripples an invention's episode survives), the rest
  reserved and zero. In `runtime/cortex-runtime/src/discovery.rs`: `search` instantiates the
  three outputs of a commit and unbinds to before the attempt, so that the store's clauses are
  self-contained and the table is empty after every search; `search_from(store, len,
  scratch, affect, budget, after, out) -> Result<(SearchReport, Option<(usize, usize)>),
  DiscoveryError>` resumes after a pair and returns the pair after which the next search
  resumes (the start after a commit or a complete pass); `search` is `search_from` from the
  start. In `cortex-affect`: `InteroceptiveState::{encode, decode, is_well_formed}` (the
  comfort and the mood within $[-1, 1]$, the reserved bytes zero). In `cortex-hippocampus`:
  `Episode::symbol: u32` at `[56..60)` (the id of the symbol the pattern stands for, zero for
  none, never written twice), `Episode::bind(symbol) -> bool` (refused for zero and for an
  episode already bound), `is_well_formed` unchanged for the field, the reserved bytes now
  `[60..64)`. In the executor: `Config::{terms, clauses, search_shift, search_budget,
  discovery_tag}`; a term arena, a clause store, the scratch (bindings, trail, stack and
  pairs sized from `terms`), the affect record, the induction record and a discovery buffer
  allocated once in `new` (`new` refuses clauses without terms and a tag of zero with terms);
  the inputs between ticks `Executor::term(node) -> Result<u32, TermError>` (refused for an
  empty kind, a child at or beyond the arena's cursor, a variable outside the table, a full
  arena; `next_variable` moved above a host's variable) and `Executor::assert_clause(head,
  body) -> Result<u32, TermError>` (the clause node into the arena, its index into the store,
  the affect primed to the store's new length; refused for a full store); the accessors
  `terms`, `clauses`, `affect`, `induction`, `searches`, `inventions`, `untagged`;
  `Executor::discover(&mut self) -> Result<DiscoverReport, DiscoverError>` between ticks:
  the search from the cursor with the record's budget over the executor's own store, the
  committed rewards' total into the modulator when positive, then the densest coincidence
  (`COINCIDENCE_TICKS`, $2^{\text{BASAL\_LEAK\_SHIFT}}$) of the ripple before now tagged from
  the executor's own train with the record's tag and bound to the first commit's predicate
  (the search's further commits reported, bound to no episode, stated as such), one tag per
  rewarded search; and the same loop inside `tick()` after the tally, on the cadence
  `Cadence::new(search_shift, 0)` while the stage is awake, its refusals counted (a search
  that ends in its own error, a tag the ledger or the train refuses) and never returned. The
  image: `SECTION_TERM` (40) holds the arena's first `free` nodes when there are any,
  `SECTION_CLAUSE` (a new kind, Appendix A's next row) the store's indices as 4-byte
  records when there are any, `SECTION_AFFECT` and `SECTION_INDUCTION` (new kinds) one
  record each, always; the loader refuses a node that is not well formed or whose child is
  not below its own index (so no loaded arena is cyclic), a variable outside the table, a
  store index at or beyond `free` or naming a node that is not a clause or named twice, an
  induction record that is not well formed or whose `free` and `clauses` are not the
  sections' counts or whose `next_invented` is outside the band, an affect record that is
  not well formed, an episode whose symbol is outside the invented band; `FORMAT_VERSION` 14
  with its history line. The ADR-0050 compositions over a caller's store (`discover`,
  `ClauseSearch`, `Tagging`) are removed; the three functions over a caller's train and the
  three `recent` forms stay, `tag_discovery` and `tag_discovery_recent` binding the episode
  they tag. Tests, every arithmetic number from an oracle first: the encodings round-trip
  and every clause of `is_well_formed` on the three records and on `Episode::bind`;
  `instantiate` on a term with no bound variable (the same index), with a bound variable at
  the root, beneath a compound and bound to a compound that contains a bound variable, at the
  depth bound and past it, and the identity that the instance's `size` and `term_hash` equal
  the original's through the bindings; `unbind` keeps the nodes and empties the table; the
  exit search of `tests/discovery.rs` commits the same two inventions to the same lengths
  with an empty table after, and every goal provable before is provable after through fresh
  bindings; `search_from` resumes where the cursor says and reports the start after a commit;
  the executor's inputs refuse what they refuse; an image with a store round-trips byte for
  byte and a loaded engine's next search commits what the un-loaded one commits; every
  loader clause in `tests/image.rs`; `tests/no_alloc.rs` with a store, a cadence and a search
  that commits inside the loop; `capture_night` in `tests/reference.rs` on the executor's own
  store through `discover` and through the cadence (the same night either way, pinned), the
  association read from the episode; the determinism test with a store and the cadence on
  one and four workers. The ADR states why the outputs are instantiated at the commit (the
  table is a scratch; the image holds nodes), why the cursor (a budget without one repeats
  its first pairs), why the episode carries the symbol (one owner for the pattern and what it
  stands for; ADR-0016's field-not-crate rule), what one episode does with a search of two
  commits, what the cadence costs per tick (bounded by the budget and the walk limit), and
  what is Specified (a compaction of the arena's garbage; standardising apart; a trigger that
  is not a cadence).
- [ ] **The waking-day ADR (the number after it)** (`depends-on: ADR-0049`; ADR-0031,
  ADR-0036, ADR-0037 and ADR-0044 named). In `cortex-core`:
  `istdp_alpha_q1_15(target_period_ticks) -> i32` with `ISTDP_PERIOD_MIN_TICKS` (100, 1 kHz,
  α 13 434) and `ISTDP_PERIOD_MAX_TICKS` (1 000 000, 0.1 Hz, α 1) and a `const` assertion that
  the function at `ISTDP_TARGET_PERIOD_TICKS` is `ISTDP_ALPHA_Q1_15`; `step_stdp(slot,
  pre_now_tick, post_last_tick, polarity, istdp_alpha_q1_15)` and `step_stdp_all(now_tick,
  post_last_ticks, polarity, istdp_alpha_q1_15)`, the excitatory branch ignoring the
  argument; every caller updated. In the executor: `Config::istdp_target_period_ticks`
  (default 20 000; refused outside the bounds), the period in the modulator section at
  `[20..24)` (the image's outranks the configuration's, as the baseline's does; the loader
  refuses it outside the bounds), the depression published to the workers before the barrier
  with the gain, `Executor::istdp_target_period_ticks()`. In `tests/reference.rs`: a harness
  `day(prior, units, windows, step, shift, period)` returning per window the spikes, the gain,
  the estimate, the stage, the sum of the inhibitory magnitudes and of the excitatory weights
  over the arena, with a gate test at 256 units (the lattice at a gain of 2.0 held, no
  controller, no sleep, eight windows at each of the two periods) and an `exhaustive` test at
  1 024 units (sixty-four windows at shift 5 under the controller's step of an eighth, at
  each period, on the lattice), every number pinned from the run; the sleep decision rule of
  Context item 6 applied to the day's stages. The ADR's table: per size and period the rate
  per unit per window (spikes over units, stated also in hertz), the inhibitory sum's drift
  over the windows, the excitatory sum's, the gain's course and the stages; finding F-37 and
  the §11.1 item after F-36 dispositioned (the period is an image parameter, not a registry
  entry, and why); the sleep placeholders' disposition under the rule; what a round that
  chooses a target would need.
- [ ] **The causal-count ADR (the number after that)** (`depends-on: ADR-0051`; ADR-0036,
  ADR-0044 and ADR-0047 named). In `cortex-core`: `MESSAGE_SYNAPTIC` (bit 19),
  `synaptic_message(efficacy, apical)` and `message_is_synaptic`; `CAUSAL_LATENCY_TICKS`
  (128, the oracle's, `tests/reference.rs::LATENCY` tied to it); `DendriticSuperNeuron::
  last_synaptic_tick: u32` at `[16..20)` (`NO_SPIKE_ON_RECORD` for none: no message reaches a
  turn at tick 0), `[20..24)` reserved; `note_synaptic_input(now)` and `is_descendant(now)
  -> bool` (a stamp on record within the latency); the image's at-rest check leaves the
  stamp as it is. In the executor: the two synaptic sites make synaptic messages; `turn`
  stamps a unit whose batch holds one and, on a spike, counts a descendant; `phase_turns`
  publishes the count beside the spikes; `tally` sums descendants; `Executor::descendants()`
  and a per-window reading the harness takes. In `tests/reference.rs`: the in-loop ratio per
  window beside the oracle's gross and net ratios in the 256-unit gate tests and the
  1 024-unit `exhaustive` tests on both priors, pinned; the rule of Context item 6 applied. If
  the rule holds: `HomeostaticDrivePool::{bin_descendants, window_descendants}` where
  Context item 5 says, `sum_prev` as `u32`, `count_descendants`, `causal_ratio_q16`,
  `regulate` reading it below the ceiling, the record's `is_well_formed` bounding the counts
  by the spikes, the twelve-window loops pinned again, F-38. If it does not: the record and
  the controller untouched, the counters the executor's. The determinism pin taken from the
  run with the reason (the stamp is in the unit's bytes). The ADR states the message-bit
  budget after this round (bits 20–31), the cost (one branch per drained message, one
  comparison per spike, one atomic per worker per tick), what the in-loop count is and is
  not (the oracle's first-generation rule without the counterfactual: a spike the drive would
  have caused anyway counts as a descendant when a synapse's message reached it within the
  latency), and the reading against gross and net.
- [ ] **The findings and the items.** F-37 in whitepaper §11 (the §11.1 sentence that made
  the target a registry entry against ADR-0031's third gate) Resolved by the waking-day ADR;
  F-38 (the field written by nothing) if taken; §11.1: the item after F-36, the sleep
  placeholders' item and the estimator's item dispositioned; H-9 and H-11 extended with the
  association in the image and the day's reading.
- [ ] **The documents.** Whitepaper 4.13.0: the executive summary's sentence on what exists;
  §1.6 rows (`cortex-reasoning`, `cortex-affect`, `cortex-hippocampus`, `cortex-core`,
  `cortex-homeostasis` if taken; the date); §5.2.1 (the message bit, the stamp, the two
  rules, the period's argument; the unit's table row `[16..24)`); §5.2.2 (format 14, the new
  kinds, the version history, the directive); §5.2.15 (the episode's `symbol`, `bind`; the
  reserved bytes); §5.2.16 (the record's rows if taken; the rate parameter's home); §5.2.23
  (the affect record's bytes and its section); §5.2.30 (`InductionState`, `instantiate`,
  `unbind`, `encode`/`decode`); §6.7 (the loader's new clauses); §6.10 (the loop inside the
  tick); §8.3 (the period and the search parameters in the image); §8.8 rows (STDP: the
  argument; complementary learning: the symbol; criticality: the causal count; compression
  progress: the cadence); §9 three rows; §11 and §11.1; Appendix A rows; Appendix C (M5's
  clause); the glossary (Descendant, Instantiate, Symbol); directives for every "exists"
  sentence (`fn instantiate`, `InductionState`, `fn bind`, `MESSAGE_SYNAPTIC`, `fn
  is_descendant`, `SECTION_CLAUSE`, `FORMAT_VERSION: u32 = 14`) and the old `fn discover`
  directive moved to where the loop now lives. README (the Implemented rows), `CLAUDE.md`
  (the sentence on what exists and what does not), `docs/zh-TW/README.md` (§6 and §11 rows),
  `docs/adr/README.md` (three rows), `CHANGELOG.md` (one entry under Unreleased in the shape
  of brief 024's).
- [ ] **Not adopted, with the reason in the ADR that is closest:** the binding table in the
  image (the term-arena ADR: a scratch persisted is a second store); a section of
  associations (the same ADR: the episode owns what its pattern stands for); the target rate
  as a `REGISTRY` entry (the waking-day ADR: the behaviour gate); a tuned target rate (the
  same ADR: no objective in the tree chooses one); a fractional or a most-recent-input
  attribution (the causal-count ADR: the oracle's own criterion is the one the comparison
  can read); a line for the regression's sums (ADR-0051 stands); a compaction of the arena
  (Specified, its own round).
- [ ] **This brief archived** under `briefs/archive/` with the frozen banner, every box
  dispositioned, the precondition directives removed and the links rebased.

<!-- @assert-absence target="crates/cortex-reasoning" symbol="fn instantiate" reason="precondition: no rule copies a term through the bindings yet; the term-arena ADR of this brief adds one" -->
<!-- @assert-absence target="crates/cortex-core" symbol="MESSAGE_SYNAPTIC" reason="precondition: no message says whether it is a synapse's; the causal-count ADR of this brief adds the bit" -->
<!-- @assert-count target="crates/cortex-connectome" symbol="FORMAT_VERSION: u32 = 13" min="1" reason="precondition: the image format is 13; this brief moves it to 14" -->

## Not empowered

- No image section, no format bump beyond the one this round makes (13 → 14), and no field
  in any record unless the ADR that decides it is written first; the reserved bytes stay
  zero except where an ADR of this round names the field.
- No new crate; no dependency in a state crate; no `unsafe` outside the runtime's arena.
- No float anywhere, including the tests and the oracle.
- No change to the excitatory rule's amounts, the inhibitory rule's form, the window, the
  eligibility time constant, the modulator, `estimate_branching_ratio`, the bin, the window,
  the ceiling, the ripple or the replay drive; the controller's reading changes only if the
  causal rule holds, and then to the causal ratio below the ceiling as the ADR states it.
- No sleep constant moved without the rule of Context item 6 saying so; no target rate chosen.
- No renaming of the CI jobs the ruleset requires; no move of the determinism pin without the
  run's reason.
- No test in the pull request's gate above the nights at 256 units the gate holds today plus
  this round's 256-unit tests; the 1 024-unit day and the 1 024-unit ratios are the weekly
  job's; nothing at 4 096 units this round.
- No claim that a rule at 1 024 units says what the same rule does at Appendix A's scale.
- The registry's gates are not changed; a gate for parameters that change behaviour is a
  decision of its own round, named in the waking-day ADR as what a registry entry would need.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in
the ADR that owns it: the name and layout of the induction record and which of the search's
parameters it holds, the shape of the executor's inputs for terms and clauses, whether the
loop runs before or after the tally, how a search's own error inside the tick is counted, the
form of `instantiate` (copy on need or always) and its depth bound, the section kinds' numbers
within Appendix A's scheme, the harness of the day (its windows, gains and periods, so long as
one period is 20 000 and one is in the regime), the placement of the period's argument in the
rule's signature, the form of the descendant rule so long as it is the oracle's latency
criterion, which bytes of the homeostasis record take the counts if the rule holds, and which
numbers of the re-pinned tests the ADRs restate. It may not reach the standing directives, the
whitepaper's invariants or the constraints in `CLAUDE.md`.

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
reports no dependency in a state crate. The determinism pin moves once, for the stamp, and
the new value is taken from the run and stated in the causal-count ADR. The three precondition
directives above are gone with the archived brief.

## Report

The closing message states: what the image holds now and what the loader refuses, with the
format's history line; how a committed invention is instantiated and what that did to the
pinned lengths of the exit search; the loop inside the tick, its cadence, its cost and its
counters, and what `capture_night` looks like on the engine's own store; the day's readings
at both sizes and both periods (the rates, the drifts, the gain, the stages) beside the
decision on the target's home and the sleep placeholders; the in-loop ratios beside the
oracle's on both priors at both sizes, the rule as written and what it decided about the
record and the controller; the determinism pin's new value and why; what the mutation gate
found on the changed lines and how each survivor was answered; what was not done (the
compaction, standardising apart, a chosen target rate, a gate for behaviour-changing
parameters, anything at 4 096 units) and why; and what the re-examination after the round
recommends next.
