---
status: accepted
date: 2026-09-10
decision-makers: VirtualCortex maintainers
depends-on: ADR-0021
---

# ADR-0026: Social acumen and re-representation — a sincerity gap and a second-level expectation in `cortex-social`, tact and indirectness in `cortex-linguistic`, an anomaly that marks a concept's framework stale in `cortex-knowledge`, and a basis rotation in `cortex-symbolic`

## Context and Problem Statement

A directive dated 2026-09-10 asked for two "universal faculties": social acumen (recursive theory of mind of depth two or more, a "cognitive immune shield" against manipulation, and compassionate tact) and autonomous paradigm innovation (challenging axioms, rotating hypervector bases, restructuring the ontology), with fields in `SocialPerspectiveNode` (`recursive_tom_depth`, `pragmatic_subtext_hash`) and `SemanticOntologyNode` (`paradigm_shift_epoch`, `re_representation_flags`), and asked the document to say the engine "effortlessly sees through manipulation" and produces "genuinely original" synthesis. Whitepaper §2.3 and §8.12 say a name is descriptive and a claim needs a rule and a test. Which parts are integer rules a record can own, and what may the document say?

## Decision Drivers

- `CLAUDE.md` principle 3 and [ADR-0020](0020-computational-phenomenology-and-synthetic-qualia.md)'s stance: a rule is Implemented when a test pins it; whether a rule amounts to "seeing through" anything is a Hypothesis.
- [ADR-0016](0016-thirty-two-crate-architecture.md) test 2: a quantity has one owner; a field without a rule is not added. Depth is derivable from what is stored, so it is a function, not a field.
- [ADR-0021](0021-native-cognitive-language-and-conceptual-blending.md): the register follows trust; a permutation of a hypervector is a change of basis.
- The only detection this engine can do is a comparison of what it recorded: what an agent stated against what followed, what a frame's surface act is against what it means.
- Rule L-6: fields carved from reserved bytes bump the image format.

## Considered Options

1. Add the fields as proposed and describe the faculties in prose.
2. **Implement, in the crate that owns each quantity, the comparisons that exist: the sincerity gap and its slow average; the agent's expectation of the self as the second level of the model, with depth derived; tact as a rule over the register and the valence; indirectness as an intended act beside the surface act; an anomaly average that marks a concept's representation stale and a re-representation that moves it under a new category; a composed basis rotation on the hypervector header; the divergent rollout at the anomaly's temperature. Record the claims of insight as a hypothesis.**

## Decision Outcome

Option 2. Image format version 8 (shared with [ADR-0027](0027-vocal-synthesis-and-computational-humor.md)).

| Faculty | Rule | Owner | Status |
| :--- | :--- | :--- | :--- |
| Sincerity ("immune shield") | `assess_sincerity(stated, outcome)`: the gap $\lvert\text{stated} - \text{outcome}\rvert$ clamped to 1.0; `insincerity_q16` moves toward it by $2^{-3}$ of the distance and at least one LSB; a gap at or above 0.5 is `update_trust(false)`, a smaller one `update_trust(true)`; `is_suspect` at an average of 0.25 | `cortex-social` | Implemented; what a suspect agent's requests cost at the veto gate is the runtime's (Specified) |
| Second-level model ("k-ToM") | `expect_of_self(action)`: what the agent expects the self to do (read from a directive addressed to the self, or a stated prediction); `would_surprise(planned)`; `tom_depth` = 2 when an expectation of the self is held, else 1 when a belief is attributed, else 0, derived, not stored; `close_exchange` clears the expectation, which belongs to the exchange | `cortex-social` | Implemented; the inference that fills the hashes Specified |
| Tact ("face") | `apply_face(register, valence)`: a negative valence to a courteous or formal listener forces the soften marker, opens the particle slot and raises the politeness level; a familiar register is frank; applied last, so tact has the last word | `cortex-linguistic` | Implemented |
| Subtext ("indirect speech") | `mark_indirect(intended_act)`: the frame keeps its surface act and records the act it means; `intended_act`, `is_indirect` | `cortex-linguistic` | Implemented; reading the intent from an utterance is the lexicon's (Specified) |
| Premise check ("challenging axioms") | `note_anomaly(error)`: the prediction error a concept leaves unexplained, averaged with $2^{-3}$ and a one-LSB floor; at 0.5 the representation is marked `REPRESENTATION_STALE`, once, until a re-representation | `cortex-knowledge` | Implemented; which error is the concept's is `cortex-curiosity`'s target and the runtime's join (R-14) |
| Re-representation ("ontological evolution") | `re_represent(epoch, new_parent, rebase_shift)`: only when stale, and only when something changes (a new category or a rotation); the concept moves under a new category, its affordances, mass and hazard stay, the epoch is stamped, the anomaly halves, the count grows, `REPRESENTATION_REBASED` when a rotation was applied | `cortex-knowledge` | Implemented; choosing the new category Specified |
| Basis rotation | `rebase(shift)`: the cyclic permutation advances by `shift` modulo the dimensionality, composing with earlier rotations; `rebase_count`, `FLAG_REBASED` | `cortex-symbolic` | Implemented; the vector arithmetic Specified |
| Divergent rollout | `wander_at(temperature)`: the wander of ADR-0021 at the anomaly | `cortex-imagination` | Implemented |

