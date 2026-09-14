---
status: archived
date: 2026-09-14
---

> **Executed 2026-09-14 in pull request #62.** Writes ADR-0055 (a weight that settles: the
> excitatory depression scaled by the weight's magnitude), ADR-0056 (a compaction of the term
> arena at every slow-wave onset) and ADR-0057 (the inhibitory rule read from below the rail);
> no finding closed and none opened; image format 14 unchanged; the determinism pin moved once,
> for the depression's amount, with its reason. Every deliverable is done; notes under the boxes
> say where the tree departs from the text (the stabilising rule's criterion held at 1 024 units
> at both periods and at 256 units at one, so §11.1's item stays open with the reading; the
> below-rail reading rule held at one period and the ADR says why the rise it read is not the
> rule's; the night's reclaimed count is pinned as the store test's thirty; the compaction's
> scratch is the executor's work stack). The report is in the pull request and in
> `CHANGELOG.md`. The body below describes the tree before execution and is not maintained;
> its relative links gained one `../`.

# Brief 026 — A weight that settles: the excitatory depression scaled by the magnitude, a compaction of the term arena at slow-wave onset, and the inhibitory rule read from below the rail

> A self-contained prompt for one round of work. Read `briefs/README.md`, then this file, then
> the files it names. Frozen when executed; the outcome lives in the ADRs that accept it, the
> changelog and the code.

## Mission

Take the three steps the closing report of brief 025 recommended, each re-derived against the
tree and two of them redirected by what the tree says. **One ADR** gives every excitatory
weight a fixed point: the pair rule's depression scales with the weight's magnitude (van
Rossum, Bi and Turrigiano 2000; the $\mu = 1$ depression of Gütig et al. 2003, the potentiation
additive as it is), so that under a stationary drive a magnitude settles where a pairing's
depression equals its potentiation instead of draining to nothing, as the day of
[ADR-0053](../../docs/adr/0053-the-waking-day-and-the-target-period.md) read; the rule is measured
on the same day harness at 256 units (sixteen windows, the gain held) and 1 024 units (eighty
windows under the controller, a night inside them) under a criterion written in this brief
before the run, every pin of the reference tests is taken again from the runs, and the
determinism pin moves once, with its reason, if the run pairs a spike the rule now depresses
differently. **One ADR** compacts the term arena: the nodes the clause store does not reach are
reclaimed by a mark in one descending pass (the arena is bottom-up by construction, so a marked
node's children lie below it) and a move in one ascending pass, the store's indices remapped
and the induction record's cursor moved, as a rule of `cortex-reasoning` over its own arena;
the executor runs it between ticks on a caller's call and by itself at every entry into
slow-wave sleep, the reclamation whitepaper §8.8's glymphatic row places there; the image is
unchanged (format 14). **One ADR** reads the inhibitory rule where it can move both ways: the
reference prior with its inhibitory gain at 2.0, so that the inhibitory weights start between a
third and three quarters of the rail instead of on it, a day at 256 and 1 024 units at the two
periods of ADR-0053, a fraction of units at the target per window as the reading a chosen target
needs, under a reading rule written here first; no target is chosen, and the registry's gates
are untouched, since `CLAUDE.md`'s invariant holds the behaviour gate and a lane for parameters
that change behaviour is not a brief's to open. When the round is done: the whitepaper, README,
`CLAUDE.md`, the reader's guide, the ADR index and the changelog say all of this, the format is
14, and this brief is archived with every check green.

---

## Standing directives

- Every claim is Implemented, Specified, Target or Hypothesis. What a test holds on a network
  of 256, 1 024 or 4 096 units is stated as what it is, with the prior's parameters; what the
  same rules do at Appendix A's scale is a Target with the same generator
  ([ADR-0010](../../docs/adr/0010-measured-or-target.md)). No timing figure enters a document from a
  developer machine. A rule is what it does: no "validates", no "autonomous", no "stable"
  without the quantity, the band and the windows it held over; "a day" is the engine's own
  (the pressure's time constant), stated in windows and in simulated seconds, never a clock's.
- The repository wins over the document; a disagreement is a numbered finding in whitepaper
  §11, never a silent edit. An accepted ADR is history: the numbers ADR-0044 to ADR-0054 pinned
  stay in them with a note where this round moves them; the whitepaper and the tests carry
  the numbers as the tree has them after this round.
- No `f32`/`f64`, in the crates and in the tests; a ratio is Q16.16 in `u32`/`i32`, widened to
  `i64` to multiply; every operation on a state field saturates or wraps by name
  (`clippy::arithmetic_side_effects` is denied everywhere,
  [ADR-0029](../../docs/adr/0029-structural-enforcement.md)); a shift amount is bounded a line above
  the shift.
- Every loop ends by construction: a countdown, a range, a scan by `get`, a slice's iterator,
  a recursion whose depth argument falls to a stated bound; never by a comparison alone that
  one operator flip turns into a walk without end. Every walk over the arena visits at most
  `WALK_LIMIT` nodes; a pass over `[0, free)` is a range.
- 64-byte `#[repr(C, align(64))]` records with compile-time assertions; no heap types, threads
  or `unsafe` in a state crate (`unsafe_code = "forbid"`, ADR-0029). A rule of `cortex-core`
  takes what it needs as an argument; it does not read a unit's flag or a policy. A rule of
  `cortex-reasoning` runs over caller-provided slices and a caller-provided scratch.
- Every quantity has one owner ([ADR-0016](../../docs/adr/0016-thirty-two-crate-architecture.md)):
  the pair rule and its constants are `cortex-core`'s, the arena and its compaction
  `cortex-reasoning`'s, the composition of both the executor's
  ([ADR-0023](../../docs/adr/0023-executor.md)); `cortex-immune`'s record is composed by nothing
  and stays so unless an ADR of this round says otherwise. No new crate; the crate count
  stays 32.
- No record changes and the image format stays 14: no field, no section, no reserved byte
  taken. If an ADR of this round finds it must change a record, it says so first and the
  format moves once for the round, with the table in whitepaper §5.2, every fixed offset of
  the image tests and a changelog entry.
- Rule L-3 and §1.5: no word, no string and no language name enters a crate.
- The determinism pin of [ADR-0030](../../docs/adr/0030-verification-governance.md) moves only with
  a stated reason (this round has one candidate: the depression's amount at a pairing whose
  magnitude is not the reference); the mutation gate on the changed lines must pass; a new
  rule carries a test over the lattice of `testkit/prop.rs`; every pinned number an arithmetic
  oracle can produce is computed by that oracle before the test that asserts it is written
  (the depression at each magnitude, a fixed point under a repeated pairing, the nodes a hand-
  built arena reclaims, the target count per window at each period); a number only the engine
  produces is pinned from one run and stated as the engine's.
- A decision rule for a measurement is written in this brief before the run and applied as
  written; what the numbers say beyond it is recorded as a reading, never folded into the rule
  after the fact; no constant of a rule is tuned after the run it was measured in (brief 024's
  lesson, [ADR-0051](../../docs/adr/0051-the-estimator-at-4096-units.md); brief 025's,
  [ADR-0054](../../docs/adr/0054-the-causal-count-inside-the-loop.md)).
- The engine never amends its own code ([ADR-0031](../../docs/adr/0031-policy-amendment.md)); a
  parameter that changes what a run does is in the image or in the trace (§8.3), never in a
  configuration alone; what the engine may amend by itself passes a trial whose behaviour
  hashes are equal, and this round opens no other lane.
- A heavy exit test runs in the weekly job as an ignored test whose name contains
  `exhaustive`; the pull request's gate runs at most what it runs today plus this round's
  tests at 256 units, the two days at sixteen windows the one growth, sized so that the gate's
  runtime tests stay within their present order.
- No product name enters a crate. Conventional Commits with a real body; never commit on
  `main`; the required checks keep their names.

## Context

Re-derived on 2026-09-14 against `main` at `179f436`. Line numbers move; the symbols and the
quoted sentences are what to re-derive.

1. **The excitatory rule and its drain.** `crates/cortex-core/src/dynamics/synapse.rs`:
   `step_stdp(slot, pre_now_tick, post_last_tick, polarity, istdp_alpha_q1_15)` gives an
   excitatory block's trace `window(STDP_A_PLUS_Q1_15, q − p)` when the target's last spike
   $q$ follows the previous presynaptic spike $p$ and takes `window(STDP_A_MINUS_Q1_15, t − q)`
   when $q$ precedes the spike now $t$; both amounts are independent of the weight;
   `STDP_A_PLUS_Q1_15` is 328 and `STDP_A_MINUS_Q1_15` 344 ("depression-dominant at equal
   windows"); `Polarity::magnitude` reads a weight's magnitude under its polarity in
   $[0, 32\,767]$ and `consolidate` keeps the magnitude in $[0, \mathtt{i16::MAX}]$ (ADR-0049).
   The day of [ADR-0053](../../docs/adr/0053-the-waking-day-and-the-target-period.md)
   (`tests/reference.rs::day`) read the excitatory sum at 1 024 units under the controller fall
   from 225.9 million to nothing within fifteen windows, and at 256 units with the gain held
   from 56.7 million to 47.0 million in eight (seventeen per cent), the rate from 9.1 to 6.9 Hz;
   whitepaper §11.1's per-unit scaling item says "a stabilising rule (this row's, or a
   rate-dependent balance of $A_-$ against $A_+$) is the next controller round's subject".
   *Why the drain is the rule's geometry.* Under stationary firing with no correlation between
   a synapse's two units, the depression pairs with the nearest post spike before $t$, which
   is always on record, while the potentiation needs the last post spike to fall after the
   previous presynaptic spike and decays with $q - p$; the expected window factor of the
   depression exceeds the potentiation's at every rate the population reaches, whatever
   $A_- / A_+$ is, so the drift of an additive rule is negative until the floor. *Why §8.8's
   Turrigiano row does not answer it.* The row specifies a gain per unit at `[52..54)` of the
   unit, applied by the turn to the unit's input sums beside the population gain: it
   multiplies what arrives, and a weight at zero times any gain is zero; a rate homeostat on
   the inputs changes nothing in the pairing statistics that drain the weights. It stays
   Specified with that reason. *The literature's answer.* A depression proportional to the
   weight and an additive potentiation (van Rossum, Bi and Turrigiano 2000; Gütig, Aharonov,
   Rotter and Sompolinsky 2003, $\mu = 1$ on the depression; reviewed by Morrison, Diesmann and
   Gerstner 2008): $\Delta w = -A_- f(\Delta t)\, w / w_{\text{ref}}$, so that every weight has
   the fixed point $w^* = w_{\text{ref}} (A_+ / A_-)(P_+ / P_-)$ under its pairing statistics,
   positive and stable, with the documented failure mode that the competition among inputs is
   weak (the distribution stays unimodal). A depression that scales with the magnitude cannot
   reach zero from above (below $w_{\text{ref}} / (2 A_-)$ of magnitude the rounded amount is
   nothing), and an additive potentiation still reaches the rail, which the nights of
   [ADR-0044](../../docs/adr/0044-reference-network.md) and
   [ADR-0048](../../docs/adr/0048-episodes-tagged-from-the-train.md) pin ("every synapse among a
   tagged pattern at the rail after a night"). The form is the one NEST's `stdp_synapse` has
   carried as `mu_minus` since 2007; nothing newer is needed and nothing newer is admissible.
2. **What the pins hold and what moves them.** The reference prior draws excitatory weights in
   $[6\,000, 12\,000]$ (0.18 to 0.37 of the width) and inhibitory ones at the rail
   (`inhibitory_gain_q4_4` 255: `prior.rs` takes the drawn weight times the gain over sixteen,
   at most `i16::MAX`, negated). `tests/reference.rs` pins per window the spikes and the
   in-loop count at three gains on two priors at three sizes, the nights' sums among patterns
   before and after, the days' sums by polarity, the readouts; `tests/sleep.rs` pins a
   replay's weights against an oracle block; `tests/modulation.rs` pins a pairing's trace and
   weight against an oracle block; `tests/differential.rs` pins `PINNED_ARENA_HASH`
   `0x27e12eea1ee625a5` over every unit's image bytes after 95 spikes, with the fan-out
   consolidating every pairing at a modulation of 1.0, so a depression whose amount changed
   moves it if the run pairs a post spike before a presynaptic one at a magnitude that is not
   the reference. The synapse tests' `MINUS` table pins 344, 269, 129, 3 and 0 at five
   intervals for a weight the test builds (`one_synapse`); the lattice property bounds a
   pairing's move by $A_+ + A_-$. The day at 256 units runs eight windows at each period on
   the gate; the day at 1 024 units eighty windows at shift 5 under the controller's step of
   an eighth in the weekly job, its onset at the sixty-sixth window.
3. **The arena's garbage.** `runtime/cortex-runtime/src/discovery.rs::search_from`: a commit
   puts the common clause and the first definition where the two inputs were and the second
   definition at the store's end; the inputs' nodes, the generalisation's intermediate nodes
   and the matching's stay in the arena below `free`, reached by no clause of the store;
   `InduceScratch::restore` reclaims a failed attempt's nodes only (`free` back to the mark).
   `runtime/cortex-runtime/src/store.rs::Induction::new` allocates the arena of `terms` nodes,
   a binding table and a trail of `terms`, a stack of twice `terms`, a pair table and a
   discovery buffer; `terms()` is `[..free]`, `clauses()` the store's indices; the record
   (`InductionState`, `crates/cortex-reasoning/src/state.rs`) keeps `free`, `clauses`, the
   cursor `resume_i`/`resume_j` as *store positions*, `next_variable` and `next_invented` as
   numbers, never indices; `discoveries()` holds the last search's commits with arena indices;
   `set_affect` requires the affect state primed to the store's description length, which a
   compaction leaves as it is (the same clauses, node for node). The arena is bottom-up:
   `admits` refuses a child at or beyond the cursor, the loader refuses the same, and `lgg`,
   `Body::alloc` and `instantiate` allocate children before their parent, so every child's
   index is below its parent's. Between searches the binding table is empty (`unbind` after
   every commit, `restore` after every failure, `prove` restored by its callers).
   `Episode::symbol` (`cortex-hippocampus`) holds a predicate id, not an index. Whitepaper
   §8.8's row "Glymphatic clearance | `cortex-immune` | Reclamation, compaction and checksum
   audit during the slow-wave stage of ADR-0037 | Specified", §8.6's "the immune scrubber runs
   the same walk during slow-wave sleep with compaction and checksum verification
   (Specified)", §5.2.30's "a compaction of the arena's garbage: Specified" and ADR-0052's
   "what is Specified (a compaction of the arena's garbage; standardising apart)".
   `Executor::tally` (`executor.rs`) steps the stage once per window (`step_sleep` after
   `regulate`); `STAGE_AWAKE` 0, `STAGE_SWS` 1, `STAGE_REM` 2 (`cortex-homeostasis`);
   `ripple()` replays in slow-wave sleep on `RIPPLE_CADENCE`. `ImmuneScrubNode`
   (`crates/cortex-immune/src/lib.rs`: `reclamation_active`, `degenerate_synapse_count`, a
   segment id, a health score) is composed by nothing; TC-2 forbids a dependency between
   state crates, so the compaction rule is `cortex-reasoning`'s over its own arena and the
   executor composes it, as it composes every rule.
4. **The inhibitory rule and the registry.** ADR-0053 made the period an image parameter and
   wrote: "a round that chooses one needs an objective over the rate, and if the engine is to
   amend it by itself, a registry gate of its own (option 4), which this ADR names and does
   not make"; whitepaper §11.1's registry item says "a gate for such parameters, with an
   objective the trial measures in both forks (a rate, a length, a cost), is a decision of its
   own round". `CLAUDE.md`'s invariant: "What it may amend by itself is a parameter in
   `cortex-executive`'s `REGISTRY`, through the four gates of `PolicyAmendment` and a trial in
   two forks of the image whose behaviour hashes must be equal"; §8.18: "the engine may
   change what it costs, never what it does... a change to behaviour is the maintainers'
   lane". A brief's empowerment never reaches `CLAUDE.md`'s constraints (`briefs/README.md`),
   so option 4 is not this round's to make; this round says so where the item is and gives
   the maintainers what a chosen target needs, which is the rule read where it can move both
   ways. The rule of Vogels et al. 2011 at an inhibitory synapse onto a target $j$ loses
   $\alpha$ at every presynaptic spike and gains $A_+ f(|\Delta t|)$ at each of the two
   pairings; its balance is at $\rho_j = \rho_0$ with $\alpha = 2 \rho_0 \tau A_+$: above the
   target the inhibition onto $j$ grows, below it weakens. ADR-0053's day read it on a prior
   whose inhibition starts at the rail, where growth has nowhere to go. The same generator
   with `inhibitory_gain_q4_4` 32 (2.0) draws inhibitory weights in $[-24\,000, -12\,000]$, a
   third to three quarters of the rail, and changes nothing else. The executor's train
   (`train_capacity` $2^{22}$ in `config`) holds every spike of the last windows with its unit,
   so a window's count per unit is read from it; the target's count per window is
   $2^{17} / \text{period}$, six at 20 000 ticks and twenty-six at 5 000 (the fine tick is
   10 µs; a window is 1.31 s). `day(prior, windows, step, shift, period)` returns per window
   `(spikes, gain, estimate, stage, descendants, inhibitory_sum, excitatory_sum, replays)`;
   `weights_by_polarity` sums the arena; the cluster of twelve is tagged at the start.
5. **The decision rules, written before the runs.**
   - *The stabilising rule.* The rule's form is adopted before the run as Context item 1
     states it, with one constant added, the reference magnitude at which the depression is
     $A_-$; the criterion decides what the ADR says of it and whether §11.1's item closes,
     never whether the constant moves. The criterion holds when, at both periods: at 256
     units with the gain held over sixteen windows, the excitatory sum after the sixteenth
     window is above a quarter of the sum before the first, and each of the last four windows
     changes it by less than two per cent of its value after the twelfth; at 1 024 units under
     the controller over eighty windows at shift 5, the sum after the eightieth is above a
     quarter of the sum before the first, and every one of the last sixteen windows' sums is
     within ten per cent of the sum after the sixty-fourth (the night lies inside them, so the
     band includes what a night's replay does to one cluster). Then the item is Resolved with
     the reading. Otherwise the rule stays (a magnitude it depresses cannot reach zero from
     above, so the drain it answers cannot recur), the item stays open with the reading, and
     the ADR states which clause failed and by how much, with no constant moved.
   - *The compaction.* No measurement rule; the equalities hold by construction and are
     asserted (the description length before and after, every proof at the same step count,
     the store's clauses hashing the same, a second compaction reclaiming nothing); the counts
     a run produces (the nodes the exit store reclaims after its two commits, the nodes a
     night reclaims) are pinned from the run.
   - *The inhibitory rule from below the rail.* The reading is "the rule moves the population
     toward the target" at a period when, at 256 units with the gain held over sixteen
     windows, the fraction of units at the target (Context item 4: a unit's count in the
     window within a factor of two of the target's count, inclusive both ways) averaged over
     the last four windows exceeds the average over the first four by at least 0.1 (Q16.16
     6 554), and the inhibitory sum after the sixteenth window is neither at the rail (the
     inhibitory synapses' count times 32 767) nor below a tenth of the sum before the first.
     Otherwise the reading is stated as what it is. No target is chosen either way; the ADR
     states what each period did to the rates, to the fraction at the target and to both
     sums at both sizes, and what the maintainers have after this round to choose one with.
6. **Where the round's numbers go.** The next ADR is the next free number (`ls docs/adr`); the
   whitepaper moves 4.13.0 → 4.14.0; the format stays 14 and the directive
   `FORMAT_VERSION: u32 = 14` stands; §11.1's per-unit scaling item, its item after F-36 and
   its registry item gain their dispositions; a finding takes the next number of §11 only if
   the re-derivation finds a disagreement between a document and the tree (none is known at
   writing); H-9 and H-11 gain what the nights and the arena do under this round; the
   property kit is `testkit/prop.rs`; the mutation gate's exclusions are in
   `.cargo/mutants.toml`; the weekly job runs
   `cargo test --workspace --release --locked -- --ignored exhaustive`.
7. **What is not in the tree that a reader might assume.** No rule of `cortex-core` reads a
   weight into a pairing's amount; no function reclaims an arena node the store does not
   reach; nothing runs at slow-wave onset but the ripple's cadence; no prior of the reference
   tests draws inhibition below the rail; no reading counts the units at a target; nothing
   writes `[52..54)` of the unit; the registry holds the sweep's two parameters and no other.


## Deliverables

- [x] **The stabilising-rule ADR (the next free number)** (`depends-on: ADR-0053`; ADR-0022,
  ADR-0032, ADR-0036, ADR-0044 and ADR-0049 named). In `cortex-core`:
  `STDP_DEPRESSION_REFERENCE_Q1_15` (0x2000, a quarter of the width, inside the reference
  prior's band: the magnitude at which an excitatory depression is `STDP_A_MINUS_Q1_15`) and
  the shift that divides by it, tied by a `const` assertion; `step_stdp`'s excitatory
  depression becomes $\operatorname{round}(\text{window}(A_-, t - q) \cdot M / 2^{13})$ with
  $M$ the slot's magnitude under its polarity before the pairing (so 344 at the reference,
  1 376 at the rail, nothing below a magnitude the rounding takes to zero), saturating; the
  potentiation, the inhibitory branch, the decay, the consolidation and every signature
  unchanged; the module doc, the constants' docs and the rule's doc say the form and its
  fixed point. Tests, every number from an oracle first: the `MINUS` table at the reference
  magnitude; the depression at the rail, at zero, at the largest magnitude the rounding takes
  to zero and one above it; a weight's fixed point under a repeated pairing at a fixed
  interval from below and from above, computed by an integer oracle of the balance and
  reached within two LSB from both sides; the lattice property that a pairing moves a
  magnitude by at most $A_+ + 4 A_-$ within its half of the width and that the depression is
  non-decreasing in the magnitude; every exact-step test whose number the oracle moves,
  moved with the oracle's number. In the runtime: no code changes; the oracle blocks of
  `tests/modulation.rs` and `tests/sleep.rs` pair at the same magnitude the engine's block
  holds; `tests/differential.rs`'s pin taken from the run if it moves, with the reason;
  `tests/reference.rs`: the day at 256 units extended to sixteen windows at each period, the
  criterion of Context item 5 applied to both days, every pin of the estimator's windows,
  the nights and the days taken again from the runs, the nights' "at the rail" restated as
  the runs give it. The ADR's tables: per size and period the excitatory sum's course, the
  rate per unit, the gain and the stages; the nights' sums among the patterns before and
  after under the new rule beside ADR-0048's; the fixed point the oracle gives at the
  population's typical intervals; §11.1's per-unit scaling item dispositioned; §8.8's STDP
  row carrying the equation; §8.8's Turrigiano row Specified with the reason a gain cannot
  answer a drain.
  **Departure:** ADR-0055. The criterion as written did not hold in full: the 1 024-unit clause
  held at both periods (the sum settles at 0.45 of the prior's within 0.75 per cent over the
  last sixteen windows, a night inside them) and the 256-unit clause held at the 20 000-tick
  period and failed at 5 000 by 0.27 points in one window (the sum still falling by 1.9 per
  cent per window at the sixteenth, decelerating), so the rule stands, no constant moved, and
  §11.1's item stays open with the reading; the reference magnitude is `0x2000`, a quarter of
  the width; the fixed-point test walks into an oracle band rather than a point, since the
  rounding makes a plateau.
- [x] **The compaction ADR (the number after it)** (`depends-on: ADR-0052`; ADR-0037,
  ADR-0041 and ADR-0045 named). In `cortex-reasoning`: `compact(arena, free, roots, forward)
  -> Result<Compaction, CompactError>` over the caller's arena, its cursor, its roots (the
  store's indices, remapped in place) and a scratch of at least `free` entries: the mark in
  one descending pass over `[0, free)` (a root is marked first; a marked node marks its
  children, which lie below it, so one pass is complete), the move in one ascending pass (a
  live node takes the next free slot, its children remapped through the scratch, which are
  already moved), the dead tail zeroed; `Compaction { live, reclaimed }`; refused, nothing
  changed, for a root at or beyond `free` (`Root`), a child at or beyond its parent
  (`NotBottomUp`) and a scratch shorter than `free` (`Scratch`); the bindings are not an
  input, since a caller compacts an arena whose table is empty and the doc says so. Tests:
  garbage below, between and above live nodes; a subterm shared by two roots kept once; a
  store of no roots empties the arena; the counters and the roots after; every refusal
  clause; the exit store of ADR-0045 after its two commits compacted with the reclaimed count
  pinned from the run, the description length equal, every goal provable before provable
  after at the same step count, the clauses hashing the same (`term_hash`) in the new order,
  a second compaction reclaiming nothing; the lattice property over seeded bottom-up arenas
  of up to sixty-four nodes with seeded roots (every root's `size` and `term_hash` unchanged,
  `live + reclaimed == free`, idempotent). In the runtime: `Induction::compact()` with the
  executor's own scratch as the table (the stack, empty between walks), the record's `free`
  moved, the last search's discoveries cleared (their indices moved), the affect state left
  as it is with the reason (the length is the same, node for node), the counters
  `compactions` and `reclaimed`; `Executor::compact() -> Result<Compaction, CompactError>`
  between ticks (`NoArena` for an engine without one); inside the tick, in `tally` after
  `step_sleep`, one compaction when the stage moved from awake or REM into slow-wave sleep
  (§8.8's glymphatic row: reclamation during the slow-wave stage), its outcome counted and
  never returned; `Executor::{compactions, reclaimed}`. Tests: `tests/store.rs`: a
  compaction between ticks after the loop committed, with the reclaimed count pinned and the
  store's clauses hashing the same; a search after a compaction resuming from the cursor; an
  image written after a compaction loading and searching alike; a night's onset compacting
  once and the second slow-wave bout of the same night not compacting again... unless the
  stage left slow-wave sleep in between, in which case it does, stated; `tests/no_alloc.rs`
  through a night; `tests/reference.rs`: `capture_night` pins the night's reclaimed count and
  holds every index-based reading by hash where the index would move; the determinism test
  through a night on one and four workers (the compaction inside the tick bit-identical). The
  ADR states the cost (two passes over `free` nodes at a window boundary, bounded by the
  arena's capacity), why one descending pass marks (the bottom-up invariant), why the trigger
  is the slow-wave onset and not a cadence or a full arena (a full arena mid-search is
  `ArenaFull`, an error the loop counts, and the next night reclaims; a cadence would compact
  a live arena while awake for nothing), what the image does with it (nothing: the loader
  reads a smaller arena), and what stays Specified (standardising apart; the checksum audit;
  the `cortex-immune` record's composition).
  **Departure:** ADR-0056. `CompactError::{Root, NotBottomUp, Scratch}`; the executor's scratch
  is the work stack, twice the arena's size; `Executor::compact` returns `TermError` (`NoArena`
  for an engine without one, `Malformed` for a refusal the engine's own arena cannot cause) so
  that the inputs' error type stays one; the rule refuses a cursor beyond the arena as
  `Scratch`; a night of two slow-wave bouts compacts twice, stated; the capture nights' reclaimed
  count is the store test's thirty (the same store, the same two commits); the no-alloc test's
  two bodies are serialised, since the counter is the process's.
- [x] **The below-the-rail ADR (the number after that)** (`depends-on: ADR-0053`; ADR-0031,
  ADR-0044 and ADR-0049 named). In `tests/reference.rs`: `prior_below_rail(units)`, the
  reference prior with `inhibitory_gain_q4_4` 32 and nothing else changed; `day` returning a
  ninth element, the fraction of units at the target in the window in Q16.16 (Context item
  4's rule from the train), so that every day of this round reads it; a gate test at 256
  units on the below-rail prior at both periods over sixteen windows with the gain held; an
  `exhaustive` test at 1 024 units at shift 5 under the controller's step of an eighth at
  both periods over eighty windows; every number pinned from the run; the reading rule of
  Context item 5 applied as written. The ADR's tables: per size and period the rate per unit,
  the fraction at the target, the inhibitory and the excitatory sums per window and the
  stages; the reading's outcome at each period; the disposition of §11.1's item after F-36
  (the rule read from below the rail) and of its registry item (the invariant: no lane where
  the hashes differ, which is `CLAUDE.md`'s and ADR-0031's; what the maintainers have after
  this round to choose a target with, and where that choice would go: the default of
  `Config::istdp_target_period_ticks` and the image). No change to `Prior`, to the rule, to
  the registry or to the configuration's default.
  **Departure:** ADR-0057. The reading rule held at the 20 000-tick period at 256 units and
  failed at 5 000; the rise it read at the slower period is read on the prior at the rail too,
  where the inhibitory rule cannot grow, so the ADR states it as the rate's fall under
  ADR-0055's drift and not the rule's work, which the rule as written did not distinguish;
  the priors' sums before any window are pinned by a test of their own.
- [x] **The findings and the items.** §11.1: the per-unit scaling item dispositioned by the
  stabilising-rule ADR under the criterion; the item after F-36 extended with the reading
  from below the rail; the registry item restated under the invariant; H-9 and H-11 extended
  with what the nights do under the new rule and what the arena holds after a night; a
  finding only if the re-derivation finds one.
- [x] **The documents.** Whitepaper 4.14.0: the executive summary's sentence on what exists;
  §1.6 rows (`cortex-core`, `cortex-reasoning`; the date); §5.2.1 (the constant and the
  rule's form in the STDP paragraph and the public API); §5.2.13 (the immune row's status:
  the arena's compaction is the runtime's over `cortex-reasoning`'s rule, the record still
  composed by nothing); §5.2.30 (`compact`, `Compaction`, `CompactError`; the Specified list
  without the compaction); §6.6 (R-6: the arena compacted at slow-wave onset, Implemented);
  §6.10 (the loop's arena reclaimed); §8.6 (the compaction the runtime does); §8.8 rows (STDP:
  the equation with $w / w_{\text{ref}}$; Turrigiano: Specified with the reason; glymphatic:
  the arena's compaction Implemented, the rest Specified; complementary learning: the nights
  under the new rule; sparse networks: the below-rail gain measured); §9 three rows; §11.1;
  Appendix C (M5's clause); the glossary (Compaction; Reference magnitude); directives for
  every "exists" sentence (`STDP_DEPRESSION_REFERENCE_Q1_15`, `fn compact`, `fn compactions`)
  and the Specified sentences that lose their subject moved. README (the Implemented rows),
  `CLAUDE.md` (the sentence on what exists and what does not: the stabilising rule and the
  compaction move from "do not" to "exist"), `docs/zh-TW/README.md` (§6, §8 and §11 rows),
  `docs/adr/README.md` (three rows), `CHANGELOG.md` (one entry under Unreleased in the shape
  of brief 025's).
- [x] **Not adopted, with the reason in the ADR that is closest:** the per-unit gain at
  `[52..54)` (the stabilising-rule ADR: a gain cannot lift a weight from zero; it stays
  Specified with its precondition, the measurement of H-8); a weight-dependent potentiation
  ($\mu$ on $A_+$: the nights' rail is a pinned property, and the competition it would cost
  is the documented failure mode of $\mu = 1$ on both sides); a retuned $A_-$ or $A_+$ (the
  amounts stay; the reference magnitude is the one constant added); a rate-dependent balance
  of $A_-$ against $A_+$ (a rate homeostat under an external drive above its target drains the
  recurrent weights by design, the reading of ADR-0053 at 1 024 units); a compaction on a
  cadence, on `ArenaFull` or of the binding table (the compaction ADR); standardising apart
  (Specified, its own round); the `cortex-immune` record composed (nothing reads it); a
  gate class for parameters that change behaviour (`CLAUDE.md`'s invariant; the below-the-rail
  ADR names where the item stands); a chosen target rate (no objective in the tree chooses
  one; the reading is what a choice needs); a change to `Prior`'s fields (the below-rail
  network is the same generator at another gain); anything at 4 096 units.
- [x] **This brief archived** under `briefs/archive/` with the frozen banner, every box
  dispositioned, the precondition directives removed and the links rebased.

## Not empowered

- No record changes, no section, no format bump, no reserved byte taken; `[52..54)` of the
  unit stays reserved and zero.
- No new crate; no dependency in a state crate; no `unsafe` outside the runtime's arena.
- No float anywhere, including the tests and the oracles.
- No change to `STDP_A_PLUS_Q1_15`, `STDP_A_MINUS_Q1_15`, the window, the eligibility time
  constant, the modulator, the inhibitory rule's form or its period's bounds, the potentiation's
  form, `estimate_branching_ratio`, the bin, the window, the ceiling, the sleep constants, the
  ripple or the replay drive; the reference magnitude is not tuned after the run.
- No compaction of anything but the term arena; no trigger but a caller's call and the
  slow-wave onset.
- No change to `Prior`; no registry gate; no target rate chosen; no change to the
  configuration's default period.
- No renaming of the CI jobs the ruleset requires; no move of the determinism pin without the
  run's reason.
- No test in the pull request's gate above the nights at 256 units the gate holds today plus
  this round's 256-unit tests (the two days at sixteen windows); the 1 024-unit days are the
  weekly job's; nothing at 4 096 units this round.
- No claim that a rule at 1 024 units says what the same rule does at Appendix A's scale.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in
the ADR that owns it: the name of the reference magnitude and its value so long as it is a
power of two inside the reference prior's band (so that the division is a shift and the rule
at the prior's weights is near the rule as it was); whether the depression rounds to nearest
or floors; the compaction's shape (which scratch is the table, whether the roots are remapped
in place or through a second slice, whether the dead tail is zeroed by the rule or by the
caller) so long as the mark is one pass and the arena stays bottom-up after; the trigger's
placement in `tally` so long as it is the slow-wave onset once per bout; the harness's window
counts so long as the 256-unit days hold sixteen and the 1 024-unit days a night; the
objective's factor and its form so long as it is per unit against the period's count; the
below-rail gain so long as the drawn magnitudes lie between a quarter and three quarters of
the rail; and which numbers of the re-pinned tests the ADRs restate. It may not reach the
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
reports no dependency in a state crate. The format stays 14. The determinism pin moves at most
once, for the depression's amount, with the new value taken from the run and stated in the
stabilising-rule ADR. The two precondition directives above are gone with the archived brief.

## Report

The closing message states: the rule as it is now, its fixed point at the population's
intervals, and the days' readings at both sizes and both periods (the excitatory sum's course,
the rate, the gain, the stages) beside the criterion as written and its outcome, with §11.1's
item's disposition; every pin that moved and why (the nights' sums, the estimator's windows,
the differential pin); the compaction (its two passes, its trigger, its cost, the exit store's
reclaimed count and a night's) and what the image does with it; the inhibitory rule's readings
from below the rail beside the fraction at the target at both periods, the reading rule as
written and its outcome, with no target chosen and what the maintainers have to choose one
with; what the mutation gate found on the changed lines and how each survivor was answered;
what was not done (the per-unit gain, standardising apart, the immune record's composition, a
chosen target rate, a gate for behaviour-changing parameters, anything at 4 096 units) and why;
and what the re-examination after the round recommends next.
