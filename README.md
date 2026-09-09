# VirtualCortex

**A deterministic, single-node neuromorphic virtual-actor engine for spiking neural computation, written in Rust.**

[![CI](https://github.com/DescentVTT/VirtualCortex/actions/workflows/ci.yml/badge.svg)](https://github.com/DescentVTT/VirtualCortex/actions/workflows/ci.yml)
[![License: Apache-2.0 OR MIT](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg)](#license)
[![Rust: stable](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org/)

VirtualCortex builds a spiking neural network on the virtual-actor model and constrains it to one physical server. Neural units are passive 64-byte records that occupy memory only while active; a fixed pool of core-pinned workers services them; ownership per tick is decided by a single atomic gate; axonal delay is an index into a timing wheel; and inactive tissue is evicted to local storage. Every quantity is Q16.16 fixed point, so a run is bit-identical on x86-64 and AArch64. The founding rule is **"Latest ≠ Newest"**: only technologies with a stable specification, years of production use and known failure modes are admitted to the hot path.

## Status

This repository is at the **state-model stage**. Read the labels before reading anything else:

| Label | Meaning | Today |
| :--- | :--- | :--- |
| **Implemented** | In `crates/`, checked by the compiler, a test or an executable assertion. | 18 crates, 19 `#[repr(C)]` records (18 of them with compile-time size and alignment assertions), 5 small deterministic update functions, zero dependencies, zero `unsafe`. |
| **Specified** | Designed in the whitepaper or an ADR; no code yet. | Executor, mailboxes, wheel dispatch, image loader, embodiment rings, reclamation, fabric transport, subsystem dynamics. |
| **Target** | A measurable goal with a protocol; **not yet measured**. | Every performance figure. There is no benchmark in the tree yet. |
| **Hypothesis** | A research assumption that must be validated first. | The condensation ratio behind any whole-brain-scale claim. |

Where a document and the repository disagree, the repository wins and the disagreement is a numbered finding in the whitepaper's §11.

## Workspace

Eighteen crates, one per subsystem, with no dependencies between them ([ADR-0005](docs/adr/0005-crate-per-subsystem.md)). Each exports one primary state record; sizes are asserted at compile time.

| Layer | Crate | Primary record | Size |
| :--- | :--- | :--- | ---: |
| Foundation | `cortex-core` | `DendriticSuperNeuron`, `SynapseBlock`, `FlatTimingWheel` | 64 B, 64 B, 2 248 B |
| Structure | `cortex-connectome` | `CortexFileHeader` | 64 B |
| Periphery | `cortex-sensory` | `SensoryEvent`, `trait SensoryPeripheral` | 8 B |
| Periphery | `cortex-embodiment` | `EmbodimentRingBuffer` | 64 B |
| Subcortical | `cortex-basal-ganglia` | `BasalGangliaChannelState` | 64 B |
| Subcortical | `cortex-cerebellum` | `CerebellarMicrozone` | 64 B |
| Subcortical | `cortex-salience` | `SalienceNodeState` | 64 B |
| Subcortical | `cortex-neuromod` | `NeuromodulatorState` | 16 B |
| Subcortical | `cortex-hippocampus` | `HippocampalAttractorState` | 64 B |
| Subcortical | `cortex-homeostasis` | `HomeostaticDrivePool` | 64 B |
| Cognitive | `cortex-workspace` | `GlobalWorkspaceSlot` | 64 B |
| Cognitive | `cortex-symbolic` | `SymbolicHypervectorHeader` | 64 B |
| Cognitive | `cortex-executive` | `ExecutivePlanNode` | 64 B |
| Cognitive | `cortex-predictive` | `PredictiveErrorState` | 64 B |
| Cognitive | `cortex-agency` | `AgentPerspectiveState` | 64 B |
| Systems | `cortex-immune` | `ImmuneScrubNode` | 64 B |
| Systems | `cortex-fabric` | `FabricPacketHeader` | 64 B |
| Systems | `cortex-telemetry` | `LfpSamplePacket` | 64 B |

Exact field layouts, the numeric model, the concurrency rules and the status of every subsystem are in the whitepaper, §5 and §8.

<!-- @assert-count target="Cargo.toml" symbol="crates/cortex-" expected="18" reason="the table above lists eighteen crates" -->

## Documentation

| Document | What it is |
| :--- | :--- |
| [docs/WHITEPAPER.md](docs/WHITEPAPER.md) | The canonical architecture document: arc42 structure, C4 views, per-crate record layouts, runtime scenarios, quality targets, findings, capacity model. Its claims about the tree are executable. |
| [docs/adr/](docs/adr/README.md) | Architecture decision records (MADR). |
| [docs/zh-TW/README.md](docs/zh-TW/README.md) | 繁體中文導讀：how to read the whitepaper, with no layouts or figures of its own. |
| [CONTRIBUTING.md](CONTRIBUTING.md) | Workflow, commit conventions, documentation rules, definition of done. |
| [SECURITY.md](SECURITY.md) | Vulnerability reporting. |
| [CHANGELOG.md](CHANGELOG.md) | Notable changes. |

<!-- @assert-present file="docs/WHITEPAPER.md,docs/adr/README.md,docs/zh-TW/README.md,CONTRIBUTING.md,SECURITY.md,CHANGELOG.md" -->

## Build and verify

Requires stable Rust and, for the documentation checks, Node 22 or newer.

```bash
cargo check --workspace --all-targets
cargo test --workspace
```

```bash
npm ci
npm run spec
```

`npm run spec` runs [`spec-guard`](https://www.npmjs.com/package/@descent-vtt/spec-guard), which executes the `<!-- @assert-* -->` directives embedded in the documents against `crates/`, and [`spec-graph`](https://www.npmjs.com/package/@descent-vtt/spec-graph), which checks that the documents are consistent with one another (links, ADR lifecycle, open questions). Both are pinned to exact versions and run as blocking checks in [CI](.github/workflows/ci.yml). Formatting and clippy also run in CI, advisory until the findings they report are closed.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). In short: one crate per subsystem, no floats, no allocation on the hot path, every record 64 bytes and asserted, every claim labelled, every decision an ADR.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
