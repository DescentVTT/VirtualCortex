---
status: proposed
date: 2026-09-10
decision-makers: VirtualCortex maintainers
amends: ADR-0005
depends-on: ADR-0015
---

# ADR-0016: Thirty-two state crates — fourteen subsystems admitted, three boundaries moved, and the admission test for the next one

## Context and Problem Statement

A proposal dated 2026-09-10 asked for fourteen new state crates so that the organism would have, beyond the reflex substrate of the first eighteen, a sensory relay gate, native language, digital actuation, gaze, interoception, hardware vitals, a metric map, an epistemic drive, a model of other agents, a veto gate, world knowledge, rules, exact arithmetic and counterfactual rehearsal. It asked for the whitepaper to become 4.0.0, for the capacity model to be recomputed, and for a decision record.

The first evaluation, recorded in an earlier revision of this change, measured the proposal against the tree and found that eleven of the fourteen overlapped an existing crate or field, that three crossed a boundary the whitepaper had drawn (§1.5, §8.9, §8.10), that two declared Q16.16 in sixteen bits and one used tuple fields with no defined layout, and that the proposal's capacity figure preceded any count. The maintainers answered that the boundaries are theirs to move and that the organism needs the capabilities. This record is that decision: which boundaries move, how each overlap is resolved so that every quantity keeps one owner, what the fourteen crates hold, and what test the thirty-third crate will have to pass.

## Decision Drivers

- The organism must be able to do more than reflex: attend, feel, speak, act on a digital environment, refuse an action, know things, reason, count, imagine, and model the agents around it. Each of those is a subsystem with its own record, which is what [ADR-0005](0005-crate-per-subsystem.md) asks for.
- Self-containment. The maintainers rejected every external language model and transformer runtime: language is realised inside the engine, from hypervector unbinding through construction-grammar frames to a linear recurrent cell, deterministically, in integer arithmetic, in constant memory. A tool call leaves the engine only through a broker the engine does not control and cannot become.
- The invariants stay: `#![no_std]`, no dependencies, no `unsafe`, no floating point, 64-byte `#[repr(C, align(64))]` records asserted at compile time, saturating arithmetic on state fields, formats that fit their widths ([ADR-0001](0001-64-byte-pod-records.md), [ADR-0002](0002-q16-16-fixed-point.md), whitepaper §2.2, §8.1, §8.2).
- `CLAUDE.md` principle 3: a record is a layout, and a layout implements nothing. Every admitted crate therefore carries at least one deterministic rule with boundary tests and a §8.8 row stating the mechanism it is a layout for, so that the whitepaper's status labels stay honest.
- [ADR-0010](0010-measured-or-target.md): the capacity model is computed from counts, not asserted.
- The count is thirty-two. That it is a power of two is incidental and was not a driver.

## Considered Options

1. Admit none; keep the eighteen-crate partition and route the ideas into existing records (the first evaluation).
2. Admit the fourteen as proposed, layouts only.
3. **Admit the fourteen with the overlaps resolved by narrowing the neighbouring responsibilities, the width and layout defects corrected, one tested rule per crate, the three boundaries moved explicitly, and an admission test for future crates.**

## Decision Outcome

Option 3.

### The fourteen crates and what their neighbours keep