- **Fields.** `SocialPerspectiveNode`: `expected_of_self_hash` `[40..44)`, `insincerity_q16` `[44..48)`. `SemanticOntologyNode`: `anomaly_q16` `[28..32)`, `paradigm_epoch` `[32..36)`, `representation_flags` `[36..38)`, `re_representations` `[38]`; `[25..28)` reserved. `SymbolicHypervectorHeader`: the padding byte `[39]` is `rebase_count`. `LinguisticFrameSlot`: `intended_speech_act` `[44]` as the act + 1, zero for direct. Not added: `recursive_tom_depth` (derived by `tom_depth`), `pragmatic_subtext_hash` (the intended act is in the frame that carries the act; a hash beside it would be a copy).
- **Vocabulary.** The document names the rules: a sincerity gap, a suspect agent, a second-level expectation, tact, an intended act, an anomaly, a stale representation, a re-representation, a rotation. It does not say the engine sees through anyone, or that a re-representation is original; whether a rotation or a re-categorisation yields a synthesis a reader would call novel is hypothesis H-5 (§11.1).

### Consequences

- Good: eight rules, each a fixed point a test reaches: insincerity and anomaly return to zero exactly; a re-representation needs staleness and staleness fires once.
- Good: no crate, no dependency, no `unsafe`, no float; nine new tests.
- Good: the veto gate of `cortex-ethics` is untouched; a suspect agent changes what its requests are worth at the gate through the runtime, never what the gate refuses.
- Bad: sincerity is judged on valence alone, stated against observed; an agent whose deeds match its words while it manipulates through what it omits is not detected, and the document says so.
- Bad: the second level holds one expectation; a richer model of the other's model is a second record under ADR-0016's test.

## Alternatives considered and why rejected

- **Option 1** would add two hashes and a depth with nothing to compute them from.
- **A `k`-deep recursion of belief hashes** would be hashes of hashes; depth beyond what is observed is not modelled by a record.
- **Rotating the affordance bits on a re-representation** would relabel actions, not categories; the concept's affordances are its own and stay.

## Confirmation

Nine tests: `cortex-social` (the sincerity gap moves insincerity and trust and reaches zero when words match deeds; the second level says what the agent expects of the self), `cortex-linguistic` (an indirect frame keeps its surface act; tact softens bad news in a courteous or formal register only; play yields to tact), `cortex-knowledge` (a persistent unexplained error marks the representation stale once; a re-representation moves the concept, keeps its affordances and needs staleness), `cortex-symbolic` (rotations compose modulo the dimensionality), `cortex-imagination` (wandering at a temperature). `npx spec-guard` asserts `assess_sincerity`, `note_anomaly`, `fn rebase` and `mark_indirect` exist and `FORMAT_VERSION` is 8.
