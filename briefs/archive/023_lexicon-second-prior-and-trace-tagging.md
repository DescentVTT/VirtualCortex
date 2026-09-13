---
status: archived
date: 2026-09-13
---

> **Executed 2026-09-13 in pull request #56.** Writes ADR-0046 (the lexicon), ADR-0047 (the
> second prior and the estimator's regime) and ADR-0048 (episodes tagged from the train);
> records findings F-35 and F-36; dispositions H-9's open item and H-11's synaptic half at
> 256 and 1 024 units and resolves the estimator's open question without a record line; image
> format 13 and the determinism pin untouched. Every deliverable is done; notes under the
> boxes say where the tree departs from the text (a nominal shape beside the noun; the
> reward's pattern is the densest basal time constant of the ripple, not the ripple's
> ranking; the regression rule lives in `cortex-homeostasis`, the exit tests in
> `tests/reference.rs`). The report is in the pull request and in `CHANGELOG.md`. The body
> below describes the tree before execution and is not maintained; its relative links gained
> one `../`.

# Brief 023 — The runtime lexicon on ids, a second anatomical prior and the estimator's regime, episodes tagged from the trace; H-9's open item and the synaptic half of H-11 decided at a stated scale

> A self-contained prompt for one round of work. Read `briefs/README.md`, then this file, then
> the files it names. Frozen when executed; the outcome lives in the ADRs that accept it, the
> changelog and the code.

## Mission