| Crate (whitepaper §) | Record | Takes | The neighbour keeps |
| :--- | :--- | :--- | :--- |
| `cortex-thalamus` (§5.2.19) | `ThalamicRelayNode` | The gate between a relayed `SensoryEvent` and its cortical column: tonic, burst (decimating), closed; gain. | `cortex-sensory`: the event type, the driver trait, hot-plug. `cortex-basal-ganglia`: its output *to* the thalamus. |
| `cortex-linguistic` (§5.2.20) | `LinguisticFrameSlot` | The three-layer native language record: construction-grammar frames filled from role unbinding, completeness and emission order per template, and a Q16.16 linear recurrent cell whose energy selects the prosody particle. No external model. | `cortex-symbolic`: the hypervector algebra, including the Layer 1 unbinding that fills the frame. |
| `cortex-tools` (§5.2.21) | `ToolInvocationFrame` | The frame and state machine of a brokered tool call: pending, running, completed, failed, denied. | `cortex-embodiment`: the motor frames and the ring protocol, which the tool ring reuses. |
| `cortex-attention` (§5.2.22) | `FovealAttentionFocus` | Saccade flight and fixation over a gaze target. | `cortex-salience`: the salience that selects the target. `cortex-workspace`: broadcast competition. |
| `cortex-affect` (§5.2.23) | `InteroceptiveState` | Allostatic load from pain, strain and recovery; comfort; mood. | `cortex-homeostasis`: metabolic drives, the circadian gate, criticality. `cortex-salience`: the aversive input itself. |
| `cortex-autonomic` (§5.2.24) | `AutonomicVitalsState` | Voltage, temperature and power in their own units; limits; emergency flags. | `cortex-homeostasis`: the normalised drives derived from them. `cortex-embodiment`: the watchdog heartbeat on the motor ring. |
| `cortex-spatial` (§5.2.25) | `SpatialGridCoordinate` | Position and heading by path integration; landmark fixes. | `cortex-hippocampus`: the episodic attractor and the place field. |
| `cortex-curiosity` (§5.2.26) | `CuriosityExplorationVector` | Per-target novelty, uncertainty and urgency. | `cortex-homeostasis`: the organism's single `curiosity_drive` scalar. |
| `cortex-social` (§5.2.27) | `SocialPerspectiveNode` | One record per other agent: intention, belief, trust, resonance. | `cortex-agency`: self/other attribution of a sensory change. |
| `cortex-ethics` (§5.2.28) | `EthicalEvaluationGate` | The veto gate: imperative, harm, authorization; benefit never overrides. | `cortex-basal-ganglia`: the hyperdirect stop, a motor brake, not a moral one. §8.9: the watchdog stays the last line. |
| `cortex-knowledge` (§5.2.29) | `SemanticOntologyNode` | Consolidated concepts in a category tree with affordances and hazard. | `cortex-symbolic`: transient bindings. `cortex-hippocampus`: the episodes. |
| `cortex-reasoning` (§5.2.30) | `SymbolicRuleNode` | Rule nodes with AND, OR, NOT, IMPLIES, EQUIV over a condition and a parent, and Robinson's resolution on two-literal clauses to the empty clause. | `cortex-executive`: goal-directed plan trees. |
| `cortex-arithmetic` (§5.2.31) | `ArithmeticScratchpadSlot` | Checked 128-bit and Q16.16 arithmetic with error flags. | Every state crate: its own saturating field arithmetic, which is not a scratchpad. |
| `cortex-imagination` (§5.2.32) | `MentalCanvasFrame` | Sandboxed rollout frames that can never release motor output. | `cortex-executive`: plan trees with regret. |

### Corrections made on the way in

- **Widths.** `deontology_score_q16` and `oscillation_phase_q16` were proposed as `u16`; Q16.16 is 32 bits (whitepaper §2.3; finding F-3 was the precedent), so both are `u32`. `empathy_gain` gained its format and width: `empathy_gain_q16: u32`.
- **Layout.** `ArithmeticScratchpadSlot`'s `(u64, i64)` tuple fields have no defined layout inside a `#[repr(C)]` record (§8.2); each operand is a `u64` low word and an `i64` high word, and the crate reassembles the `i128`.
- **Naming.** `affect_modifer_id` is `affect_modifier_id`; `typical_weight_grams_q16` is `typical_mass_grams_q16`, since grams are a mass; `heading_yaw_q16` is `heading_yaw_turns`, a wrapping fraction of a turn under the phase-counter convention of §8.1, because a yaw in Q16.16 radians has no defined wrap.
- **One layout kept byte for byte.** The proposal's third revision gave `LinguisticFrameSlot` as code; that layout is adopted unchanged, and the roles bound so far live in the upper nibble of its `syntax_gate_flags` rather than in a byte the proposal did not have.
- **Defaults.** Every record is `Default` and all-zero. Arrays longer than 32 elements do not implement `Default`, so ten records spell the impl out; the four whose arrays are shorter derive it, as clippy requires.
- **One rule each.** Each crate carries one deterministic, saturating rule with boundary tests, listed in whitepaper §1.6's Logic column; the tests found and fixed three stalls where a decay by right shift could never reach zero (novelty at 3, trust at 7, confidence at 255), which is now a rule: a shift-based decay takes at least one LSB.

