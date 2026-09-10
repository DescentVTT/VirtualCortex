---
status: accepted
date: 2026-09-10
decision-makers: VirtualCortex maintainers
depends-on: ADR-0016
---

# ADR-0021: Native cognitive language — nested constructions, conceptual blending, default-mode wandering and dialogue grounding, without a language model

## Context and Problem Statement

The same directive asked for "four pillars of native cognitive language": recursive construction grammar beyond the five flat templates of `LinguisticFrameSlot`, conceptual blending in vector-symbolic form that maps bodily vitals to metaphor, default-mode divergent thinking in `cortex-imagination`, and dialogue grounding with turn-taking and a register that follows trust in `cortex-social`. [ADR-0016](0016-thirty-two-crate-architecture.md) fixed that language stays inside the engine and that no external model is part of the system. Which of the four is an integer rule with a test, and where does each live?

## Decision Drivers

- Self-containment ([ADR-0016](0016-thirty-two-crate-architecture.md)): no model, no heap, no strings; the lexicon that turns concept ids into words is the runtime's.
- Nesting without allocation or recursion in a `no_std` crate: a child is an index into the caller's arena, and the walk is the runtime's, bounded by the arena.
- Conceptual blending (Fauconnier and Turner, 2002) has a vector-symbolic form, $\text{blend} = \text{target} \otimes M \oplus \text{source}$, where the cross-domain map $M$ is a permutation (Plate, 1995); the header can record it, the arithmetic is over the bodies in their arena.
- Determinism (§8.3): "stochastic" wandering is a seeded generator whose trace is reproducible.
- Grounding in dialogue (Clark, 1996) is a small state machine: who holds the floor, whether a repair is pending, what is common ground.

## Considered Options

1. Add the four pillars as fields and prose.
2. **Implement each pillar as a tested rule in the crate that owns it, with the vector arithmetic, the lexicon and the conversational policy Specified.**

## Decision Outcome

Option 2. Image format version 5 (shared with [ADR-0020](0020-computational-phenomenology-and-synthetic-qualia.md)).

| Pillar | Owner and rule | Status |
| :--- | :--- | :--- |
| I Recursive constructions | `LinguisticFrameSlot`: `parent_frame_idx`, `child_frame_idx`, `bind_child(child, self)` (refused for self-nesting or a sealed frame), `set_parent`; templates `TEMPLATE_RELATIVE` (the object slot is a clause) and `TEMPLATE_CAUSAL` (a reason follows the core); `ROLE_CHILD` is a position in the realisation order at which the runtime descends, bounded by the arena; a template that nests is complete only with its child bound | Implemented; the descent and the lexicon Specified |
| II Conceptual blending | `SymbolicHypervectorHeader::blend(target, source, domain_mask, cross_domain_shift)`: the target as the bound role, the source in `blend_source_id`, $M$ as `permutation_shift`, the domain bits accumulated, the depth counted; `InteroceptiveState::metaphor_source_domain()` maps the dominant bodily condition to a domain (heat, weight, dusk, calm); `LinguisticFrameSlot::attach_metaphor(blend_id)` puts the blend in the frame | Implemented (the headers and the map); the vector arithmetic and the words Specified |
| III Default-mode wandering | `MentalCanvasFrame::wander()`: a seeded generator advances, the hypothetical action drifts, a valence perturbation of magnitude up to `dmn_wander_temperature_q16` is taken as a one-tick step with the temperature as its uncertainty growth; a zero temperature moves the action but not the valence | Implemented; when the organism wanders (quiescence, `cortex-homeostasis`) Specified |
| IV Dialogue grounding | `SocialPerspectiveNode`: `dialogue_turn_state` (idle, self, other, repair), `take_turn`, `yield_turn`, `request_repair` (counted), `ground(referent)` (mixed into `shared_intentionality_hash`, resolves a repair), `close_exchange`; `register()` maps trust to a politeness level (familiar ≥ 0.75, courteous ≥ 0.25, formal below) that `cortex-linguistic` realises | Implemented; the conversational policy Specified |

- **Fields.** `LinguisticFrameSlot`: `parent_frame_idx`, `child_frame_idx`, `blended_metaphor_id` at `[36..44)`; gate bits `GATE_CHILD_BOUND`, `GATE_METAPHOR`. `SymbolicHypervectorHeader`: `blend_source_id`, `blending_domain_mask`, `blend_depth`, `_pad` at `[32..40)`; `FLAG_BLENDED`. `MentalCanvasFrame`: `dmn_wander_temperature_q16`, `wander_state` at `[32..40)`. `SocialPerspectiveNode`: `turn_repair_count`, `dialogue_turn_state`, `shared_intentionality_hash` at `[33..40)`.
- **What stays outside.** Words. Every rule here produces ids, bits, orders and hashes; the lexicon that renders them in a language is the runtime's, and no sentence in the whitepaper claims the output is poetry.

### Consequences

- Good: constructions nest without allocation; a blend is a header any frame can carry; wandering is reproducible; a conversation has a state a test can drive through repair and grounding.
- Good: the body-to-metaphor map is a deterministic function of the record, so the same body always offers the same domain.
- Bad: the templates are still an enumeration; a grammar that composes templates from smaller constructions is a second record under [ADR-0016](0016-thirty-two-crate-architecture.md)'s test.
- Bad: one child slot per frame; a frame with two clauses nests them in a chain.

## Alternatives considered and why rejected

- **Option 1** would document four capabilities the tree could not exercise.
- **A metaphor lexicon in the crate** would be strings in a `no_std` state crate; ids and domains are the crate's, words are the runtime's.
- **A random source for wandering** would break §8.3; a seeded generator gives the same musing twice, which is what a test needs.

## Confirmation

Ten tests, seven of them new, across `cortex-linguistic` (a relative frame needs its child and realises it in the object position; a causal frame realises its reason last; self-nesting and sealed frames are refused; a metaphor attaches once), `cortex-symbolic` (a blend records target, source, map and domain, accumulates domains and saturates its depth; no domain is refused), `cortex-affect` (the domain follows the dominant condition with a stated tie order), `cortex-imagination` (wandering is deterministic, bounded by the temperature, and a zero temperature moves only the action; a frame that could reach the motor channel refuses to wander) and `cortex-social` (the floor passes by the rules and refuses the rest; common ground accumulates in order and only inside an exchange; the register follows the trust). `npx spec-guard` asserts `bind_child`, `fn blend`, `wander` and `take_turn` exist.
