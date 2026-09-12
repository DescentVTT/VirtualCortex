# VirtualCortex

**A deterministic, single-node neuromorphic virtual-actor engine for spiking neural computation, written in Rust.**

[![CI](https://github.com/DescentVTT/VirtualCortex/actions/workflows/ci.yml/badge.svg)](https://github.com/DescentVTT/VirtualCortex/actions/workflows/ci.yml)
[![License: Apache-2.0 OR MIT](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg)](#license)
[![Rust: 1.85+](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](docs/adr/0009-rust-edition-and-msrv.md)

VirtualCortex builds a spiking neural network on the virtual-actor model and constrains it to one physical server. Neural units are passive 64-byte records that occupy memory only while active; a fixed pool of core-pinned workers services them; ownership per tick is decided by a single atomic gate; axonal delay is an index into a timing wheel; and inactive tissue is evicted to local storage. Every quantity is Q16.16 fixed point, so that a run is bit-identical on x86-64 and AArch64: CI checks a seeded 128-unit network's arena hash after 20 000 ticks on both (ADR-0030); the full form, a reference image over 10⁶ ticks, is Target T-1. The founding rule is **"Latest ≠ Newest"**: only technologies with a stable specification, years of production use and known failure modes are admitted to the hot path.

## Status

This repository is at the **state-model stage**. Read the labels before reading anything else:

| Label | Meaning | Today |
| :--- | :--- | :--- |
| **Implemented** | In `crates/`, checked by the compiler, a test or an executable assertion. | 32 `#![no_std]` crates, 42 `#[repr(C)]` records with compile-time size and alignment assertions (thirty-eight of one cache line, the hypervector body of twenty, two of 16 bytes, one of 8), small deterministic update rules with boundary tests in 26 crates (membrane integration, short-term plasticity, synaptic fan-out and three-factor STDP (an eligibility trace per synapse that the modulator consolidates) in the neuron; valence, an attention schema, criticality-gated ignition, a self-model fixed point, nested constructions, conceptual blends, dialogue grounding, first-order unification over a term arena, a sincerity gap, tact, an anomaly that marks a framework stale, an integer source–filter voice and the benign-violation appraisal among them), the turn gate and lock-free mailbox of axiom A3 under a four-thread test, the executor (`runtime/cortex-runtime`: worker threads, work-stealing deques, three barrier-separated phases per tick; 10⁶ events delivered exactly once on four workers; bit-identical arenas on one and four), the `.cortex` image (a sealed section directory, a loader that fails closed, a writer) and the clock sweep with its write-ahead log (evict, spike, re-hydrate bit for bit), zero dependencies, `unsafe` only in the executor's arena under ADR-0023, plain integer arithmetic a build error in every crate (`clippy::arithmetic_side_effects`, ADR-0029 and brief 016); the tests hold in the release profile, a mutation gate on every pull request's changed lines, property tests over a lattice and a seeded walk, and the seeded network's arena hash pinned and checked on x86-64 and AArch64 (ADR-0030); a policy amendment (a parameter of the engine's own policy, from a registry with bounds) trialled in two forks of the image and committed only when they behaved alike and the cost fell, persisted and replayed by the loader; the engine never amends its own code (ADR-0031); an image that says what a tick is and resumes the writer's clock (ADR-0033); the population's spikes tallied every tick, the branching ratio estimated from the tally by lag-one regression once per window, and a bounded global synaptic gain that every turn applies, moved by a bounded step, with the dynamics unchanged at the default step of zero (ADR-0036); a rule slower than the tick on a cadence that is a mask on the tick (ADR-0035); sleep as a state machine stepped once per window (a two-process pressure against thresholds a circadian phase sets, three stages under an ultradian budget, a wake as an input, off by default) and an episodic ledger of tagged patterns, never overwritten, replayed into the network on a ripple cadence in slow-wave sleep so that the three-factor rule consolidates the synapses among them, and depotentiated in REM (ADR-0037, ADR-0038); the hypervector body with its algebra as integer rules (binding, permutation, bundling, the Hamming distance, the nearest codebook entry; no intrinsic) and syntax as type reduction over the term arena (categories as terms, the four combinatory rules as unifications, a greedy shift-reduce reducer), composed by the runtime into a frame read from a category sequence, sealed as a hypervector and read back on ids alone with every distance pinned on both CI targets (ADR-0039, ADR-0040). |
| **Specified** | Designed in the whitepaper or an ADR; no code yet. | Core pinning, the `mmap` path of the image loader, slot reclamation, the shared-memory mappings of the embodiment and tool rings, the tool broker and its amendment register, the lexicon behind the language frames, reclamation, fabric transport, subsystem dynamics beyond the rules in §5. |
| **Target** | A measurable goal with a protocol; **not yet measured**. | Every performance figure. A benchmark harness exists; no admissible run on the reference platform does. |
| **Hypothesis** | A research assumption that must be validated first. | The condensation ratio behind any whole-brain-scale claim. |

Where a document and the repository disagree, the repository wins and the disagreement is a numbered finding in the whitepaper's §11.

## Workspace

Thirty-two crates, one per subsystem, with no dependencies between them ([ADR-0005](docs/adr/0005-crate-per-subsystem.md); fourteen admitted by [ADR-0016](docs/adr/0016-thirty-two-crate-architecture.md)). Each exports one primary state record; sizes are asserted at compile time.

| Layer | Crate | Primary record | Size |
| :--- | :--- | :--- | ---: |
| Foundation | `cortex-core` | `DendriticSuperNeuron`, `SynapseBlock`, `PlasticDelta`, `FlatTimingWheel` (`WorkerWheel`) | 64 B, 64 B, 16 B, 4.2 MB |
| Structure | `cortex-connectome` | `CortexFileHeader`, `SectionEntry` | 64 B each |
| Periphery | `cortex-sensory` | `SensoryEvent`, `trait SensoryPeripheral` | 8 B |
| Periphery | `cortex-thalamus` | `ThalamicRelayNode` | 64 B |
| Periphery | `cortex-embodiment` | `EmbodimentRingBuffer`, `TorqueFrame`, `JointStateFrame`, `VocalFrame` | 64 B each |
| Periphery | `cortex-linguistic` | `LinguisticFrameSlot` | 64 B |
| Periphery | `cortex-tools` | `ToolInvocationFrame` | 64 B |
| Subcortical | `cortex-basal-ganglia` | `BasalGangliaChannelState` | 64 B |
| Subcortical | `cortex-cerebellum` | `CerebellarMicrozone` | 64 B |
| Subcortical | `cortex-salience` | `SalienceNodeState` | 64 B |
| Subcortical | `cortex-neuromod` | `NeuromodulatorState` | 16 B |
| Subcortical | `cortex-hippocampus` | `HippocampalAttractorState`, `Episode` | 64 B each |
| Subcortical | `cortex-homeostasis` | `HomeostaticDrivePool` | 64 B |
| Subcortical | `cortex-affect` | `InteroceptiveState` | 64 B |
| Subcortical | `cortex-autonomic` | `AutonomicVitalsState` | 64 B |
| Subcortical | `cortex-spatial` | `SpatialGridCoordinate` | 64 B |
| Subcortical | `cortex-curiosity` | `CuriosityExplorationVector` | 64 B |
| Subcortical | `cortex-attention` | `FovealAttentionFocus` | 64 B |
| Cognitive | `cortex-workspace` | `GlobalWorkspaceSlot` | 64 B |
| Cognitive | `cortex-symbolic` | `SymbolicHypervectorHeader`, `HypervectorBody` | 64 B, 1 280 B |
| Cognitive | `cortex-executive` | `ExecutivePlanNode`, `PolicyAmendment` | 64 B each |
| Cognitive | `cortex-predictive` | `PredictiveErrorState` | 64 B |
| Cognitive | `cortex-agency` | `AgentPerspectiveState` | 64 B |
| Cognitive | `cortex-social` | `SocialPerspectiveNode` | 64 B |
| Cognitive | `cortex-ethics` | `EthicalEvaluationGate` | 64 B |
| Cognitive | `cortex-knowledge` | `SemanticOntologyNode` | 64 B |
| Cognitive | `cortex-reasoning` | `SymbolicRuleNode`, `TermNode` | 64 B each |
| Cognitive | `cortex-arithmetic` | `ArithmeticScratchpadSlot` | 64 B |
| Cognitive | `cortex-imagination` | `MentalCanvasFrame` | 64 B |
| Systems | `cortex-immune` | `ImmuneScrubNode` | 64 B |
| Systems | `cortex-fabric` | `FabricPacketHeader` | 64 B |
| Systems | `cortex-telemetry` | `LfpSamplePacket` | 64 B |

Exact field layouts, the numeric model, the concurrency rules and the status of every subsystem are in the whitepaper, §5 and §8. Two members sit outside `crates/`: `runtime/cortex-runtime`, the executor that composes the state crates ([ADR-0023](docs/adr/0023-executor.md)), and `benches/cortex-bench`, which holds the benchmarks and the workspace's only third-party dependency (the harness, as a dev-dependency; [ADR-0014](docs/adr/0014-benchmark-harness.md)); see [docs/benchmarks/README.md](docs/benchmarks/README.md) for what makes a run admissible.

<!-- @assert-count target="Cargo.toml" symbol="crates/cortex-" expected="32" reason="the table above lists thirty-two crates" -->

## Documentation

| Document | What it is |
| :--- | :--- |
| [docs/WHITEPAPER.md](docs/WHITEPAPER.md) | The canonical architecture document: arc42 structure, C4 views, per-crate record layouts, runtime scenarios, quality targets, findings, capacity model. Its claims about the tree are executable. |
| [docs/adr/](docs/adr/README.md) | Architecture decision records (MADR). |
| [docs/zh-TW/README.md](docs/zh-TW/README.md) | 繁體中文導讀：how to read the whitepaper, with no layouts or figures of its own. |
| [briefs/](briefs/README.md) | Numbered, self-contained prompts for the next rounds of work; executed briefs are frozen in `briefs/archive/`. |
| [CLAUDE.md](CLAUDE.md) | The switchboard for coding agents: principles, invariants, map, commands. |
| [CONTRIBUTING.md](CONTRIBUTING.md) | Workflow, commit conventions, documentation rules, definition of done. |
| [SECURITY.md](SECURITY.md) | Vulnerability reporting. |
| [CHANGELOG.md](CHANGELOG.md) | Notable changes. |

<!-- @assert-present file="docs/WHITEPAPER.md,docs/adr/README.md,docs/zh-TW/README.md,CONTRIBUTING.md,SECURITY.md,CHANGELOG.md" -->

## Build and verify

Requires Rust 1.85 or newer (edition 2024, [ADR-0009](docs/adr/0009-rust-edition-and-msrv.md)); `rust-toolchain.toml` names the toolchain CI builds with, and rustup selects it in this checkout. The documentation checks need Node 22 or newer.

```bash
cargo check --workspace --all-targets
cargo test --workspace
```

```bash
npm ci
npm run spec
```

`npm run spec` runs [`spec-guard`](https://www.npmjs.com/package/@descent-vtt/spec-guard), which executes the `<!-- @assert-* -->` directives embedded in the documents against `crates/`; [`spec-graph`](https://www.npmjs.com/package/@descent-vtt/spec-graph), which checks that the documents are consistent with one another (links, ADR lifecycle, open questions); and two dependency-free scripts, one that checks every live brief carries its mandatory sections and one that checks the state crates declare no dependencies (ADR-0029). The two tools are pinned to exact versions; all four run as blocking checks in [CI](.github/workflows/ci.yml), beside the Rust jobs (check, tests in both profiles, format, lints, rustdoc, the benchmark smoke run, the MSRV, the AArch64 determinism pin) and the mutation gate on every pull request's changed lines (ADR-0030); Appendix B of the whitepaper lists the levels.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). In short: one crate per subsystem, no floats, no allocation on the hot path, every record 64 bytes and asserted, every claim labelled, every decision an ADR. A biological or phenomenological name is descriptive, never a claim of equivalence: the whitepaper implements the variables the theories name as tested integer rules and records whether they constitute experience as a hypothesis it does not assert (§8.12).

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