### Boundaries moved

- **§1.5 scope.** A brokered digital environment is in scope. The engine still is not a general-purpose actor framework: a tool call is a 64-byte frame in a ring, as a motor command is. Language is realised natively; no external language model is part of the system.
- **§8.10 security.** A tool broker process, outside the worker seccomp filter, holds the only credentials and its own opcode allow-list, checks the `authorization_level` the veto gate wrote into the frame, and runs under its own profile. The worker filter is unchanged: spike trains still cannot escalate inside the engine process. The broker's policy is configuration reviewed like an ADR.
- **§8.9 fail-safe.** Two gates stand inside the engine, in front of the external watchdog and not in place of it: the veto gate of `cortex-ethics` before every motor or tool dispatch, and the emergency flags of `cortex-autonomic`. The sentence "the engine MUST NOT be the only thing standing between a robot and an unsafe configuration" stands.

### Capacity

Appendix A gains fourteen arena rows with stated placeholder counts, as the existing rows have; together they add 0.66 GB and Tier 1 becomes ≈ 19.6 GB, within target T-2's 20 GB. The proposal's "≈ 21.8 GB" was not adopted: Appendix A multiplies sizes by counts, and no count in the proposal produced that figure.

### Image format

`CortexFileHeader::FORMAT_VERSION` stays at 3. No existing record changed; the new arenas become sections of the `.cortex` image when the loader exists (milestone M4), under the section directory of §8.7.

### The count is locked

Thirty-two is the count until an ADR passes the test below. Capabilities that arrive after this record are added to the interaction protocols of these crates, as constants, rules and runtime scenarios, never as a thirty-third crate: the theorem-proving and document-auditing pipelines of whitepaper §6.10 and §6.11 are the first two, carried by `cortex-reasoning` (resolution), `cortex-tools` (two brokered categories and their opcodes), `cortex-knowledge` (certified theorems) and `cortex-attention` (document foveation) without a new record type. A capability that needs a record no crate has is a second record in the crate that owns it, which is the open question §11.1 records, not a crate.

### Tool opcodes are agnostic; mathematics is native first

The opcodes of `cortex-tools` name actions, never products: `ACTION_VERIFY_PROOF`, `ACTION_SOLVE_CONSTRAINTS` and `ACTION_SYMBOLIC_EVAL` under the prover category, and the three structural actions under the document engine. A product name in an identifier would couple the engine to a vendor's release cycle, which is what "Latest ≠ Newest" (whitepaper §2.1) exists to prevent; the first revision of the prover opcodes named two products and was corrected. Mathematics runs on two tracks (whitepaper §6.10): the native track, resolution and exact arithmetic in `cortex-reasoning` and `cortex-arithmetic`, is complete and offline; the brokered track is optional acceleration through whatever system the broker's operator configured, and returns only a certificate hash. The engine is never less capable without the broker, only slower on large searches.

### Admission test for the next crate

A new member under `crates/` requires all of the following in the pull request that adds it:

