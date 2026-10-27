# Menteion

**A deterministic, single-node neuromorphic virtual-actor engine for spiking neural computation, written in Rust.**

[![CI](https://github.com/Menteion/Menteion/actions/workflows/ci.yml/badge.svg)](https://github.com/Menteion/Menteion/actions/workflows/ci.yml)
[![License: Apache-2.0 OR MIT](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg)](#license)
[![Rust: 1.85+](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](docs/adr/0009-rust-edition-and-msrv.md)

Menteion builds a spiking neural network on the virtual-actor model and constrains it to one physical server. Neural units are passive 64-byte records that occupy memory only while active; a fixed pool of core-pinned workers services them; ownership per tick is decided by a single atomic gate; axonal delay is an index into a timing wheel; and inactive tissue is evicted to local storage. Every quantity is Q16.16 fixed point, so that a run is bit-identical on x86-64 and AArch64: CI checks a seeded 128-unit network's arena hash after 20 000 ticks on both (ADR-0030); the full form, a reference image over 10⁶ ticks, is Target T-1. The founding rule is **"Latest ≠ Newest"**: only technologies with a stable specification, years of production use and known failure modes are admitted to the hot path.

## Status

Where the work stands, and the next decision, is in [docs/STATUS.md](docs/STATUS.md), which every round rewrites whole. Read the labels before anything else:

| Label | Meaning | Today |
| :--- | :--- | :--- |
| **Implemented** | In `crates/`, checked by the compiler, a test or an executable assertion. | 32 `#![no_std]` crates and 43 `#[repr(C)]` records with compile-time size and alignment assertions, the executor, the `.menteion` image and the learning loop of [docs/STATUS.md](docs/STATUS.md). Subsystem by subsystem in whitepaper [§1.6](docs/WHITEPAPER.md#16-implementation-status-at-a-glance). |
| **Specified** | Designed in the whitepaper or an ADR; no code yet. | Core pinning, the `mmap` path of the image loader, slot reclamation, the shared-memory mappings of the embodiment and tool rings, the tool broker and its amendment register, reclamation, fabric transport, subsystem dynamics beyond the rules in §5. |
| **Target** | A measurable goal with a protocol; **not yet measured**. | Every performance figure. A benchmark harness exists; no admissible run on the reference platform does. |
| **Hypothesis** | A research assumption that must be validated first. | The condensation ratio behind any whole-brain-scale claim. Every hypothesis asked of the engine, with its verdict, is in [docs/research/](docs/research/README.md). |

Where a document and the repository disagree, the repository wins and the disagreement is a numbered finding in the whitepaper's §11.

## Workspace

Thirty-two crates, one per subsystem, with no dependencies between them ([ADR-0005](docs/adr/0005-crate-per-subsystem.md); fourteen admitted by [ADR-0016](docs/adr/0016-thirty-two-crate-architecture.md)). Each exports one primary state record; sizes are asserted at compile time.

| Layer | Crate | Primary record | Size |
| :--- | :--- | :--- | ---: |
| Foundation | `menteion-core` | `DendriticSuperNeuron`, `SynapseBlock`, `PlasticDelta`, `FlatTimingWheel` (`WorkerWheel`) | 64 B, 64 B, 16 B, 4.2 MB |
| Structure | `menteion-connectome` | `MenteionFileHeader`, `SectionEntry` | 64 B each |
| Periphery | `menteion-sensory` | `SensoryEvent`, `trait SensoryPeripheral` | 8 B |
| Periphery | `menteion-thalamus` | `ThalamicRelayNode` | 64 B |
| Periphery | `menteion-embodiment` | `EmbodimentRingBuffer`, `TorqueFrame`, `JointStateFrame`, `VocalFrame` | 64 B each |
| Periphery | `menteion-linguistic` | `LinguisticFrameSlot` | 64 B |
| Periphery | `menteion-tools` | `ToolInvocationFrame` | 64 B |
| Subcortical | `menteion-basal-ganglia` | `BasalGangliaChannelState` | 64 B |
| Subcortical | `menteion-cerebellum` | `CerebellarMicrozone` | 64 B |
| Subcortical | `menteion-salience` | `SalienceNodeState` | 64 B |
| Subcortical | `menteion-neuromod` | `NeuromodulatorState` | 16 B |
| Subcortical | `menteion-hippocampus` | `HippocampalAttractorState`, `Episode` | 64 B each |
| Subcortical | `menteion-homeostasis` | `HomeostaticDrivePool` | 64 B |
| Subcortical | `menteion-affect` | `InteroceptiveState` | 64 B |
| Subcortical | `menteion-autonomic` | `AutonomicVitalsState` | 64 B |
| Subcortical | `menteion-spatial` | `SpatialGridCoordinate` | 64 B |
| Subcortical | `menteion-curiosity` | `CuriosityExplorationVector` | 64 B |
| Subcortical | `menteion-attention` | `FovealAttentionFocus` | 64 B |
| Cognitive | `menteion-workspace` | `GlobalWorkspaceSlot` | 64 B |
| Cognitive | `menteion-symbolic` | `SymbolicHypervectorHeader`, `HypervectorBody` | 64 B, 1 280 B |
| Cognitive | `menteion-executive` | `ExecutivePlanNode`, `PolicyAmendment` | 64 B each |
| Cognitive | `menteion-predictive` | `PredictiveErrorState` | 64 B |
| Cognitive | `menteion-agency` | `AgentPerspectiveState` | 64 B |
| Cognitive | `menteion-social` | `SocialPerspectiveNode` | 64 B |
| Cognitive | `menteion-ethics` | `EthicalEvaluationGate` | 64 B |
| Cognitive | `menteion-knowledge` | `SemanticOntologyNode` | 64 B |
| Cognitive | `menteion-reasoning` | `SymbolicRuleNode`, `TermNode` | 64 B each |
| Cognitive | `menteion-arithmetic` | `ArithmeticScratchpadSlot` | 64 B |
| Cognitive | `menteion-imagination` | `MentalCanvasFrame` | 64 B |
| Systems | `menteion-immune` | `ImmuneScrubNode` | 64 B |
| Systems | `menteion-fabric` | `FabricPacketHeader` | 64 B |
| Systems | `menteion-telemetry` | `LfpSamplePacket` | 64 B |

Exact field layouts, the numeric model, the concurrency rules and the status of every subsystem are in the whitepaper, §5 and §8. Two members sit outside `crates/`: `runtime/menteion-runtime`, the executor that composes the state crates ([ADR-0023](docs/adr/0023-executor.md)), and `benches/menteion-bench`, which holds the benchmarks and the workspace's only third-party dependency (the harness, as a dev-dependency; [ADR-0014](docs/adr/0014-benchmark-harness.md)); see [docs/benchmarks/README.md](docs/benchmarks/README.md) for what makes a run admissible.

<!-- @assert-count target="Cargo.toml" symbol="crates/menteion-" expected="32" reason="the table above lists thirty-two crates" -->

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

`npm run spec` runs [`spec-guard`](https://www.npmjs.com/package/@descent-vtt/spec-guard), which executes the `<!-- @assert-* -->` directives embedded in the documents against `crates/`; [`spec-graph`](https://www.npmjs.com/package/@descent-vtt/spec-graph), which checks that the documents are consistent with one another (links, ADR lifecycle, open questions); [`spec-brief`](https://www.npmjs.com/package/@descent-vtt/spec-brief), which holds every live brief to its contract and every archived brief to the hash it was archived with (ADR-0185); and dependency-free scripts, among them one that checks every archived brief carries that hash and one that checks the state crates declare no dependencies (ADR-0029). The three tools are pinned to exact versions; all four run as blocking checks in [CI](.github/workflows/ci.yml), beside the Rust jobs (check, tests in both profiles, format, lints, rustdoc, the benchmark smoke run, the MSRV, the AArch64 determinism pin) and the mutation gate on every pull request's changed lines (ADR-0030); Appendix B of the whitepaper lists the levels.

Two of those scripts hold the documents' shape ([ADR-0197](docs/adr/0197-one-place-for-each-kind-of-fact.md)). One writes the ADR index, the whitepaper's §9 and the hypotheses' tables from front matter. The other holds the text a change adds or modifies to its limits, and every accepted ADR's body to its base copy.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). In short: one crate per subsystem, no floats, no allocation on the hot path, every record 64 bytes and asserted, every claim labelled, every decision an ADR. A biological or phenomenological name is descriptive, never a claim of equivalence: the whitepaper implements the variables the theories name as tested integer rules and records whether they constitute experience as a hypothesis it does not assert (§8.12).

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