Answer an architectural proposal ("The Runtime Lexicon, Second Network Prior Dynamics,
Autonomous Trace-Driven Episodic Tagging, and Closing the Synaptic Loop of H-11", received
2026-09-13) with decisions, code, measurements and tests instead of a document that drifts.
The proposal follows the sequence the closing report of brief 022 recommended and names three
pillars: a bidirectional lexicon between external token ids and the language stream of
[ADR-0040](../../docs/adr/0040-categorial-reduction.md); a second anatomical prior on which the
lag-one estimator of [ADR-0036](../../docs/adr/0036-criticality-control.md) is measured again, and
a decision on whether a smoothed or multistep estimate needs a line of the record; and episodes
tagged from the network's own spike train rather than placed, a night, a readout, and the same
capture bound to a rewarded invention so that the synaptic half of H-11 has a rule. Four of its
premises are not in the tree (the Context says which), so the round begins by re-deriving them.
When the round is done: **one ADR** gives the runtime a lexicon that is a caller's table of
token ids to concept ids with a lexical shape, instantiates a token's category on the term
arena with fresh variables, reads a token sequence into a frame through `comprehend` (a token
that is one of the lexicon's speech-act markers sets the act, one that is a particle sets the
marker), and realises a complete frame back into token ids in the template's order, descending
into a nested frame at `ROLE_CHILD`, with the act's markers, the register's variant and the
prosody particle, writing `surface_token_id`, which no rule wrote before; a modifier's own
concept reaches the affect role through a role term that names its filler. **One ADR** measures
the estimator on a second prior, the sparse random network with local delays (the same
generator at the widest window), beside the lattice of [ADR-0044](../../docs/adr/0044-reference-network.md):
the per-window slope and the causal ratios as before, and, caller-side from the open bin's
count sampled between ticks, the lag-$k$ slopes at a bin near the generation time, so that the
question "is the noise the window's or the bin's" is answered with numbers; it decides whether
the record needs a second line and says what that line holds. **One ADR** gives
`cortex-hippocampus` the two rules that tag an episode from a spike train (the densest span of
a ripple's length, and the pattern of the units that fired most within a span), gives the
runtime their composition with the ledger and with a rewarded invention (the pattern active in
the ripple before the reward, tagged and associated with the invented predicate's id in the
caller's table), and holds on the reference network what a night does to a pattern the rule
found in an experience, to one it found in the background drive, and to the invention's, each
with a spiking readout. Finding F-35 records the proposal's premises the tree contradicted;
H-9's open item and H-11's synaptic half are dispositioned at the stated scale; the format
stays 13 and the determinism pin is untouched. The whitepaper, README, `CLAUDE.md`, the
reader's guide, the ADR index and the changelog say all of this, and this brief is archived
with every check green.

---

## Standing directives

- Every claim is Implemented, Specified, Target or Hypothesis. What a test holds on a network
  of 256 or 1 024 units is stated as what it is, with the prior's parameters; what the same
  rules do at Appendix A's scale is a Target with the same generator
  ([ADR-0010](../../docs/adr/0010-measured-or-target.md)). No timing figure enters a document from
  a developer machine. The words "autonomous", "embodiment", "cementing", "validates" and
  "reference-scale" (for anything below Appendix A's counts) describe the proposal, never the
  tree: a rule is what it does.
- The repository wins over the document; a disagreement is a numbered finding in whitepaper
  §11, never a silent edit.
- No `f32`/`f64`, in the crates and in the tests; a ratio is Q16.16 in `u32`/`i32`, widened to
  `i64` (or `i128` for a regression's products) to multiply; every operation on a state field
  saturates or wraps by name (`clippy::arithmetic_side_effects` is denied everywhere,
  [ADR-0029](../../docs/adr/0029-structural-enforcement.md)); a shift amount is bounded a line
  above the shift.
- Every loop ends by construction: a countdown, a range, a scan by `get`, a slice's iterator;
  never by a comparison alone that one operator flip turns into a walk without end (the
  lesson of brief 022's mutant timeout).
- 64-byte `#[repr(C, align(64))]` records with compile-time assertions; no heap types, threads
  or `unsafe` in a state crate (`unsafe_code = "forbid"`, ADR-0029); a rule over a spike train
  takes the train as a caller's slice.
- Every quantity has one owner ([ADR-0016](../../docs/adr/0016-thirty-two-crate-architecture.md)):
  a rule that tags an episode from a spike train is `cortex-hippocampus`'s; an anatomical prior
  is `cortex-connectome`'s; the lexicon, which instantiates a `TermNode` category and reads a
  `LinguisticFrameSlot`, is the runtime's under TC-2, whatever the proposal's "or
  `cortex-linguistic`"; the composition with the ledger, the reward and the forks is the
  runtime's ([ADR-0023](../../docs/adr/0023-executor.md)). No new crate; the crate count stays 32.
- No new record and no field in a record this round: `Episode`'s reserved bytes stay zero, so
  the format stays 13 (rule L-6). An association between an invented predicate and an episode
  is the caller's table, as the clause store, the codebook and the affect state are
  ([ADR-0043](../../docs/adr/0043-discovery-path.md)); a field the loop never reads is the class
  finding F-30 named.
- Rule L-3 and §1.5: a token is an id the host chose; no word, no string and no language name
  enters a crate. The exit test's word table exists in the test alone, as brief 020's did.
- No change to `estimate_branching_ratio`, `regulate`, the bin, the window, the ceiling, the
  ripple, the replay drive or the sleep constants: this round measures them on a second prior
  and says what it saw; a change is the ADR after the measurement.
- The engine never amends its own code ([ADR-0031](../../docs/adr/0031-policy-amendment.md)); a
  capture's window, a search's budget and a tag's priority are the caller's arguments this
  round.
- The determinism pin of [ADR-0030](../../docs/adr/0030-verification-governance.md) moves only
  with a stated reason; the mutation gate on the changed lines must pass; a new rule carries a
  test over the lattice of `testkit/prop.rs`; every pinned number an arithmetic oracle can
  produce is computed by that oracle before the test that asserts it is written; a number only
  the engine produces is pinned from one run and stated as the engine's, as the determinism
  pin is.
- A heavy exit test runs in the weekly job as an ignored test whose name contains
  `exhaustive`; the pull request's gate runs at most one more night and a few windows at 256
  units (the Context sizes it).
- No product name enters a crate. Conventional Commits with a real body; never commit on
  `main`; the required checks keep their names.

## Context

Re-derived on 2026-09-13 against `main` at `e125aaf`. Line numbers move; the symbols and the
quoted sentences are what to re-derive.

1. **The language stream and the field no rule writes.** `runtime/cortex-runtime/src/language.rs`:
   `comprehend(categories, scratch, sentence, template, speech_act, politeness)` reduces a
   sequence of category term indices and binds, for every reduction whose role term is a
   constant in the band `ROLE_CONCEPT_BASE | role_bit` (`role_concept`, `role_of_concept`,
   `role_slot`, `ROLES`), that role to `head(argument)`; a modifier category `X/X` or `X\X`
   therefore fills a role with the head of the phrase it modifies, never with its own concept,
   so no shape of category puts a hedge or a tag into the affect role today (the epistemic
   template's hedge is bound by a caller). `encode_frame`, `read_role`, `decode_frame` are Layer
   1. `runtime/cortex-runtime/tests/language.rs`: a table `WORDS` of seven `(word, concept,
   Part)` with `Part::{Determiner, Noun, Transitive, Intransitive}`, an `Arena` that builds a
   category per part with fresh variables (`NP(X)/N(X)` with a role constant outside the band's
   nibble, `N(c)`, `(S(c)\NP(A):subject)/NP(P):object`, `S(c)\NP(A):subject`), atom functors
   `S` 0x1000, `NP` 0x1001, `N` 0x1002, and `render` walking `realisation_order`. Every
   shape a lexicon needs is in that test and nowhere in the crates. `crates/cortex-linguistic/src/lib.rs`:
   `surface_token_id` "Surface token the lexicon last realised" is written by no rule
   (`grep -rn surface_token_id`: the definition and the whitepaper's table); `politeness_level`
   "the lexicon reads it"; `GATE_PARTICLE_OPEN` "the lexicon may emit the marker";
   `realisation_order()` yields the bound roles and `ROLE_CHILD` in the template's order,
   `child_frame_idx` the frame to descend into; `PROSODY_*` six markers, `SPEECH_ACT_*` four
   acts, `POLITENESS_FORMAL` 2; `bind_child` refuses a self-child and a parent-child, so a
   cycle of length two cannot be built but a longer one through three frames can. Whitepaper
   §5.2.20's status: "the lexicon (Chinese and English) ... Specified"; §6.9's heading: "the
   lexicon Specified"; §8.8's construction rows: "the descent and the lexicon Specified". A
   lexicon that instantiates a category is a rule over `TermNode`, which `cortex-linguistic`
   cannot name (TC-2, `scripts/check-deps.mjs`), so it is the runtime's. Part of F-35.
2. **The estimator's bin against the delays.** `crates/cortex-homeostasis/src/lib.rs`:
   `ACTIVITY_BIN_SHIFT` 12 ("above the wheel's horizon (2 560 ticks), so that every direct
   descendant of a bin's spikes lands in that bin or the next"), `ACTIVITY_WINDOW_SHIFT` 5;
   `estimate_branching_ratio` is the lag-one least-squares slope over the window's pairs with
   an intercept, a negative slope 0, no estimate for a constant window; `regulate` reads it or
   `SIGMA_MAX_Q16` at the ceiling and clears the sums; `bin_activity` is the open bin's running
   count, and the executor's `homeostasis()` reads it between ticks, so a caller can sample
   the population's count at any interval that divides the bin (the tally closes the bin on
   `BIN_CADENCE`, `executor.rs::tally`, setting `bin_activity` to 0 and `last_activity` to the
   closed count). The prior of [ADR-0044](../../docs/adr/0044-reference-network.md) has local
   delays of 100 to 300 ticks and far ones of 1 400 to 2 559, and a unit fires within about
   12 to 128 ticks of a message (`LATENCY` in `tests/reference.rs`): one generation is a few
   hundred ticks and every generation of a cascade completes inside one bin of 4 096, so the
   slope of bin against previous bin is the autocorrelation at ten to forty generations, which
   is $m^{\Delta/\tau}$ for a branching process (Wilting and Priesemann 2018 assume a bin near
   the generation time). ADR-0044's table (0.659 then 0.000 at one gain; 0.560 then 0.032)
   and its open question in §11.1 ("a smoothed estimate across windows ... needs a line the
   record does not have") are what this round measures against a bin the caller samples.
   `crates/cortex-connectome/src/prior.rs`: `Prior::is_well_formed` needs `window >= 1 &&
   window < units`; a local target is a place in `1..=2W` (`between(1, window × 2)`), the
   first `W` to the right; at `window = units / 2 - 1` every unit but the source and its
   antipode is a target with equal probability under the local delay band, and at
   `rewire_q0_8 = 0` no far band is drawn: the sparse random network of Brunel (2000) with
   short delays is the same generator at its widest window, and no code in the crate changes
   for the topology. `tests/reference.rs::windows` already reads `last_activity` after each
   bin. `HomeostaticDrivePool` has no reserved byte (ADR-0037 took `[28..64)`).
3. **The ledger, the trace and what "tagged from the trace" can mean.**
   `crates/cortex-hippocampus/src/lib.rs`: `Episode { tagged_tick, tag, replays, len, _pad,
   pattern: [u32; 12], _reserved: [u8; 8] }` (the proposal's "`units: [u32; 12], count, tag`"
   names fields the record does not have), `Episode::tag(tick, units, priority)`,
   `PATTERN_MAX` 12, `RIPPLE_SHIFT` 11; `Executor::tag_episode(units, priority)` between ticks,
   refused without `Config::episodes`. `runtime/cortex-runtime/src/branching.rs`: `fork(image,
   config, drive, kicks, ticks)` returns the sorted `(tick, unit)` train of a run from the
   image; `trace(exec)` consumes the executor (`WorkerReport::spikes` exists at shutdown
   only), and each worker's `spiked` list of a tick is its own thread's (`executor.rs`), so a
   capture that runs inside the tick and is bit-identical on every worker count needs a
   merge across workers the executor does not have: an executor change, which this round
   specifies and does not make. `Image::encode` refuses a non-quiescent engine (`image.rs`),
   so the train of a waking run is a fork's, from the image the run started at, under the
   same drive and the same inputs, which is the same run (§8.3). Whitepaper §11.1 H-9: "the
   protocol: a reference image, episodes tagged from its own activity (a pattern the trace
   shows firing within a ripple's length)"; the open item after brief 022: "whether a pattern
   the network's own activity produces (tagged from the trace) is of the completing kind".
   [ADR-0037](../../docs/adr/0037-sleep-regulation.md) adopted no weight downscaling in slow-wave
   sleep ("a weight-downscaling sweep is not adopted", §6.6); the proposal's "SWS downscaling"
   is not in the tree. Part of F-35.
4. **The synaptic half of H-11 and the id of an invention.** [ADR-0045](../../docs/adr/0045-clause-search.md):
   "the rule that would close it writes an `Episode` from an invention (the pattern active
   when the reward came, tagged for replay), which is a decision for a round that has a
   representation of an id in the population". `crates/cortex-reasoning/src/induce.rs`:
   `INVENTED_BASE` 0xFFFE_0000, `INVENTED_LIMIT` 0xFFFF_0000; `Invention::predicate`.
   `runtime/cortex-runtime/src/discovery.rs`: `search(store, len, scratch, affect, budget,
   out) -> SearchReport { attempts, commits, rejections, length_before, length_after,
   reward_total_q16 }`, `Discovery { invention, length_before, length_after, valence_q16,
   reward_q16 }`; `tests/discovery.rs` holds the store of twenty-four clauses (106 nodes → 100,
   two commits, rewards of three quarters) and the modulator's coupling on a two-unit network.
   `Executor::reward` adds to the dopamine signal, which `NeuromodulatorState::modulation`
   clamps with the baseline to $[0, 1]$ and `decay_dopamine` lowers by $2^{-14}$ per tick: at
   the reference network's baseline of 1.0 a reward changes no weight, and the consolidation
   a reward gives a pending trace is the two-unit test's, not this round's. The representation
   this round gives an id in the population is the episode tagged from the pattern active in
   the ripple before the reward, associated with the id in the caller's table.
5. **The executor's inputs, bounds and quiescence.** As brief 022's item 4: `Config {
   workers, units, blocks, deltas, nodes_per_worker, deque_capacity, injector_capacity,
   trace_capacity, amendments, modulation_baseline_q16, control_step_q0_16, sleep_shift,
   episodes }`; `arenas_mut`, `injector`, `reward`, `tag_episode`, `wake`, `sleep_stage`,
   `homeostasis`, `episodes`, `replays`, `depotentiations`, `is_quiescent`; the production
   wheel `Executor<2048>`; the replay drive two messages of 1.25; the night's stages at
   `sleep_shift` 5 from a pressure of 1.0 (`tests/reference.rs::reload_with` patches the
   record through the image); `tests/reference.rs` holds `prior`, `drive`, `config`,
   `at_gain`, `synapses_of`, `windows`, `attribution`, `among`, `readout` and `night` as
   file-local helpers, which this round moves into `tests/common/mod.rs` so that three exit
   files share them.
6. **Sizing, from a developer machine (not admissible, not written anywhere but here).** The
   night at 256 units (`tests/reference.rs`, eight windows) is about 35 s in the debug profile;
   a window at 256 units about 5 s; CI's mutation gate ran 256 mutants with that night in the
   runtime's suite in eighteen minutes. The gate may take one more night at 256 units and a
   handful of windows on the second prior; the sweeps at 1 024 units and the three-gain form at
   256 are ignored `exhaustive` tests.
7. **Where the round's numbers go.** The next ADR is 0046 (`ls docs/adr`); the next finding
   F-35 (whitepaper §11); H-9 and H-11 gain dispositions in §11.1 and the estimator's open
   question a resolution or a narrowing; the next reference 78 (Appendix D); the whitepaper
   moves 4.10.0 → 4.11.0; the property kit is `testkit/prop.rs`; the mutation gate's exclusions
   are in `.cargo/mutants.toml`; the determinism pin is `PINNED_ARENA_HASH` in
   `runtime/cortex-runtime/tests/differential.rs`; the weekly job runs `cargo test --workspace
   --release --locked -- --ignored exhaustive`.
8. **The proposal's premises the tree contradicts (F-35).** The lexicon "in the runtime (or
   `cortex-linguistic` without external dependencies)": a category is a `TermNode`, so the
   second place is TC-2's; `surface_token_id` defined and written by no rule; "`Episode`
   (`units: [u32; 12]`, count, tag)" against `pattern`, `len`, `replays`, `tag`,
   `tagged_tick`; "SWS downscaling" against ADR-0037; "cued 6 units complete 12" against a cue
   of six firing the other six; "the population hits the saturation ceiling ... before σ or the
   net causal branching ratio reaches 1.0" against the gross ratio's 1.000 at 2.25 and 1 024
   units (the net stayed at 0.475); "`x86-64-v4` and `ARMv9-A` (T-1)" against T-1's checked
   form, the 20 000-tick pin on x86-64 and AArch64 (ADR-0030). None of them changes the work;
   each is recorded so that the next proposal starts from the tree.


## Deliverables

- [x] **The lexicon ADR (the next free number)** (`docs/adr/0046-*.md`, `depends-on: ADR-0040`,
  ADR-0021 and ADR-0026 named). In the runtime, module `lexicon`: `Entry { token, concept,
  shape, formal }` (a token id, its concept id, its lexical shape, and the token the formal
  register uses in its place, 0 for none); the shapes as constants, each a category the
  reducer of ADR-0040 takes, instantiated with fresh variables on the caller's arena:
  `SHAPE_NOUN` (`N(c)`), `SHAPE_DETERMINER` (`NP(X)/N(X)`, no role), `SHAPE_ADJECTIVE`
  (`N(X)/N(X)`, no role), `SHAPE_INTRANSITIVE` (`S(c)\NP(A):subject`), `SHAPE_TRANSITIVE`
  (`(S(c)\NP(A):subject)/NP(P):object`), `SHAPE_HEDGE` (`S(V)/S(V)` whose role term names its
  own concept as the affect filler) and `SHAPE_TAG` (`S(V)\S(V)`, the same at the end); the
  role-term extension in `language.rs`: a role term that is a compound `ROLE(c)` over a role
  constant binds `c`, a bare role constant binds the argument's head as before, so the
  reducer is untouched and a modifier reaches the affect slot. `Lexicon { entries, sentence,
  np, n, act_open: [u32; 4], act_close: [u32; 4], particles: [u32; 6] }` over a caller's slice
  sorted by token (`Lexicon::new` refuses an unsorted or duplicated slice; a lookup is a binary
  search, a reverse lookup a scan bounded by the slice); `LexiconError::{Unsorted,
  UnknownToken(index), NoToken(concept), Instantiate, Language(LanguageError), Incomplete,
  NoSuchFrame, TooDeep, OutFull}`. `comprehend_tokens(tokens, lexicon, scratch, template,
  speech_act, politeness) -> Result<LinguisticFrameSlot, LexiconError>`: a token equal to
  one of `act_open`/`act_close` sets the frame's act and is not a category, one equal to a
  particle sets the marker and opens the particle slot, every other token is looked up and
  instantiated, then `comprehend`. `realise(frames, root, lexicon, out) -> Result<usize,
  LexiconError>`: the act's opener, then the root frame's `realisation_order` with each
  role's concept as its token (the formal variant at or above `POLITENESS_FORMAL`), descending
  into `child_frame_idx` at `ROLE_CHILD` with an explicit stack of at most `MAX_NESTING`
  frames and a visit count bounded by the arena (a cycle is `TooDeep`), then the closer, then
  the particle when the slot is open and the marker is not none; writes the last token into
  the root's `surface_token_id`; refused for an incomplete frame, a concept without an entry,
  a frame index outside the arena and an output that does not fit. Exit tests in
  `runtime/cortex-runtime/tests/language.rs` (the seven-word table becomes a `Lexicon` of
  token ids, the words a test-local array for the assertions' messages): the round trips
  token ids → frame → token ids for "the dog chased the cat" (the content words in
  subject–action–object order), for a hedge-first epistemic sentence and a tag-last one (the
  affect filled with the modifier's own concept, the hedge realised first and the tag last),
  for a directive with an opener and a particle (the act read from the marker, the particle
  from the token, both realised back), for a nested causal frame (the child clause realised
  at its position), and for the formal register's variant; every id pinned; the whole path
  through `encode_frame` and `decode_frame` realising the same tokens; the refusals. The ADR
  says the descent bound, what a shape cannot express (a coordination, an object relative:
  H-10) and that the words are the host's.
  **Departure:** an eighth shape, `SHAPE_NOMINAL` (`NP(c)`: a name, a pronoun, a noun of a language without articles), since a realised utterance drops the determiners and can be read back only where a bare content token is a noun phrase; the round trip that closes is a sentence of names. The reading takes a `Reading { categories, next_variable }` for the caller's category slice and the fresh-variable counter. `LexiconError` has `Malformed`, `UnknownToken`, `UnknownShape`, `ArenaFull`, `CategoriesFull`, `Language`, `Incomplete`, `NoSuchFrame`, `NoToken`, `TooDeep` and `OutFull`.
- [x] **The second-prior ADR (the number after it)** (`docs/adr/0047-*.md`, `depends-on:
  ADR-0044`, ADR-0036 named). No crate code for the topology: the random prior is
  `Prior { window: units / 2 - 1, rewire_q0_8: 0, .. }` with the lattice's other parameters,
  named in the ADR and built by a helper in `tests/common/mod.rs`. In the runtime or the
  test harness: the fine-bin sampler (the population's count over spans of $2^8$ ticks from
  `bin_activity` and `last_activity`, the bin's close handled) and the lag-$k$ slope
  $r_k = (n \sum a_t a_{t+k} - \sum a_t \sum a_{t+k}) / (n \sum a_t^2 - (\sum a_t)^2)$ over
  the fine series in `i128`, Q16.16, a negative slope 0, for $k$ in `[1, 2, 4, 8, 16]`, in a
  runtime module (`estimator.rs`) with unit tests against an oracle series (a geometric series
  whose slopes are known, a constant series with no estimate, a window's coarse slope equal to
  the record's on the same bins). Exit tests in `runtime/cortex-runtime/tests/estimator.rs`:
  at 256 units on the random prior, one fixed gain, two windows and four kicks (the gate),
  and on both priors the fine-bin slopes over the same windows, pinned; the three-gain,
  two-window, eight-kick sweep on the random prior and the fine-bin slopes at every gain on
  both priors at 256 and at 1 024 units as `exhaustive` tests. The ADR's table holds, per
  prior, size and gain: the coarse estimate per window, the spikes, the gross and net causal
  ratios, and the fine-bin slopes $r_1 \ldots r_{16}$; the exponential average of the coarse
  estimates over the windows is derived in the ADR from the pinned values, not pinned
  separately. **What is decided**: whether the one-window noise is the window's or the bin's
  (the fine slopes say which); whether the fine lag-one slope tracks the causal ratio where
  the coarse one does not; whether a smoothed coarse estimate helps; and, from that, whether
  the record needs a second line, with the line's contents (the sums at a bin near the
  generation time for lags 1 to $K$, or nothing) written as Specified with the format bump it
  would cost, for the round that makes it; the ceiling's role restated.
  **Departure:** `count_bins` and `slope_at_lag` live in `cortex-homeostasis` (the estimator's owner; the lag-one case is the record's rule, held equal by a property walk), not in a runtime module; the fine series comes from the run's train, not from a sampler of the open bin (the record clears its last bin at a window's end, so the train is the only complete source); the exit tests are in `tests/reference.rs`, not a new file (every helper is that file's). The decision: no line of the record, since the coarse estimate's noise is the bin's and the fine slope reads a driven population's autocorrelation, which moves with the gross causal ratio at 1 024 units and not at 256.
- [x] **The trace-tagging ADR (the number after that)** (`docs/adr/0048-*.md`, `depends-on:
  ADR-0038`, ADR-0044 and ADR-0045 named). In `cortex-hippocampus`: `Burst { from, spikes }`
  and `burst(train, window) -> Option<Burst>`, the start tick of the densest span of `window`
  ticks over a `(tick, unit)` slice sorted by tick, the first such span at equal counts, by
  two cursors over the slice (`None` for an empty train or a window of zero); `capture(train,
  from, window, out: &mut [u32; PATTERN_MAX]) -> u8`, the distinct units that fired in
  `[from, from + window)` ranked by spike count descending, first spike ascending, index
  ascending, the first `PATTERN_MAX` written to `out` in that order and their number returned
  (0 for none); both bounded by the slice; unit tests and a property walk (every unit written
  fired in the span, no unit twice, the ranking's three keys, the span's density against a
  brute-force scan of every start tick on a small train). In the runtime, module `episode`:
  `tag_from_trace(exec, train, from, window, priority) -> Result<(u32, [u32; PATTERN_MAX],
  u8), TagError>` (the capture tagged into the executor's ledger, the index and the pattern
  returned; `InvalidPattern` for a span with no spike), `tag_burst(exec, train, window,
  priority)` (the densest span's pattern), and `tag_discovery(exec, train, at, window,
  discovery, priority) -> Result<Option<(u32, u32)>, TagError>` (when `discovery.reward_q16`
  is positive, the pattern of `[at - window, at)` tagged and returned with the invention's
  predicate id as the caller's association; `None` for a reward that is not positive, with
  nothing tagged). Exit tests in `runtime/cortex-runtime/tests/capture.rs` at 256 units (the
  gate) and 1 024 (`exhaustive`), on the lattice prior of ADR-0044 at a gain of 2.0 with the
  baseline at 1.0: the network awake under the drive for two bins, an experience (the cue of
  twelve neighbours, as ADR-0044's night) at the third bin's start, the clause store of
  `tests/discovery.rs` searched between ticks a ripple's quarter later with the committed
  reward passed to `Executor::reward`; the train of that run from a fork of the start image
  under the same drive and cues; `tag_discovery` at the search's tick over the ripple before
  it (the pattern it finds, pinned: the experience's twelve, since each fired more than once
  and no other unit did), `tag_burst` over the two bins before the experience (the densest
  background span and its pattern, pinned), and a search whose store has nothing to invent
  tagging nothing; the night of ADR-0044 from a pressure of 1.0 at shift 5; the synapses
  among each pattern before and after, the stages, the replays, the tags; the readout of a cue
  of the first six of each pattern on forks of the image before and after the night, pinned.
  The ADR records both sizes' numbers and states: for H-9, that the rule finds an experience
  in the train and the pattern completes after the night, and what the background span's
  pattern does; for H-11, that a rewarded invention now has a pattern of units in the ledger
  tagged from the activity of the ripple before its reward, that a night consolidates it, and
  that a cue of half of it fires the rest, the association held in the caller's table, and
  what would move it into the image (the round that runs discoveries inside the executor's
  loop). The online capture in the tick is Specified with its requirement (a per-tick merge
  of every worker's spikes in unit order).
  **Departure:** `tag_discovery` takes a `coincidence` window beside the ripple: the pattern is the densest basal time constant (512 ticks) of the ripple before the reward, ranked as `capture` ranks it, since ranking over the whole ripple found the units the stationary drive had fired most beside the experience; the invention's pattern at 256 units is eleven of the twelve cued neighbours and one outsider and completes five of six, at 1 024 all twelve with every synapse at the rail and five of six on a cue in rank order; the two episodes at 256 share unit 0, whose synapses into the invention's pattern the other episode's replays depress, six of eleven to the negative rail; the network's own pattern at 1 024 holds inhibitory units whose synapses the replay carried across zero (finding F-36). The exit test is in `tests/reference.rs`.
- [x] **Finding F-35** in whitepaper §11: the Context's item 8, with the disposition (the
  lexicon in the runtime, `surface_token_id` written by `realise`, the record's fields named,
  no downscaling, the readout's count, the gross ratio's 1.000, T-1's checked form).
  **Also:** finding F-36, the pair rule potentiating an inhibitory synapse among a replayed pattern across zero, found by the 1 024-unit night; recorded with an open question, not fixed (a change to the pair rule is its own ADR).
- [x] **H-9's open item and H-11's synaptic half dispositioned** in §11.1 with the numbers;
  the estimator's open question resolved or narrowed by the second-prior ADR's decision; a
  new open item only if the measurement leaves one.
- [x] **The documents.** Whitepaper 4.11.0: the executive summary's sentence on what exists
  and what is Specified (the lexicon out of the Specified list); §1.6 rows
  (`cortex-hippocampus`: `burst`, `capture`; the date); §5.1 (the runtime's modules); §5.2.15
  (Public API, the two rules, a directive `fn capture`); §5.2.16 (the second prior's
  measurement beside the first); §5.2.20 (Status: the lexicon Implemented on ids; the
  composition paragraph: `comprehend_tokens`, `realise`, the role-term extension; directives
  `fn realise`, `fn comprehend_tokens`); §5.2.30 if the role term's shape is stated there;
  §6.6 (a pattern tagged from the train); §6.9 (heading and body: realisation Implemented on
  ids, the words the host's); §6.10 (the invention's episode); §8.8 rows (complementary
  learning: the capture; criticality: the second prior and the fine-bin slopes; the
  construction rows: the descent and the lexicon Implemented on ids); §9 three rows; §11
  F-35; §11.1; Appendix C (M5 and M8 clauses); Appendix D references 78 onward (Brunel 2000
  is present; add what the ADRs cite that is not); the glossary (Lexicon, Burst, Capture).
  README (the Implemented and Specified rows), `CLAUDE.md` (the sentence on what exists),
  `docs/zh-TW/README.md` (§6 and §11 rows), `docs/adr/README.md` (three rows), `CHANGELOG.md`
  (one entry under Unreleased in the shape of brief 022's).
- [x] **Not adopted, with the reason in the ADR that is closest:** the online capture inside
  the tick (the trace-tagging ADR: an executor change with a worker-count-independence
  requirement, taken by the round that also runs discoveries in the loop); a concept field in
  `Episode`'s reserved bytes and the format bump (the same ADR: a field the loop never reads);
  a chart or type raising (the lexicon ADR: H-10's decision, not this round's); standardising
  apart in `prove` (unchanged, ADR-0045); a layered or columnar prior (the second-prior ADR:
  the random network is the measurement the open question asked for; section kind 38 stays
  Specified); a change to the estimator or the controller on the strength of the measurement
  (the same ADR names the change and its cost for the next round); words, strings or a
  language's name in a crate (the lexicon ADR: the host's).
- [x] **This brief archived** under `briefs/archive/` with the frozen banner, every box
  dispositioned, the precondition directives removed and the links rebased.

## Not empowered

- No executor field, no image section, no format bump, no field in any record: the ledger's
  reserved bytes stay zero; the association is the caller's.
- No new crate; no dependency in a state crate.
- No float anywhere, including the tests and the sampler.
- No change to `estimate_branching_ratio`, `regulate`, the bin, the window, the ceiling, the
  ripple, the replay drive, the sleep thresholds or `reduce`: this round measures and
  composes.
- No renaming of the CI jobs the ruleset requires; no move of the determinism pin.
- No test in the pull request's gate above one night and a few windows at 256 units; the
  sweeps are the weekly job's.
- No word of the proposal's vocabulary in a document's claims; no language name in a crate.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in
the ADR that owns it: the lexicon's entry layout, its shapes and their categories, the
markers' form (an opener and a closer per act, or one), the descent's bound, where the sampler
and the slopes live (the runtime or the harness), the lags measured, the second prior's exact
window and delays, the capture's ranking keys and the burst's tie rule, the window before a
reward, the sizes, gains and priorities the exit tests use, and which file each exit test
lives in. It may not reach the standing directives, the whitepaper's invariants or the
constraints in `CLAUDE.md`.

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
each correction went; the lexicon's shapes, its markers and the role-term extension, and the
round trips the exit test pins; the second prior's parameters and census, the coarse estimate
against the causal ratios on it, the fine-bin slopes on both priors at each gain, what that
decides about the noise, the bin and the record's line, and what change the next round would
make with its cost; the capture rules and what they found in the train (the experience, the
background span, the reward's window), the synapses among each pattern before and after the
night and the readouts on both images, and what that decides for H-9's open item and for
H-11's synaptic half, with the reason the association is the caller's; that the format and the
pin are untouched; what the mutation gate found on the changed lines and how each survivor was
answered; what was not done (the online capture, the record line, a chart, the tuning) and
why; and what the re-examination after the round recommends next.
