---
status: proposed
date: 2026-09-13
---

# Brief 020 — Hypervector bodies and categorial reduction: the vector-symbolic algebra as a second record of `cortex-symbolic`, syntax as type reduction over the term arena, and a frame that round-trips through both

> A self-contained prompt for one round of work. Read `briefs/README.md`, then this file, then
> the files it names. Frozen when executed; the outcome lives in the ADRs that accept it, the
> changelog and the code.

## Mission

Answer an architectural proposal ("Native Dual-Stream Neuro-Linguistic Cortex, Metric
Conceptual Spaces & Universal Expressive Architecture", received 2026-09-13) with decisions,
code and tests instead of a document that drifts. The proposal names four frontiers: a
10 000-bit hypervector body arena with SIMD unbinding and a `popcnt` codebook search, plus
Gärdenfors quality spaces in Q16.16; combinatory categorial grammar (CCG) as algebraic type
reduction over `TermNode`; a cortico-striatal thematic gate (Dominey) that buffers tokens in
the cerebellar delay line and pops them into frame roles when a function word fires the
basal-ganglia gate; and a five-axis "universal cognitive stance" record with a "surface
realization boundary" it calls *"What stays outside. Words."*. Four of its stated premises are
not in the tree (the Context below says which), so the round begins by re-deriving them. When
the round is done: **one ADR** admits a second record to `cortex-symbolic` under
[ADR-0016](../docs/adr/0016-thirty-two-crate-architecture.md)'s test, `HypervectorBody`, 160
words of 64 bits (10 240 bits, twenty cache lines), with the vector-symbolic algebra the
whitepaper's §8.8 row has called Specified since 3.0.0 as integer rules over the words:
binding by XOR (its own inverse, so unbinding is binding), permutation by cyclic rotation,
bundling by per-bit majority with a fixed tie-breaker, the Hamming distance by population
count, the clean-up as the nearest codebook entry, a decode confidence from the distance, and
a seeded generator for a codebook; every rule a `u64` loop the compiler vectorises for the
target it builds for, with no intrinsic and no `unsafe`, so the result is the same integer on
every target. **One ADR** puts syntax on the term arena of
[ADR-0025](../docs/adr/0025-term-arena-and-unification.md): a category is a term (an atom, or a
functor category `X/Y` or `X\Y` as a compound over a reserved slash functor, its argument slot
optionally annotated with the role the argument fills), the four combinatory rules (forward
and backward application, forward and backward harmonic composition) are unifications over
categories, and a shift-reduce reducer over caller-provided slices reads a sequence of lexical
categories into one root category with a log of the reductions it made, deterministically,
with every bound a result; type raising and a chart are Specified. **The runtime composes the
three crates** in one module: a category sequence is reduced, the roles its derivation reports
are bound into a `LinguisticFrameSlot` (Layer 2), the sealed frame is encoded as a bundle of
role-bound concept bodies, and each role is read back through the codebook with a confidence
(Layer 1, the unbinding R-9 has called Specified); the exit test carries "the dog chased the
cat" as concept ids from a lexicon table that exists only in the test, so that no word enters
a state crate, and pins every distance so that the AArch64 job holds the arithmetic equal.
The Dominey gate, the stance record, Gärdenfors coordinates as a second metric and SIMD
intrinsics are not adopted, with the reasons; the proposal's sequencing claim about H-9 is
answered. No record in the image changes: format 13 stays and the determinism pin is untouched.
The whitepaper, README, `CLAUDE.md`, the reader's guide, the ADR index and the changelog say
all of this, and this brief is archived with every check green.

## Standing directives

- Every claim is Implemented, Specified, Target or Hypothesis. What a bundle of three bound
  pairs decodes to on a codebook of sixty-four seeded bodies is stated as what it is; what a
  lexicon and a corpus would derive is a hypothesis with a protocol
  ([ADR-0010](../docs/adr/0010-measured-or-target.md)).
- The repository wins over the document; a disagreement is a numbered finding in whitepaper
  §11, never a silent edit.
- No `f32`/`f64`; Q16.16 in `i32`/`u32`, widened to `i64` to multiply; every operation on a
  state field saturates or wraps by name (`clippy::arithmetic_side_effects` is denied
  everywhere, [ADR-0029](../docs/adr/0029-structural-enforcement.md)); a shift amount is
  bounded a line above the shift.
- 64-byte `#[repr(C, align(64))]` records with compile-time assertions; a record that is a
  whole number of cache lines larger than one asserts its size as that multiple; no heap
  types, threads or `unsafe` in a state crate (`unsafe_code = "forbid"` in every one of them,
  ADR-0029), which is why no `core::arch` intrinsic can appear there; `core::simd` is nightly
  and fails TC-1.
- Every quantity has one owner ([ADR-0016](../docs/adr/0016-thirty-two-crate-architecture.md));
  a second record in a crate passes ADR-0016's six-part test in its ADR; the crate count
  stays 32.
- A change to a record bumps `CortexFileHeader::FORMAT_VERSION`, updates its §5.2 table and
  gets a changelog entry (rule L-6); a new record no section carries yet does not.
- State crates declare no dependencies (TC-2); a composition of two crates lives in
  `runtime/cortex-runtime`, the one crate that depends downward
  ([ADR-0023](../docs/adr/0023-executor.md)).
- Rule L-3: a term, a frame and a body hold ids and bits, never strings; the lexicon that
  turns an id into a word is the runtime's boundary and is Specified (§1.5, §6.9).
- The determinism pin of [ADR-0030](../docs/adr/0030-verification-governance.md) moves only
  with a stated reason; the mutation gate on the changed lines must pass; a new rule carries
  a test over the lattice of `testkit/prop.rs`.
- Conventional Commits with a real body; never commit on `main`; the required checks keep
  their names.
- Latest ≠ Newest (§2.1): binary spatter codes and hyperdimensional computing (Kanerva 1996,
  2009), holographic reduced representations (Plate 2003), combinatory categorial grammar
  (Steedman 2000; Hockenmaier and Steedman 2007), shift-reduce CCG parsing (Zhang and Clark
  2011) and normal-form derivations (Eisner 1996) are decades old with documented failure
  modes and pass; they are admissible as mechanisms, and adopted only where the tree has the
  record they need.

## Context

Re-derived on 2026-09-13 against `main` at `182c00c`.

- `crates/cortex-symbolic/src/lib.rs`: `SymbolicHypervectorHeader` is 64 bytes with
  `vector_id` `[0..4)`, `dimensionality` `[4..8)`, `binding_role_id`, `filler_concept_id`,
  `token_vocab_id`, `hamming_distance_cache` `[20..24)`, `permutation_shift: u16` `[24..26)`,
  `flags` `[26..28)`, `confidence_score` `[28..32)`, the blend fields, `rebase_count` `[39)`
  and `_reserved: [u8; 24]` `[40..64)`; `DIMENSIONS` is `10_000`; `bind` stores two ids and a
  flag, `blend` a header, `rebase` composes a shift modulo the dimensionality. **No vector
  exists**: the crate's own doc says "Bundling, permutation, unbinding and the codebook are
  Specified", and `hamming_distance_cache` and `confidence_score` are written by nothing.
  The proposal's "implements `bind`, `blend`, and `rebase`" is right about the names and wrong
  about what they do: they write metadata.
- The proposal's byte accounting is wrong: 10 000 bits are 1 250 bytes and 156.25 words;
  160 words are 1 280 bytes and 10 240 bits. Whitepaper §5.2.9 and Appendix A row 9 say
  1 250 B; a record must be a whole number of words and, under
  [ADR-0001](../docs/adr/0001-64-byte-pod-records.md)'s alignment, of cache lines. The body
  is 160 words, every bit used, and the width the header's `DIMENSIONS` names follows it: a
  rotation over 10 000 of 10 240 bits would need a mask on every word and a well-formedness
  clause (240 bits MUST be zero) for no gain.
- "SIMD-vectorized unbinding using AVX-512 (`_mm512_xor_si512`), ARM SVE2": every
  `core::arch` intrinsic is an `unsafe fn`, and `Cargo.toml`'s `[workspace.lints.rust]`
  forbids `unsafe_code` in every state crate (ADR-0029, TC-9); `core::simd` is nightly-only
  and fails TC-1. Whitepaper §8.10 says the next `unsafe` "will be SIMD intrinsics and
  `mmap`", each needing an ADR with an invariant and a test. A loop of `u64` XORs and
  `count_ones` over 160 words is what the compiler vectorises for the target it builds for
  (SSE2, AVX2, AVX-512 or NEON/SVE by `-C target-cpu`), and its result is an integer that is
  the same on every target, which is the property T-1 holds; whether an intrinsic ever
  beats the compiler is a benchmark under `docs/benchmarks/README.md`, not a premise.
- `crates/cortex-reasoning/src/term.rs`: `TermNode` is `kind` `[0)`, `arity` `[1)`, `_pad`
  `[2..4)`, `functor` `[4..8)`, `children: [u32; 8]` `[8..40)` (index + 1), `_reserved: [u8;
  24]`; `compound(concept, args)` refuses more than eight; `unify(a, b, arena, bindings,
  trail, stack) -> (UnifyResult, bound)` writes the variables it binds from `trail[0]` and
  undoes them on any failure; `deref` follows bindings bounded by the arena; `undo` reverts a
  prefix. A category as a compound over a reserved functor needs no byte of the reserved 24:
  the proposal's "or by utilizing the 24 reserved bytes" is the option that would change the
  record and the format for what a compound already expresses.
- `crates/cortex-linguistic/src/lib.rs`: `bind_role(role, concept, confidence)` binds one of
  four roles with a confidence and lowers the frame's to the weakest; `required_roles`,
  `is_complete`, `seal`, `realisation_order`; `_reserved: [u8; 19]` `[45..64)`. Role bits
  are `ROLE_SUBJECT` 1, `ROLE_ACTION` 2, `ROLE_OBJECT` 4, `ROLE_AFFECT` 8. The frame is the
  one owner of "which concept fills which role"; a second mechanism that writes the same
  fields would be two owners.
- `crates/cortex-cerebellum/src/lib.rs`: `pred_ring: [i32; 7]` `[32..60)` holds the scalar
  predictions of the last seven steps in Q16.16 for `step_forward_model`; it is not a token
  buffer and holds no id. `crates/cortex-basal-ganglia/src/lib.rs`: `compute_gating` is a
  linear gate over three drives of one channel; nothing routes tokens. `crates/cortex-social/
  src/lib.rs`: `_reserved: [u8; 16]` `[48..64)`, not 18. **No token path reaches the loop**:
  the injector ring delivers spikes (§8.5), and a category sequence is an input a caller hands
  a rule between ticks, as a reward or a tag is. The Dominey circuit would be a learned
  sequence mechanism for the role assignment the derivation already makes, on an input that
  does not exist; what "a function word triggers the gate" is, symbolically, is the
  application rule firing when the functor's argument arrives.
- Whitepaper §1.5: "Language stays inside the engine: frames are assembled natively from
  hypervector unbinding (§5.2.20) and no external language model is part of the system";
  §6.9: the lexicon "maps each concept, the politeness level and the marker to Chinese or
  English tokens (Specified)". **No invariant named "What stays outside. Words." exists** in
  the tree; the rule the proposal wants is §1.5's and rule L-3's, enforced today by
  `#![no_std]` without `alloc` (a state crate has no string type at all) and the three
  TC-5 absence directives in §2.2 (`String`, `Box<`, `Vec<`). One directive is missing: that
  no state crate pulls `alloc` in.
- Whitepaper §11.1 H-9: "a behaviour that reads them". The proposal's sequencing note says
  the engine "lacks behavioral readouts" and therefore the language stream must precede the
  H-9 measurement. The readout consolidation needs is pattern completion: cue a part of a
  tagged pattern through the injector after a night of the stages of
  [ADR-0037](../docs/adr/0037-sleep-regulation.md) and count whether the rest fires, which is
  a spiking readout on the executor and needs no symbol. The language stream and H-9 are
  orthogonal; this round does not change H-9's protocol and does not claim to serve it.
- Whitepaper §8.8, row "Vector-symbolic architecture (Plate, Kanerva)": "Binding by XOR /
  circular convolution, bundling by majority, permutation by cyclic shift, clean-up by nearest
  codebook entry | Specified" since 3.0.0; row "Native language …": "Layer 1 … Specified".
  Appendix A row 9: "Hypervector bodies | 1 000 000 | 1 250 B | 1.25 GB". Appendix C M8:
  "hypervector unbinding and the lexicon behind `cortex-linguistic`" open.
- `runtime/cortex-runtime/Cargo.toml` depends on seven state crates by path; `src/trial.rs`
  is the precedent for a runtime module that composes crates between ticks with no executor
  field. `tests/differential.rs` pins `0x1724f3486c1d674e` with 95 spikes; nothing in this
  round touches the executor, so the pin is untouched by construction.
- Preconditions, checked by `spec-guard` until this brief is archived:

<!-- @assert-absence target="crates/cortex-symbolic" symbol="HypervectorBody" word="true" reason="brief 020 precondition: no hypervector body record exists yet" -->

<!-- @assert-absence target="crates/cortex-reasoning" symbol="CATEGORY_FORWARD" word="true" reason="brief 020 precondition: no categorial functor exists on the term arena yet" -->

<!-- @assert-count target="crates/cortex-symbolic" symbol="DIMENSIONS: usize = 10_000" min="1" reason="brief 020 precondition: the header still names a width no whole number of words holds" -->

## Deliverables

1. [ ] **The body** (`cortex-symbolic`, the first ADR; no format bump). A second record,
   `HypervectorBody`, `#[repr(C, align(64))]`, `words: [u64; BODY_WORDS]` with `BODY_WORDS`
   160 and `BODY_BITS` 10 240, size 1 280 asserted at compile time as twenty cache lines;
   `Default` the zero body (the identity of binding); `Clone, Copy, Debug, PartialEq, Eq`.
   `SymbolicHypervectorHeader::DIMENSIONS` becomes `BODY_BITS` (the reason in the ADR; the
   rebase tests follow). Rules: `from_seed(seed: u64) -> Self` (splitmix64 over the words: a
   dense pseudo-random body, the same on every target); `bind(&self, other) -> Self` (XOR;
   an involution, so $\text{Concept} \approx S \otimes \text{Role}^{-1}$ is `sealed.bind(role)`);
   `permute(&self, shift: u32) -> Self` (cyclic rotation by `shift` modulo `BODY_BITS`, the
   permutation $\Pi^k$ the header's `permutation_shift` names; 0 the identity); `bundle(items:
   &[Self]) -> Option<Self>` (per-bit majority; an even count adds `TIE_BREAKER`, a fixed body
   from `TIE_SEED`, as one more operand so that no tie exists; `None` for no item or more than
   `BUNDLE_MAX` 15); `hamming(&self, other) -> u32`; `nearest(&self, book: &[Self]) ->
   Option<(usize, u32)>` (the index and distance of the nearest codebook entry, the lowest
   index on a tie; `None` for an empty book); `confidence_q16(distance) -> u32` ($1 - 2d /
   \text{BITS}$ clamped to $[0, 1]$: 1.0 at zero, 0.5 at a quarter, 0 at half and beyond).
   `SymbolicHypervectorHeader::record_readout(role_id, filler_id, distance)` binds the pair
   and writes `hamming_distance_cache` and `confidence_score`, the two fields nothing wrote.
   Every rule is a loop over the words with named operations; no intrinsic, no `unsafe`, no
   allocation. The ADR carries ADR-0016's six-part test, says why the width is 10 240 and
   why intrinsics are refused, names the image section the arena takes when a runtime store
   composes it (46, Specified, each word little-endian), and says why Gärdenfors coordinates
   are not a second metric (a quality dimension is a level body bound to its dimension's
   role and read by the same distance; Specified). Tests: the sizes; bind an involution with
   the zero body its identity over the lattice's seeds; permute by 0 and by `BODY_BITS` the
   identity, by $k$ then $\text{BITS} - k$ the identity, by 1 moving every bit by one; the
   distance against a per-bit oracle, symmetric, zero on itself, at most `BODY_BITS`, and
   `BODY_BITS` on a complement; bundle against a per-bit oracle for one to fifteen items with
   the tie-breaker's rule pinned and sixteen refused; nearest on the codebook itself, the
   lowest index on a tie, `None` on nothing; the capacity case (a bundle of three role-bound
   fillers on a codebook of sixty-four seeded bodies: each role unbound recovers its filler
   with the distance pinned and every other entry farther, and a fourth role reads as noise);
   the confidence at 0, a quarter, half, and `BODY_BITS`; the readout's three fields; and a
   property walk (`mod prop` with the kit: bind round-trips, bundle equals the oracle at
   random counts, permute composes, the distance is a metric on sampled triples).
2. [ ] **Categorial reduction** (`cortex-reasoning`, the second ADR; no record change). A
   module `category` over the term arena: `CATEGORY_FORWARD` and `CATEGORY_BACKWARD`, two
   reserved functor ids at the top of the id space (`0xFFFF_FF01`, `0xFFFF_FF02`); a
   functor category is a compound over one of them with children `[result, argument]` or
   `[result, argument, role]`, the role a term (a constant of the caller's role vocabulary)
   meaning "the argument fills this role of the result"; an atomic category is any other
   term (a constant such as `S`, or a compound such as `NP(dog)` whose children carry
   features and heads, variables among them). `forward(result, argument, role) ->
   Option<TermNode>` and `backward` build them; `is_functor`, `slash`, `result`, `argument`,
   `role` read them. Rules `RULE_FORWARD_APPLICATION` ($X/Y \; Y' \Rightarrow X$ when $Y$
   unifies with $Y'$), `RULE_BACKWARD_APPLICATION` ($Y' \; X\backslash Y \Rightarrow X$),
   `RULE_FORWARD_COMPOSITION` ($X/Y \; Y'/Z \Rightarrow X/Z$, the new slash carrying the
   right functor's role) and `RULE_BACKWARD_COMPOSITION` ($Y'\backslash Z \; X\backslash Y
   \Rightarrow X\backslash Z$), tried in that order (application before composition, forward
   before backward: the normal-form preference), each a unification over the categories the
   two stack items dereference to. `Reduction { rule, left, right, result, role:
   Option<u32>, argument }` is one step. `ParseScratch` bundles the caller's slices: the
   arena and its free cursor (composition needs one new node), the binding table, the trail
   and its length (the parse accumulates one substitution, undone by the caller with `undo`),
   the unification stack, the parse stack and the step log. `reduce(categories: &[u32],
   scratch) -> Result<u32, ParseError>`: shift each category, reduce the top two while a rule
   applies, return the one remaining term or `NoDerivation { remaining }`; `Empty`,
   `StackFull`, `ArenaFull`, `BoundExceeded` and `Malformed` are results; an occurs failure
   is "the rule does not apply". Greedy and deterministic: the reducer commits to the first
   applicable rule and never backtracks, so a sequence whose only derivation needs a later
   reduction first (an object relative) is `NoDerivation`; type raising and a chart are
   Specified, and whether the greedy reducer covers the constructions the templates of
   `cortex-linguistic` need is a hypothesis with a protocol. Tests: each rule alone with the
   result's shape; subject–verb–object with determiners (`NP/N N (S\NP)/NP NP/N N` to `S`,
   the log's roles and arguments naming the heads through the bindings); a modal by forward
   composition then application; an adverb by backward composition; a non-sentence, an empty
   sequence and a lone category; a parse stack of one, an arena with no free node, a trail
   too small, an index outside the arena, each a result and nothing bound; determinism (the
   same sequence twice gives the same log and table); and a property walk (random sequences
   of one to eight categories from a small lexicon never panic; a result names a node inside
   the arena, every step's three indices do, the steps are one fewer than the shifts, and the
   table dereferences).
3. [ ] **The composition** (`cortex-runtime`, both ADRs; a module `language`, no executor
   change). The runtime depends on `cortex-symbolic`, `cortex-reasoning` and
   `cortex-linguistic` by path. Role concepts for the grammar: `role_concept(role_bit)` and
   `role_of_concept(id)` over a reserved band (`0xFFFF_FE00 | bit`). `comprehend(categories,
   scratch, template, speech_act, politeness) -> Result<LinguisticFrameSlot, ParseError>`:
   reduce, then bind into a new frame each role a step reports with the head concept of its
   argument (the argument dereferenced; a compound's first child dereferenced to a constant),
   and the root's head as the action, every binding at a confidence of 1.0; the frame is
   returned unsealed. `encode_frame(frame, roles: &[HypervectorBody; 4], book) ->
   Option<HypervectorBody>`: the bundle of each bound role's body bound to its concept's body
   (`None` for a concept outside the book or no role bound). `read_role(sealed, role_body,
   book) -> Option<(usize, u32, u32)>` (Layer 1: unbind, nearest, the distance and the
   confidence). `decode_frame(sealed, roles, book, template, speech_act, politeness) ->
   LinguisticFrameSlot`: every role whose confidence reaches `DECODE_FLOOR_Q16` (0.125) is
   bound with it. No allocation in any of them. The exit test
   (`runtime/cortex-runtime/tests/language.rs`): a lexicon table in the test file alone maps
   the words of "the dog chased the cat" to concept ids and categories; `comprehend` gives a
   causative frame with `dog`, `chase` and `cat` in subject, action and object; sealed and
   encoded on a codebook of seeded bodies, `decode_frame` returns the same three concepts
   with the distances pinned (each near a quarter of `BODY_BITS`) and leaves the affect role
   unbound (its readout near half, the confidence 0); `realisation_order` gives
   subject–action–object and the test's lexicon renders it, in the test, as three tokens; a
   second sentence with the same verb and swapped nouns decodes to the swapped frame;
   `comprehend` refuses a non-sentence; and the whole path is bit-identical on a second run.
   The AArch64 job runs the same test, so every pinned distance is held on both targets.
4. [ ] **Specified and not adopted, with reasons in the ADRs and the whitepaper.** The
   Dominey cortico-striatal gate: a second owner for the role assignment `bind_role` makes
   from the derivation, on a token path that does not exist; what the gate is symbolically
   is the application rule. The stance record: three of its five axes have owners already
   (the illocutionary force is `speech_act_type` and `intended_speech_act`; the relational
   register is `politeness_level`, set from `cortex-social`'s trust; the epistemic hedge is
   the epistemic template's affect role and the frame's confidence), and the other two have
   no reader, so a record for them fails ADR-0016's third test. The "surface realization
   boundary" is §1.5's boundary; its executable form gains the missing directive. "Zero
   semantic distortion" into code, proofs or a voice is not a claim the tree can make and is
   not made. Gärdenfors coordinates: a second metric beside the Hamming distance on the same
   concept; Specified as level bodies. Intrinsics: refused as above. The sequencing claim
   about H-9 is answered in the whitepaper (pattern completion is the readout; the protocol
   stands). CKY, type raising, the lexicon, the state-vector arena and the body arena's
   section stay Specified.
5. [ ] **Documents.** Whitepaper §1.6 (the three Logic cells; thirty-nine records), the
   executive summary, §2.2 (the `alloc` directive), §5.2.9 (the body's table, the API, the
   status, the rules with executable assertions), §5.2.20 (Layer 1's status; the
   comprehension path), §5.2.30 (the categories, the reducer, the API), §6.9 (R-9 with what
   is Implemented, in both directions), §8.8 (the vector-symbolic row, the native-language
   row, a categorial row, the equations), §9, §11 (a finding for the width and the two
   fields nothing wrote), §11.1 (the hypothesis on the reducer's coverage; the sequencing
   answer under H-9), Appendix A (row 9 at 1 280 B), Appendix C (M8), Appendix D (Kanerva
   1996 and 2009; Plate 2003; Steedman 2000; Hockenmaier and Steedman 2007; Zhang and Clark
   2011; Eisner 1996; Dominey 1995 and Gärdenfors 2000 for what was not adopted), the
   glossary; the ADR index; `README.md`, `CLAUDE.md`, the reader's guide; `CHANGELOG.md`;
   this brief archived with every box dispositioned.
6. [ ] **Departures and measurements recorded.** Where the tree departs from this text (a
   constant, a name, a rule's order), the archived brief says so under the box, as brief 019
   did for the ripple and the drive.

## Not empowered

- To add a state crate, a dependency, `#[allow]`, a feature flag, a nightly attribute, a
  `core::arch` intrinsic, `core::simd`, or `unsafe` outside `arena.rs`; to add a barrier, a
  phase or an executor field.
- To store a string, a byte string or a token in any record or rule of a state crate; the
  lexicon is a table in the exit test and nowhere else.
- To change `TermNode`, `LinguisticFrameSlot` or `SymbolicHypervectorHeader`'s layout: the
  categories are compounds, the roles are the frame's, and the readout writes fields that
  exist. To bump the format for a record no section carries.
- To make the reducer allocate, recurse without a bound, or search (a chart is the next
  decision, not a hidden loop); to make `bundle` break a tie by anything but the fixed body.
- To write "SIMD" as a claim about the tree: the compiler's vectorisation is neither
  measured nor promised; a benchmark under `docs/benchmarks/README.md` is where a figure
  would come from.
- To move a pin, or to decide the `mmap` path, core pinning, the broker, the lexicon, affect's
  composition, per-column records, the canvas hydration or the H-9 measurement.

## Architectural empowerment

- The executing round may choose the width of the body (a whole number of cache lines), the
  bundle's maximum, the tie-breaker's seed, the confidence's form, the reserved id bands and
  the rules' order, with the reason in the ADR and a test at each boundary; the width MUST be
  a whole number of 64-bit words and of 64-byte lines, and the distance MUST equal the
  per-bit count exactly.
- It may encode the role on the slash's third child or on the argument category, if it names
  why the other form loses information under composition; the third child is the default.
- It may put the composition in a test alone instead of a runtime module if it shows that no
  rule in the module is more than a call into a crate; a module with a rule (the head of an
  argument, the floor on a readout) is the default.
- It may set the decode floor from the capacity case's numbers rather than at 0.125, with the
  margin stated.
- It may split the first ADR in two (the record; the algebra) if the six-part test and the
  intrinsics refusal read better apart.

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
with 95 spikes, untouched. The three precondition directives above are gone with the archived
brief.

## Report

The closing message states: which of the proposal's premises the tree contradicted and where
each correction went; the body's layout and byte accounting and why the width is 10 240; the
algebra's rules with the numbers the capacity case reached (the distance of a recovered
filler, of the nearest wrong entry, of an unbound role); the category encoding, the four rules,
their order and what the greedy reducer cannot derive; what the exit test holds in both
directions with the pinned distances; that the format and the pin are untouched and why; what
the mutation gate found on the changed lines and how each survivor was answered; what was not
done (the Dominey gate, the stance record, the second metric, intrinsics, type raising, the
chart, the lexicon, the section, the H-9 readout) and why.