1. **A named gap.** An ADR names the capability the organism lacks and the subsystem that supplies it. A brief may propose one; a proposal without a record is answered with this one.
2. **No owner already.** No Responsibility row in whitepaper §5.2 covers the quantity and no existing field carries it. Otherwise it is a field in that record's reserved bytes with the format bump of §8.7, not a crate.
3. **A mechanism with the layout.** A §8.8 row states the intended dynamics, and the crate carries at least one deterministic rule with boundary tests. A record whose only content is fields and padding under a promising name is a claim the tree cannot back.
4. **Widths that hold their formats.** Q16.16 is 32 bits; no `repr(Rust)` type inside a `#[repr(C)]` record.
5. **Inside the boundaries.** §1.5 and §8.10 are intact, or moved by that ADR first.
6. **The count moves with it.** The `expected="32"` directives in §1.6, §2.2 and the README, the §1.6 table, the §5.1 diagram, a §5.2 entry with the layout transcribed from source, an Appendix A row with a stated count, the README table, `CLAUDE.md` and `CONTRIBUTING.md` change in the same pull request.

### Consequences

- Good: the organism has the state, and one rule each, for attention, feeling, speaking, acting on a digital environment, refusing, knowing, reasoning, counting, imagining and modelling others; milestone M8 names what is left to make each of them run.
- Good: every quantity still has one owner. A reader who wants "curiosity" finds the organism's drive in `cortex-homeostasis` and the per-target vector in `cortex-curiosity`, and the two rows say so.
- Good: the security story is stronger than a crate that "operates the filesystem" would have made it: the worker filter is unchanged and the broker is a separate, auditable process.
- Good: the next proposal of this shape is answered by a test rather than by a review argument.
- Bad: thirty-two `Cargo.toml` files, thirty-two §5.2 entries, thirty-two rows in every count. Mitigated by inheritance from `[workspace.package]` and by the executable assertions that fail when any count drifts.
- Bad: each new crate has one rule; the dynamics behind it are Specified, like those of the first eighteen. That is the state-model stage (§1.6), and M5 and M8 are the answer.
- Bad: the lexicon, the unbinding, the broker and the ring mappings are runtime work that no state crate can carry; until they exist the language and tool paths are frames without a reader.

## Alternatives considered and why rejected

- **Option 1** was the first evaluation's answer. It kept the partition clean at the cost of the capabilities; the maintainers chose the capabilities and moved the boundaries explicitly rather than leaving them to be crossed silently.
- **Option 2** would have added fourteen layouts under names that promise behaviour, the defect that produced findings F-1 to F-18.
- **An external language model over a shared-memory ring** was the first draft of `cortex-linguistic`. It was rejected for self-containment: a process the engine does not control would have been part of every utterance, and its output is neither deterministic nor integer. The adopted design is the proposal's three-layer pipeline: vector-symbolic unbinding, construction-grammar framing, and a linear recurrent cell $s_{t+1} = \alpha s_t + k_t v_t$ in saturating Q16.16. The whitepaper names the cell by its form, a leaky integrator with a multiplicative input, which linear-attention and RWKV-style models share; it does not name a 2023 model family as a dependency, because whitepaper §2.1 admits mechanisms with a decade of use and the mechanism here is the integrator, not the family.
- **A tool crate that makes the calls itself** would have required the worker filter of §8.10 to admit `socket` and friends; the broker keeps the filter and moves the credentials out of the engine.

## Confirmation

On the change that proposes this decision (2026-09-10):

- `cargo check --workspace --all-targets --locked`, `cargo test --workspace --locked` (145 tests, 75 of them in the fourteen crates), `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings` and the benchmark smoke run pass on the pinned toolchain; `cargo check` and `cargo test` pass on the MSRV 1.85.
- `npx spec-guard` holds the member count at thirty-two in whitepaper §1.6, §2.2 and the README, one test-module assertion per crate, one record assertion per new crate under §5.2.19 to §5.2.32, the recurrent cell of `cortex-linguistic`, and the absence of `String` alongside `Box` and `Vec` under TC-5.
- `spec-graph` checks this record's status and links. Whitepaper 4.0.0 carries the fourteen entries, the moved boundaries and the recomputed capacity model.
