---
status: proposed
date: 2026-09-13
---

# Brief 021 — Induction on the term arena, exact continued fractions, and the discovery path: predicate invention by inverse resolution, a bounded search for a polynomial continued fraction, and the runtime's composition of a description-length drop into the modulator's reward and of a prover's certificate into a theorem

> A self-contained prompt for one round of work. Read `briefs/README.md`, then this file, then
> the files it names. Frozen when executed; the outcome lives in the ADRs that accept it, the
> changelog and the code.

## Mission

Answer an architectural proposal ("Autonomous In-Engine Discovery, Geometric Intuition
Manifold & Synthetic Mind Synthesis", received 2026-09-13) with decisions, code and tests
instead of a document that drifts. The proposal names four frontiers: a geometric intuition
manifold composed of `cortex-spatial`, `cortex-imagination` and the hypervector body, in which
conjectures "emerge as minimum-energy attractor basins"; inductive predicate invention by the
inverse-resolution operators (absorption, identification, intra-construction) over `TermNode`
and a "Ramanujan machine" search for polynomial continued fractions in `cortex-arithmetic`;
an "aesthetic" drop in free energy turned into a dopamine reward-prediction error that
consolidates a discovery through three-factor plasticity; and a two-track verification whose
second track writes a prover's certificate into `SemanticOntologyNode` fail-closed. Four of its
stated premises are not in the tree (the Context below says which), so the round begins by
re-deriving them. When the round is done: **one ADR** puts definite clauses on the term arena
of [ADR-0025](../docs/adr/0025-term-arena-and-unification.md) (a clause is a compound over a
reserved functor: its head and up to seven body literals) and gives `cortex-reasoning` the
inductive operators as bounded rules over caller slices: Plotkin's least general
generalisation of two terms, absorption, identification, and intra-construction, which invents
a predicate from a reserved band of ids and whose outputs resolve back to their inputs (the
identity a test holds for every case), plus the one definite-clause resolution step that
states that identity; every bound a result, no allocation, no recursion. **One ADR** gives
`cortex-arithmetic` its first sequencing of slots into an expression: the convergents of a
polynomial continued fraction $a_0 + b_1/(a_1 + b_2/(a_2 + \dots))$ with $a_n$ and $b_n$
polynomials of degree at most two in $n$, computed exactly through the scratchpad slot with
overflow as a flag; an exact test of a convergent against a target rational within a
tolerance; and a bounded enumeration of coefficient tuples that reports the ones whose
deepest comparable convergent lies within the tolerance, so that a target the caller supplies
as a rational yields candidate identities the broker can be asked to certify. **One ADR** puts
the discovery path in the runtime with no executor field: the description length of a clause
store in nodes is the free energy `InteroceptiveState::update_valence` reads (the identity of
minimum description length and Helmholtz free energy is Hinton and Zemel's), an invention's
drop is its valence, a quarter of the valence clamped to $[-1, 1]$ is the reward-prediction
error `Executor::reward` takes (the scale the mirth already uses), and the next presynaptic
spike consolidates the pending eligibility trace, which the exit test pins; and a completed
prover frame whose payload carries the statement hash and a certificate hash certifies the
node, every other status leaving it untouched. Finding F-31 records the certificate-hash
sentence the tree contradicted and the two "Specified" statuses the proposal asserted that no
document carried; hypothesis H-11 names what the reward path is not claimed to do. The
manifold, an interoceptive record in the image, the broker itself, a cryptographic hash, a
meet-in-the-middle search and an "aesthetic threshold" on dispatch are not adopted, with the
reasons. No record in the image changes: format 13 stays and the determinism pin is untouched.
The whitepaper, README, `CLAUDE.md`, the reader's guide, the ADR index and the changelog say
all of this, and this brief is archived with every check green.

---

## Standing directives

- Every claim is Implemented, Specified, Target or Hypothesis. What a search over 2 401
  coefficient tuples reports for a rational near $e$ is stated as what it is; whether a
  description-length drop consolidating a trace is anything a later behaviour reads is a
  hypothesis with a protocol ([ADR-0010](../docs/adr/0010-measured-or-target.md)). The words
  "discovers", "understands", "aesthetic" and "intuition" describe the proposal, never the tree.
- The repository wins over the document; a disagreement is a numbered finding in whitepaper
  §11, never a silent edit.
- No `f32`/`f64`, in the crates and in the tests; a rational target is two integers. Q16.16 in
  `i32`/`u32`, widened to `i64` to multiply; every operation on a state field saturates or
  wraps by name (`clippy::arithmetic_side_effects` is denied everywhere,
  [ADR-0029](../docs/adr/0029-structural-enforcement.md)); a shift amount is bounded a line
  above the shift; exact arithmetic that must not saturate goes through
  `ArithmeticScratchpadSlot` or a `checked_*` operation whose `None` is a result.
- 64-byte `#[repr(C, align(64))]` records with compile-time assertions; no heap types, threads
  or `unsafe` in a state crate (`unsafe_code = "forbid"`, ADR-0029); no recursion whose depth
  the input decides: a walk uses a caller slice as its work stack and an exhausted slice is a
  result ([ADR-0025](../docs/adr/0025-term-arena-and-unification.md)).
- Every quantity has one owner ([ADR-0016](../docs/adr/0016-thirty-two-crate-architecture.md)):
  a clause is a term, so it lives in `cortex-reasoning`; a convergent is exact arithmetic, so
  it lives in `cortex-arithmetic`; a valence is `cortex-affect`'s, a reward `cortex-neuromod`'s,
  a certified theorem `cortex-knowledge`'s; what joins them lives in `runtime/cortex-runtime`,
  the one crate that depends downward ([ADR-0023](../docs/adr/0023-executor.md)). No new crate;
  no new record; the crate count stays 32.
- A change to a record bumps `CortexFileHeader::FORMAT_VERSION` (rule L-6). This round changes
  no record and adds none to the image, so the format stays 13.
- State crates declare no dependencies (TC-2).
- Rule L-3: a term holds ids, never strings; a clause's predicate is a concept id.
- The engine never amends its own code ([ADR-0031](../docs/adr/0031-policy-amendment.md)); an
  invented predicate is an id in a term, not a rule of the engine.
- The determinism pin of [ADR-0030](../docs/adr/0030-verification-governance.md) moves only
  with a stated reason; the mutation gate on the changed lines must pass; a new rule carries
  a test over the lattice of `testkit/prop.rs`; every pinned number is computed by an
  independent oracle before the test that asserts it is written.
- No product name enters a crate (§6.10: the prover the broker runs is its operator's
  configuration; the directives `LEAN4` and `SMT_Z3` absent stay).
- Conventional Commits with a real body; never commit on `main`; the required checks keep
  their names.

## Context

Re-derived on 2026-09-13 against `main` at `c4e99e8`. Line numbers move; the symbols and the
quoted sentences are what to re-derive.

1. **`cortex-reasoning` has clauses of two propositional literals and terms, not clauses of
   terms.** `crates/cortex-reasoning/src/lib.rs`: `pub type Clause = (u32, u32)`, `resolve`,
   `SymbolicRuleNode::apply_resolution`. `crates/cortex-reasoning/src/term.rs`: `TermNode`
   (`kind`, `arity`, `functor`, `children: [u32; MAX_ARITY]` with `MAX_ARITY` 8, index + 1),
   `unify(a, b, arena, bindings, trail, stack) -> (UnifyResult, usize)`, `deref`, `undo`,
   `resolve_first_order` over two-literal clauses of term literals.
   `crates/cortex-reasoning/src/category.rs`: `CATEGORY_FORWARD` `0xFFFF_FF01`,
   `CATEGORY_BACKWARD` `0xFFFF_FF02`, `CATEGORY_RESERVED` `0xFFFF_FF00` ("a caller's concept
   ids stay below it"), `ParseScratch`, the greedy `reduce`. No rule generalises two terms,
   inverts a resolution step, measures a term's size or hashes one; nothing names an inductive
   operator. Whitepaper §5.2.30's Specified list is "Clause search, standardising apart, a
   proof store, constraint propagation, type raising and a chart"; induction is in no document.
   The proposal's gap statement is right about the crate and wrong that the gap was Specified.
2. **`cortex-arithmetic` has eight opcodes and no expression.** `crates/cortex-arithmetic/src/lib.rs`:
   `ArithmeticScratchpadSlot` (two `i128` operands as `u64` low and `i64` high words, a result,
   `error_flags`, `opcode`), `execute` with `OP_ADD`..`OP_DIV_Q16`, flags `ERR_OVERFLOW`,
   `ERR_DIVIDE_BY_ZERO`, `ERR_UNKNOWN_OP`; the crate doc: "the sequencing of slots into an
   expression is Specified". No document in the tree contains "continued fraction",
   "Ramanujan", "lattice" or "meet-in-the-middle" (`grep -ri` over `docs/`, `briefs/`,
   `README.md`, `CHANGELOG.md`): the proposal's "remain Specified" asserts a status no
   document carries. Part of F-31.
3. **The valence rule exists; its free energy does not; the executor holds no affect.**
   `crates/cortex-affect/src/lib.rs`: `InteroceptiveState::update_valence(free_energy_q16) -> i32`
   sets `valence_df_dt_q16` to $F_{\text{prev}} - F_{\text{now}}$ clamped and moves
   `existential_stake_q16` toward $\lvert \Delta F \rvert$; `free_energy_prev_q16` is a public
   field. Whitepaper §5.2.23 Status: "The free energy itself (`cortex-predictive`) ...
   Specified". `runtime/cortex-runtime/src/executor.rs` contains no `InteroceptiveState`
   (`grep -n affect`), and `Executor::reward(rpe)` is documented as "an input, like an
   injection, so a run that replays its rewards at the same ticks is the same run".
   `crates/cortex-neuromod/src/lib.rs`: `NeuromodulatorState::reward` "the mirth of
   `cortex-affect` arrives here as a quarter of itself".
   `crates/cortex-core/src/dynamics/synapse.rs`: `SynapseBlock::consolidate` clamps the
   modulation to $[0, 1]$, so a non-positive reward on a zero baseline consolidates nothing.
   `runtime/cortex-runtime/tests/modulation.rs` is the test that makes a trace pend and a
   reward consolidate it (`network`, `fire`, `pairing`, `synapse`); the exit test reuses its
   shape. The proposal's "no active signal path" is right; its remedy (a path inside the
   executor) would need a record the executor writes every window, and nothing in the tree
   computes a free energy for it to write.
4. **Certification is Implemented; the certificate hash has no home.**
   `crates/cortex-knowledge/src/lib.rs`: `certify(statement_hash)` stores the hash in
   `property_vector_hash`, sets `AFFORDANCE_CERTIFIED_THEOREM` (bit 31), counts the
   consolidation; `is_certified_theorem`; six tests. `crates/cortex-tools/src/lib.rs`:
   `ToolInvocationFrame::new_call(call_id, category, opcode, param_hash, level)`, `start`,
   `complete(payload)` (at most `PAYLOAD_BYTES` = 32), `fail`, `deny`, `payload()`;
   `TOOL_CATEGORY_FORMAL_PROVER` `0x0004`, `ACTION_VERIFY_PROOF` `0x0001`,
   `ACTION_SOLVE_CONSTRAINTS` `0x0002`, `ACTION_SYMBOLIC_EVAL` `0x0003`. Whitepaper §6.10 step
   4: "the broker ... returns the certificate hash in the payload"; its close: "the engine
   stores a certificate hash"; §5.2.29's table: `property_vector_hash` "for a certified
   theorem, the hash of its statement". The two hashes are different quantities; the record
   holds the statement's; no document defines the bytes of a prover's payload; the hash is a
   `u32`, not a cryptographic digest. The proposal calls the callback Specified; the record
   rule is Implemented and the runtime coupling is what is missing. Part of F-31.
5. **The canvas has no energy and the grid no potential.** `crates/cortex-imagination/src/lib.rs`:
   `MentalCanvasFrame::step(delta_valence, uncertainty_growth, dt)` accumulates what the caller
   supplies; the crate doc: "the generative model that supplies the deltas is Specified";
   `wander` perturbs the valence by a linear congruential draw scaled by the temperature.
   `crates/cortex-spatial/src/lib.rs`: `integrate`, `fix`; "the grid-cell attractor ... is
   Specified". A manifold composed of three records none of which computes a potential is
   three records; the proposal's Frontier I has no rule to write until the generative model
   exists, and that is the item §8.8 already calls Specified.
6. **Three premises of the proposal's preamble.** "Closed-loop Self-Organized Criticality
   ($\sigma \approx 1.0$)": [ADR-0036](../docs/adr/0036-criticality-control.md) says under
   "What is not claimed" that the branching ratio settles at 1 on a network; H-8 is open.
   "442 passing tests": `cargo test --workspace --locked` at `c4e99e8` runs 446 (445 passed,
   1 ignored). "23.86-hour": [ADR-0037](../docs/adr/0037-sleep-regulation.md) says it.
7. **The reserved id bands.** `runtime/cortex-runtime/src/language.rs`: `ROLE_CONCEPT_BASE`
   `0xFFFF_FE00`; `category.rs`: `CATEGORY_RESERVED` `0xFFFF_FF00`. An invented predicate's id
   must sit below both, and a `const _` assertion in each place that can see both must say so.
8. **Where the round's numbers go.** The next ADR is 0041 (`ls docs/adr`); the next finding
   F-31 and hypothesis H-11 (whitepaper §11, §11.1); the next reference 66 (Appendix D); the
   whitepaper moves 4.8.0 → 4.9.0; the property kit is `testkit/prop.rs` via
   `include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../testkit/prop.rs"))`; the mutation
   gate's exclusions are in `.cargo/mutants.toml`; the determinism pin is
   `PINNED_ARENA_HASH` `0x1724f3486c1d674e` in `runtime/cortex-runtime/tests/differential.rs`.

<!-- @assert-absence target="crates/cortex-reasoning" symbol="fn intra_construct" glob="*.rs" reason="brief 021 precondition: no inductive operator exists yet" -->
<!-- @assert-absence target="crates/cortex-arithmetic" symbol="fn convergent" glob="*.rs" reason="brief 021 precondition: no continued fraction exists yet" -->
<!-- @assert-absence target="runtime/cortex-runtime/src" symbol="fn certify_from_frame" glob="*.rs" reason="brief 021 precondition: no certification from a frame exists yet" -->

## Deliverables

- [ ] **The induction ADR (the next free number), induction on the term arena** (`docs/adr/0041-induction-on-the-term-arena.md`,
  `depends-on: ADR-0025`; `crates/cortex-reasoning/src/induce.rs`, re-exported from `lib.rs`).
  A definite clause is a compound over `CLAUSE` (`0xFFFF_FF03`, above `CATEGORY_RESERVED`,
  asserted at compile time distinct from the slashes) whose first child is the head and whose
  remaining children, at most `MAX_BODY` = 7, are positive body literals (terms); `clause`,
  `is_clause`, `clause_head`, `clause_body_len`, `clause_literal`. An invented predicate takes
  the next id of the band `INVENTED_BASE` `0xFFFE_0000` to `INVENTED_LIMIT` `0xFFFF_0000`
  (asserted below `CATEGORY_RESERVED`; the runtime asserts `ROLE_CONCEPT_BASE` at or above
  the limit). `InduceScratch` mirrors `ParseScratch`: arena and `free`, bindings, trail and
  `trail_len`, work stack, a pair table for the generalisation, `next_variable`,
  `next_invented`. Rules, each over caller slices, each bound a result, every failure leaving
  the arena, the bindings and the counters as they were: `size` (a term's node count through
  the bindings), `term_hash` (FNV-1a 32 over the pre-order walk: kind, functor, arity),
  `free_variables` (the unbound variables of a term, first occurrence, deduplicated), `lgg`
  (Plotkin: equal constants stay, compounds of one functor and arity generalise child by
  child, any other pair is one variable per distinct pair), `absorb(c2, c)` (from
  $q \leftarrow A$ and $p \leftarrow A', B$ with $A'$ an instance of $A$ literal by literal,
  first fit in order, to $p \leftarrow q\theta, B$), `identify(c1, c)` (from
  $p \leftarrow q, B$ and $p \leftarrow A, B$ to $q \leftarrow A$; exactly one literal of
  $c_1$ unmatched), `intra_construct(ca, cb)` (from $p \leftarrow A, B_1$ and
  $p \leftarrow A, B_2$ to $p \leftarrow A, q(V)$, $q(V) \leftarrow B_1$, $q(V) \leftarrow B_2$
  with $q$ new and $V$ the variables of $p \leftarrow A$ that occur in $B_1$ or $B_2$, in order
  of first occurrence, at most `MAX_ARITY`; refused when $A$, $B_1$ or $B_2$ is empty), and
  `resolve_definite(goal, rule)` (the first literal of the goal that unifies with the rule's
  head is replaced by the rule's body). `InduceError::{Malformed, NoMatch, NothingToInvent,
  ArenaFull, BoundExceeded, PairsFull, TooManyArguments, BodyFull, InventionsExhausted}`.
  Tests: the generalisation's textbook cases (`f(a, b, a)` with `f(c, b, c)` is `f(X, b, X)`;
  a variable against a constant is a new variable; different functors are one variable; a term
  with itself is itself); each operator on a named example with the identity checked by
  `resolve_definite` (the outputs resolve back to the inputs, literal for literal through the
  bindings, in some order); every error variant reached; the arena, the bindings and the
  counters unchanged after every failure; a property walk over random clause pairs from a
  small vocabulary holding the identities for every success and the restoration for every
  failure. Every number a test pins comes from an oracle outside the tree first.
- [ ] **The fractions ADR (the number after it), exact continued fractions** (`docs/adr/0042-continued-fractions.md`,
  `depends-on: ADR-0016`; `crates/cortex-arithmetic/src/fraction.rs`, re-exported). `Poly`
  = `[i64; 3]`, $c_0 + c_1 n + c_2 n^2$; `poly_at`. `Convergent { p, q, depth }`;
  `convergent(a, b, depth, slot)` runs $p_n = a_n p_{n-1} + b_n p_{n-2}$,
  $q_n = a_n q_{n-1} + b_n q_{n-2}$ from $(p_{-1}, p_0, q_{-1}, q_0) = (1, a_0, 0, 1)$ through
  the slot's `OP_MUL` and `OP_ADD`, a flag ending the walk as `FractionError::Arithmetic(flags)`
  with the slot left showing the operation that failed; `MAX_DEPTH` 64; `deepest` the last
  depth that fits. `within(c, target, tolerance, slot)` reduces $p/q$ by their gcd and decides
  $\lvert p\,t_d - t_n\,q \rvert \cdot o_d \le o_n \cdot \lvert q\,t_d \rvert$ through the slot,
  an overflow a result. `search(target, tolerance, degree, bound, depth, slot, out)` enumerates
  every coefficient tuple with $\lvert c_i \rvert \le$ `bound` for $i \le$ `degree`, skips
  an $a$ or $b$ that is identically zero, walks each candidate's convergents to `depth` or the
  first overflow, remembers the verdict of the deepest depth at which `within` could be
  decided, and writes the candidates whose verdict is within into `out`, in enumeration order,
  `SearchError::OutFull` when one more would not fit and `SearchError::Bound` for a degree
  above two, a negative bound, a depth above `MAX_DEPTH`, a zero denominator or a non-positive
  tolerance. `Candidate::statement_hash` (FNV-1a 32 over the six coefficients) is the
  `param_hash` a frame carries for it. Tests, pinned by the oracle in the brief's execution
  record: for the target $2718281828459045 / 10^{15}$ with tolerance $1/10^{12}$, degree 1,
  bound 3, depth 20, exactly one candidate, $a = [3, 1, 0]$, $b = [0, -1, 0]$, at depth 20 with
  $p = 2916471173788403280463$ and $q = 1072909785605898240000$ ($e = 3 - 1/(4 - 2/(5 - \dots))$);
  for $2414213562373095 / 10^{15}$ with degree 0, bound 2, depth 40, exactly one,
  $a = [2, 0, 0]$, $b = [1, 0, 0]$, $p = 4217293152016490$, $q = 1746860020068409$
  ($1 + \sqrt 2$); an overflow at the depth the oracle says with the flag set; a zero
  denominator; every `Bound` case; `OutFull`; a property walk comparing `convergent` against a
  checked `i128` reference over random small polynomials.
- [ ] **The discovery ADR (the number after that), the discovery path** (`docs/adr/0043-discovery-path.md`, `depends-on:
  ADR-0032`; `runtime/cortex-runtime/src/discovery.rs`, re-exported; runtime `Cargo.toml`
  gains path dependencies on `cortex-affect`, `cortex-tools` and `cortex-knowledge`).
  `description_length(store, arena, bindings, stack)` is the saturating sum of the sizes of a
  clause store's clauses; `prime(affect, length)` writes the length, as whole nodes in Q16.16
  saturating at the format's ceiling, into `free_energy_prev_q16`; `invent(ca, cb, store,
  scratch, affect)` runs `intra_construct`, computes the store's length after (the two inputs
  replaced by the three outputs), calls `update_valence` once with it, and returns
  `Discovery { invention, length_before, length_after, valence_q16, reward_q16 }` with the
  reward the valence shifted right by `COMPRESSION_REWARD_SHIFT` = 2 and clamped to
  $[-1, 1]$; the caller passes it to `Executor::reward`, as the exit test does.
  `conjecture_frame(call_id, statement_hash, level)` is a pending `ACTION_VERIFY_PROOF` frame
  whose `param_hash` is the statement hash. `certify_from_frame(frame, statement_hash, node)`
  certifies the node and returns the certificate hash only when the frame is completed, its
  category the prover, its opcode a verification or a solve, its `param_hash` the statement
  hash, and its payload at least `CERTIFICATE_BYTES` = 8 with `[0..4)` the statement hash and
  `[4..8)` a non-zero certificate hash, little-endian; every other frame leaves the node
  untouched and names why (`CertifyError::{NotCompleted, NotAProver, NotAVerification,
  Mismatch, NoCertificate}`). Exit test `runtime/cortex-runtime/tests/discovery.rs`: two
  clauses of one head, three common literals and one differing literal each (ids from a
  vocabulary table that exists only in the test) are intra-constructed on a primed affect
  state; the description length falls from the pinned value to the pinned value, the valence
  and the reward are pinned (the reward at 1.0); the reward into the modulator of a network
  with a pending trace (the shape of `modulation.rs`) consolidates the trace at the next
  presynaptic spike, the weight gaining what the trace lost; an invention whose saving is not
  positive (one common literal) gives a non-positive reward and consolidates nothing; the
  invention's `term_hash` is pinned; a conjecture frame completed with that hash and a
  certificate certifies a node, and every refusal leaves it unchanged. Every pinned number is
  computed by an oracle outside the tree first.
- [ ] **Finding F-31** in whitepaper §11: §6.10's "stores a certificate hash" against
  `certify(statement_hash)`; the payload bytes of a prover's result defined nowhere; the two
  statuses the proposal asserted that no document carried ("continued fractions Specified",
  "the certification callback Specified"). Disposition: resolved by the discovery ADR's payload layout
  and the corrected sentences; the certificate hash lives in the completed frame's payload,
  the node holds the statement's.
- [ ] **Hypothesis H-11** in §11.1: that a description-length drop consolidating pending
  eligibility traces, on a network that computes anything, biases what a later behaviour reads
  toward the invention (compression progress as a reward: Schmidhuber's theory is about
  curiosity and aesthetics; what the tree holds is a mechanism whose exit test moves one
  weight); and that the first-fit matcher of the three operators finds the matchings the
  clause stores of a later round need (the greedy gap of H-10, restated for induction).
- [ ] **The documents.** Whitepaper 4.9.0: the executive summary's sentence on what exists;
  §1.6 rows (`cortex-reasoning`, `cortex-arithmetic`, `cortex-affect`, `cortex-tools`,
  `cortex-knowledge`, dates); §5.2.21 (the payload layout of a prover's result, under the
  frame's table); §5.2.23 (the description length as one free energy the rule reads, Partial);
  §5.2.29 (the sentence on which hash the node holds); §5.2.30 (Public API, Status, an
  "Induction" paragraph with directives `CLAUSE`, `fn intra_construct`, `fn lgg`); §5.2.31
  (Public API, Status "Sequencing ... Implemented for the convergent", a "Continued fractions"
  paragraph with directives `fn convergent`, `fn search`); §6.10 (R-10 becomes Partial: the
  discovery path, the conjecture generators, the certificate round trip on the record side;
  the broker Specified); §8.8 rows (induction; continued fractions; the reward path; the
  deduction row's status); §9 three rows; §11 F-31; §11.1 H-11; Appendix C M8 clause;
  Appendix D references 66 onward (Plotkin 1970; Muggleton and Buntine 1988; Muggleton 1991;
  Hinton and Zemel 1994; Schmidhuber 2009; Rissanen 1978; Raayoni et al. 2021; Wall 1948 or
  Lorentzen and Waadeland 1992 for continued fractions); the glossary (Clause, Least general
  generalisation, Predicate invention, Convergent, Description length). README (the
  Implemented row's clause; the reasoning and arithmetic rows' rule names), `CLAUDE.md` (the
  sentence on what exists), `docs/zh-TW/README.md` (§6 and §11 rows), `docs/adr/README.md`
  (three rows), `CHANGELOG.md` (one entry under Unreleased in the shape of brief 020's).
- [ ] **Not adopted, with the reason in the ADR that is closest:** the geometric intuition
  manifold and the "energy-based" attractor (no potential exists to descend; the generative
  model of the canvas is the Specified item, the discovery ADR's consequences); an interoceptive record
  in the image (nothing in the executor writes a free energy; a record the loop carries and
  never updates is the class F-30 named; the discovery ADR); the broker process and its ring
  (milestone M8; the discovery ADR does the record side only); a cryptographic certificate (the record
  has 32 bits; the discovery ADR); the meet-in-the-middle search over rational functions of the target
  (Specified in the fractions ADR: the direct comparison suffices at this bound and no measurement asks
  for more); an "aesthetic selection threshold" gating dispatch (the veto gate is the gate on
  every frame; which conjectures to dispatch is the executive's Specified search, the discovery ADR);
  inverse substitution beyond `lgg` (the induction ADR: the operators produce instances, the
  generalisation is a separate rule the caller composes).
- [ ] **This brief archived** under `briefs/archive/` with the frozen banner, every box
  dispositioned, the precondition directives removed and the links rebased.

## Not empowered

- No executor field, no image section, no format bump: the affect state, the clause store and
  the scratch are the caller's, as the language module's codebook and categories are. A later
  round that gives the executor a loop which computes a free energy every window is the round
  that puts the record in the image.
- No new crate and no new record: a clause is a `TermNode`; a convergent is two `i128` in a
  scratchpad slot; a candidate is a return value.
- No float anywhere, including the tests: the oracle that computes $e$ or $1 + \sqrt 2$ to
  fifteen digits runs outside the tree and its output enters the test as two integers.
- No recursion that the input bounds; no `Vec`, `Box` or `String` in a state crate; no
  `#[allow]`.
- No word of the proposal's vocabulary in a document's claims: a rule is what it does.
- No product name; no change to the veto gate; no change to `ToolInvocationFrame`'s state
  machine or bytes.
- No renaming of the CI jobs the ruleset requires; no move of the determinism pin.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in
the ADR that owns it: the matching discipline of the operators (first fit in order, or
another deterministic order), the band constants, the shape of `InduceScratch`, the degree and
bound of the search, the payload layout, the reward's shift, the exit test's example. It may
not reach the standing directives, the whitepaper's invariants or the constraints in
`CLAUDE.md`.

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
each correction went; the clause encoding, the four inductive rules and the identity each
holds, what first-fit matching cannot find; the convergent's recurrence, what the search over
2 401 tuples reports for the rational near $e$ and over 20 for the one near $1 + \sqrt 2$, and
where it overflows; the discovery path's numbers (the description length before and after,
the valence, the reward, the weight and the trace before and after the spike) and what the
non-positive case shows; the certificate round trip's layout and the five refusals; that the
format and the pin are untouched and why; what the mutation gate found on the changed lines
and how each survivor was answered; what was not done (the manifold, the record in the image,
the broker, the cryptographic hash, the meet-in-the-middle, the dispatch threshold, inverse
substitution) and why; and what the re-examination after the round recommends next.
