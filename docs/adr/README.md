# Architecture decision records

One file per decision, in [MADR](https://adr.github.io/madr/) format, numbered in order of acceptance. The `status:` front-matter field is read by `spec-graph`, which fails CI if a live document depends on a retired decision or if an open question is delegated to one.

To add a decision: copy the most recent file, take the next number, set `status: proposed`, open a pull request. A decision becomes `accepted` when merged with that status, `superseded` when a later ADR names it in `supersedes:`, and `rejected` if closed without adoption. Never renumber.

| ID | Title | Status |
| :--- | :--- | :--- |
| [ADR-0001](0001-64-byte-pod-records.md) | 64-byte cache-line POD records as the unit of state | accepted |
| [ADR-0002](0002-q16-16-fixed-point.md) | Q16.16 fixed-point arithmetic; no IEEE-754 on the hot path | accepted |
| [ADR-0003](0003-zero-allocation-hot-path.md) | Zero allocation and zero syscalls on the hot path | accepted |
| [ADR-0004](0004-two-tier-timing-wheel.md) | Two-tier timing wheel for axonal delay | accepted |
| [ADR-0005](0005-crate-per-subsystem.md) | One crate per subsystem; no dependencies among state crates | accepted |
| [ADR-0006](0006-virtual-actor-turn-invariant.md) | Virtual-actor turn invariant enforced by an atomic gate | accepted |
| [ADR-0007](0007-cortex-image-format.md) | The `.cortex` memory-mappable image format | accepted |
| [ADR-0008](0008-documentation-governance.md) | Documentation governance: arc42, MADR, BCP 14, executable assertions | accepted |
| [ADR-0009](0009-rust-edition-and-msrv.md) | Rust edition 2024 and a pinned MSRV | accepted |
| [ADR-0010](0010-measured-or-target.md) | Every performance figure is Measured or Target, never asserted | accepted |
| [ADR-0011](0011-epoch-based-reclamation.md) | Epoch-based reclamation for structural plasticity | accepted |
| [ADR-0012](0012-synaptic-weight-q1-15.md) | Sixteen-bit synaptic base weights are Q1.15 | accepted |
| [ADR-0013](0013-timing-wheel-geometry.md) | Timing wheel geometry: 256 × 10 µs fine, 256 × 100 µs coarse, fixed-capacity token lists (amends ADR-0004) | accepted |
| [ADR-0014](0014-benchmark-harness.md) | Benchmark harness: criterion 0.7, confined to a bench-only crate | accepted |
| [ADR-0015](0015-embodiment-frame-abi.md) | Embodiment frame ABI and single-producer single-consumer ring protocol | accepted |
| [ADR-0016](0016-thirty-two-crate-architecture.md) | Thirty-two state crates: fourteen subsystems admitted, three boundaries moved, and the admission test for the next one (amends ADR-0005) | accepted |
| [ADR-0017](0017-mailbox-and-gate-protocol.md) | Mailbox and gate protocol: an index stack drained whole, no ABA tag, four sequentially consistent operations (amends ADR-0006) | accepted |
| [ADR-0018](0018-membrane-integration.md) | Membrane integration: shift leaks with a one-LSB floor, difference coupling, a 2 ms refractory window, apical-gated plateaus, adaptive threshold | accepted |
| [ADR-0019](0019-short-term-plasticity.md) | Short-term plasticity: event-driven Tsodyks–Markram on the Q0.8 fields, exponentials by binary exponentiation | accepted |
| [ADR-0020](0020-computational-phenomenology-and-synthetic-qualia.md) | Computational phenomenology: the state variables the theories name, as integer rules, and what the document does not claim for them | accepted |
| [ADR-0021](0021-native-cognitive-language-and-conceptual-blending.md) | Native cognitive language: nested constructions, conceptual blending, default-mode wandering and dialogue grounding, without a language model | accepted |
| [ADR-0022](0022-synapse-fan-out-and-stdp.md) | Synaptic fan-out and STDP: index + 1 chains, synapse tokens, stored releases, spike messages, the nearest-neighbour pair rule at the presynaptic spike; image format 6 | accepted |
| [ADR-0023](0023-executor.md) | The executor: a runtime crate, in-house work-stealing deques, three barrier-separated phases per tick, and the one `unsafe` in the workspace | accepted |
| [ADR-0024](0024-cortex-image-and-clock-sweep.md) | The `.cortex` image: section directory, table-free CRC-64/XZ, a read-into-arenas loader and writer, a write-ahead log for the clock sweep, and the Tier-2 delta record; format version 7 | accepted |
| [ADR-0025](0025-term-arena-and-unification.md) | A term arena and first-order unification for `cortex-reasoning`: a second record under ADR-0016's test, bindings in a caller's table, a trail undone on failure, bounds that are results | accepted |
| [ADR-0026](0026-social-acumen-and-re-representation.md) | Social acumen and re-representation: a sincerity gap and a second-level expectation, tact and indirectness, an anomaly that marks a framework stale and a re-representation, a basis rotation; what is not claimed | accepted |
| [ADR-0027](0027-vocal-synthesis-and-computational-humor.md) | Vocal synthesis as motor output and computational humor: a `VocalFrame` with an integer source–filter renderer, the benign-violation appraisal, reward and the playful marker; what is not claimed | accepted |
| [ADR-0028](0028-edge-behaviour-audit.md) | Edge behaviour under audit: a gate is closed until evaluated (format version 9), a reflex is released with its override, every packed or shifted input is read within its bound, the loader refuses what the writer would never produce, quiescence includes the injector ring | accepted |
| [ADR-0029](0029-structural-enforcement.md) | Structural enforcement of the invariants that were held by review: workspace lints for `unsafe` and floats, a manifest check for dependencies, rustdoc as a gate, the arithmetic lint where it passes, pinned actions and normalised line endings | accepted |
| [ADR-0030](0030-verification-governance.md) | Verification governance: a mutation gate on the lines a change touches, a zero-dependency property kit with a lattice and a seeded walk, exhaustive tests where the domain allows, the tests in the engine's profile, a determinism pin checked on two architectures; what is not adopted | accepted |
| [ADR-0031](0031-policy-amendment.md) | Self-amendment: a policy amendment on trial in two forks of the image, committed only when the engine behaved exactly as before and cost less; the engine never amends its own code; the amendment register; image format 10 | accepted |
| [ADR-0032](0032-three-factor-plasticity.md) | Three-factor plasticity: an eligibility trace per synapse in the block, consolidated into the weight by the modulator at the presynaptic spike; the apical mask in the chain word; the engine's modulator in the executor and the image; format version 11 | accepted |
| [ADR-0033](0033-tick-duration-in-the-header.md) | The tick in the image header: its duration, refused when it is not this build's, and the clock at the write, which the loader resumes so that every stamp keeps its meaning | accepted |
| [ADR-0034](0034-synaptic-density.md) | Synaptic density: the four-synapse block stands; eight-bit logarithmic weights, eight synapses per block and an in-engine far-memory synapse tier rejected; the density levers in order, each gated by a measurement | accepted |
| [ADR-0035](0035-cadence-and-the-population-tally.md) | Multirate stepping as a cadence: a power-of-two period and a phase, a mask on the tick; the population spike tally; the idle-tick benchmark; conservative lookahead left to a measurement | accepted |
| [ADR-0036](0036-criticality-control.md) | Criticality control: the branching ratio by lag-one regression over bins of population activity in the homeostasis record, a bounded global synaptic gain as the actuator, stepped once per window and persisted in the image; per-unit scaling Specified; format version 12 | accepted |
| [ADR-0037](0037-sleep-regulation.md) | Sleep as a state machine on the window cadence: a two-process sleep pressure against a circadian threshold, three stages under an ultradian budget, a wake as an input; the homeostasis record's sleep fields; format version 13 | accepted |
| [ADR-0038](0038-episodic-ledger-and-replay.md) | The episodic ledger and its replay: an append-only second record of `cortex-hippocampus` under ADR-0016's test, tagged as an input, replayed into the network on a ripple cadence in slow-wave sleep and consolidated by the modulator's rule, depotentiated in REM; the canvas hydration Specified; sections 44 and 45 | accepted |
