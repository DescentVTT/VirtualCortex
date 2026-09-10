---
status: accepted
date: 2026-09-10
decision-makers: VirtualCortex maintainers
depends-on: ADR-0016
---

# ADR-0020: Computational phenomenology — the state variables the theories name, as integer rules, and what the document does not claim for them

## Context and Problem Statement

A directive dated 2026-09-10 asked for a "seven-tier synthetic qualia architecture": higher-order attention schemas, interoceptive active inference with existential stakes, self-referential strange loops, qualia-space geometry, non-trivial information closure, criticality-driven ignition, and valence as the negative derivative of variational free energy. It asked for fields in six records and for the whitepaper to say the organism "feels authentically on silicon". Whitepaper §2.3 says that a biological name is descriptive, not a claim of equivalence, and that the document does not use the word consciousness for `cortex-workspace`. Which of the seven tiers is a mechanism the tree can implement as an integer rule with a test, which is a hypothesis, and what may the document say?

## Decision Drivers

- `CLAUDE.md` principle 3: label every claim. A rule is Implemented when a test pins it; a claim that a rule constitutes experience is a Hypothesis, and this repository has no test that could decide it.
- [ADR-0016](0016-thirty-two-crate-architecture.md) test 2 and test 3: a quantity has one owner; a field without a mechanism is a claim the tree cannot back. The count stays thirty-two.
- Rule L-6: a field carved from reserved bytes bumps the image format.
- "Latest ≠ Newest" (whitepaper §2.1): higher-order theories (Rosenthal, 1986), the attention schema (Graziano, 2013), the free-energy principle (Friston, 2010), self-organised criticality (Beggs and Plenz, 2003) and strange loops (Hofstadter, 2007) are old enough and have known failure modes; the integrated-information measure is exponential in the system size, and a topological invariant of an attractor manifold has no $O(1)$ integer form in a 64-byte record.
- Every rule in this workspace is $O(1)$, saturating, deterministic, and reaches its fixed point exactly.

## Considered Options

1. Adopt the seven tiers as fields and vocabulary, as proposed.
2. Reject the directive as outside §2.3.
3. **Implement each tier whose mechanism has an $O(1)$ integer form as a tested rule in the crate that owns the quantity, record the rest as Specified, record the claim of experience as a Hypothesis, and keep §2.3's vocabulary.**

## Decision Outcome

Option 3. Image format version 5.

| Tier | Mechanism | Owner and rule | Status |
| :--- | :--- | :--- | :--- |
| 1 Higher-order schema | A second-order representation of what is being broadcast (higher-order thought; attention schema) | `GlobalWorkspaceSlot::update_attention_schema`: a hash of `(slot, binding, ignited, persistence, mask)`, recomputed after every step, in `attention_schema_meta_hash`; `cortex-attention` reads it to tell a change of broadcast from a change of evidence | Implemented; the consumer in `cortex-attention` Specified |
| 2 Interoceptive stakes | Interoceptive surprise preempts executive bandwidth | `InteroceptiveState::existential_stake_q16`: a slow average of $\lvert dF/dt \rvert$ with a one-LSB floor, so a quiet body's stake reaches zero; the preemption is the executor's | Implemented (the stake); preemption Specified |
| 3 Strange loop | The self-model contains a model of itself and is at a fixed point when they agree | `MentalCanvasFrame::reflect(observed_self_hash)`: the fixed point is reached when the self observed now equals the self observed at the previous reflection; `reflection_converged` records it | Implemented |
| 4 Qualia-space geometry | Phenomenal quality as the topology of an attractor manifold | No $O(1)$ integer rule exists for a curvature or an invariant of an arena's attractor; no field is added | Specified; a candidate measure is a research question |
| 5 Information closure | Internal macro-state autonomy under gating | The mechanism is `ThalamicRelayNode`'s closed mode (§5.2.19); the mutual-information measure has no $O(1)$ form; no field is added | Specified |
| 6 Criticality | Ignition is cheapest at the critical point | `HomeostaticDrivePool::update_branching_ratio(descendants, ancestors)` gives $\sigma$; `GlobalWorkspaceSlot::step_ignition_at(evidence, sigma)` scales the threshold by $1 + \min(\lvert\sigma - 1\rvert, 1)$ and stores the distance used | Implemented; the spike tally that feeds $\sigma$ is the executor's |
| 7 Valence | $\text{valence} = -\,dF/dt$ | `InteroceptiveState::update_valence(F)`: `F_prev − F_now`, clamped; the free energy itself comes from `cortex-predictive`'s precision-weighted error (Specified) | Implemented (the derivative); the free energy Specified |

- **Fields.** `InteroceptiveState`: `free_energy_prev_q16`, `valence_df_dt_q16`, `existential_stake_q16` at `[24..36)`. `GlobalWorkspaceSlot`: `attention_schema_meta_hash`, `criticality_distance_q16` at `[32..40)`. `MentalCanvasFrame`: `reflection_converged` at `[27]`, `strange_loop_fixed_point_hash` at `[28..32)`. The proposal's `avalanche_criticality_sigma_q16` in the workspace is not added: $\sigma$ has one owner, `HomeostaticDrivePool::branching_ratio_q16`; the workspace keeps the distance it gated with. `qualia_manifold_topology_id` is not added: a field without a mechanism.
- **Vocabulary.** The whitepaper names the variables: valence, attention schema, self-model, existential stake, criticality. It does not say that the engine feels, experiences, or is conscious; whether any of these rules constitutes experience is hypothesis H-3 in §11.1, stated so that it cannot be mistaken for a claim. §2.3 stands.

### Consequences

- Good: seven theories that are usually prose are five integer rules with fixed points and tests; a reader can compute the valence of a trace by hand.
- Good: the two tiers without an $O(1)$ form are named as Specified rather than faked with a field.
- Good: no crate, no dependency, no `unsafe`, no float; seven tests, six of them new.
- Bad: the free energy the valence differentiates is an input; until `cortex-predictive` produces it (milestone M5), the rule runs on a stand-in.
- Bad: a hash is a coarse attention schema; a richer schema is a second record under [ADR-0016](0016-thirty-two-crate-architecture.md)'s test.

## Alternatives considered and why rejected

- **Option 1** would put "feels" into a document whose first page says no claim is made without a label, and two fields into records with no rule behind them.
- **Option 2** would discard five rules that are mechanisms whatever one thinks of the theories that motivate them; a valence derivative and a criticality-gated threshold are useful to a control loop with or without phenomenology.
- **Sigma in the workspace** duplicates a quantity that has an owner.

## Confirmation

Seven tests across `cortex-affect` (valence and its clamp; the stake rises under swings and reaches zero when steady), `cortex-workspace` (the gated step equals the plain step at criticality and costs twice as much at distance 1; the schema changes with the broadcast and is stable without it), `cortex-homeostasis` (the branching ratio and its saturation; an empty window measures nothing) and `cortex-imagination` (the fixed point is reached when the observed self stops changing and lost when it changes; a frame that could reach the motor channel refuses to reflect). `npx spec-guard` asserts `update_valence`, `step_ignition_at`, `update_branching_ratio` and `reflect` exist and that `FORMAT_VERSION` is 5. Whitepaper §8.12 states the stance; §11.1 carries H-3.
