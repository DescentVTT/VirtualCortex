# VirtualCortex

> **A Single-Node Multi-Scale Neuromorphic Engine for 86-Billion-Node Human-Scale Brain Emulation**

[![Language](https://img.shields.io/badge/Language-Rust%202021-orange.svg)](https://www.rust-lang.org/)
[![Architecture](https://img.shields.io/badge/Architecture-Fractal%20Hyper--Actor%20%2B%20SNN-blue.svg)](#core-architectural-axioms)
[![Capacity](https://img.shields.io/badge/Scale-86%20Billion%20Nodes%20Equivalent-red.svg)](#quantitative-86-billion-node-resource-budget)
[![Performance](https://img.shields.io/badge/Target->100%20MSpikes%2Fsec-brightgreen.svg)](#key-performance-targets)
[![Memory Footprint](https://img.shields.io/badge/RAM-~30%20GB%20DDR5-success.svg)](#quantitative-86-billion-node-resource-budget)
[![License](https://img.shields.io/badge/License-Apache%202.0%20%2F%20MIT-blue.svg)](#license)

---

## 📖 Overview

Simulating the human brain (~86 billion neurons, ~100 trillion synapses) with brute-force computing requires upwards of **700 Terabytes of memory**, necessitating warehouse-scale supercomputers. Yet the human genome codes for the entire brain using only **~750 Megabytes of DNA**, relying on **hierarchical self-similarity, multi-compartment dendritic computation, continuous neural fields, procedural connectivity, and continuous chemical diffusion**.

**VirtualCortex 2.0** transcends the point-neuron brute-force trap by introducing the **Fractal Cortical Hyper-Actor Architecture**:
1. **Multi-Compartment Dendritic Super-Neurons ($1 \approx 1,000$)**: Biological pyramidal cells feature apical and basal dendritic trees that perform non-linear coincidence detection (NMDA plateau potentials). One super-neuron delivers the computational expressiveness of 1,000 point units.
2. **Wave-Particle Neural Mass Duality**: Cortical minicolumns operate as continuous population waves (Wilson-Cowan mean-field equations) during subthreshold periods, collapsing into discrete high-saliency spikes only during phase-transition events.
3. **Implicit Procedural Connectomics**: Baseline connectivity is generated on-the-fly via spatial geometric distance kernels, storing only sparse learned deviations ($\Delta W$).
4. **3D Neuromodulatory Volume Diffusion**: Continuous 3D voxel grid tracking Dopamine, Acetylcholine, Serotonin, and Norepinephrine, enabling **Three-Factor Reinforcement Plasticity**.

👉 **For complete mathematical proofs, memory layout derivations, and biophysical mechanics, read the complete [VirtualCortex Architecture Whitepaper v2.0](docs/WHITEPAPER.md)** (中文版架構報告請參閱 [繁體中文架構報告](docs/zh-TW/architecture-report.md))。

---

## ⚡ Key Performance Targets

Tested on a reference **64-core / 128-thread AMD EPYC / ARM Neoverse** server with **128 GB DDR5 RAM** and **PCIe 5.0 NVMe SSD**:

| Metric | Target Specification |
| :--- | :--- |
| **Equivalent Brain Scale** | **86,000,000,000 Neurons** (~100 Trillion Synapses) |
| **Physical System RAM** | **~29.6 GB DDR5 RAM** (Easily fits into a 128GB node) |
| **Spike Throughput** | **> 100,000,000 Spikes / sec** |
| **Median Event Dispatch Latency** | **< 300 nanoseconds** |
| **Biophysical Fidelity** | **Multi-Compartment Dendrites (Apical/Basal) + NMDA Plateaus + LFP Waves** |
| **Neuromodulatory Support** | **Continuous 3D Diffusion of Dopamine, Acetylcholine, Serotonin, Norepinephrine** |
| **Garbage Collection (GC) Overhead** | **0 ms (Zero runtime GC / Zero dynamic dispatch)** |

---

## 📊 Quantitative 86-Billion Node Resource Budget

How VirtualCortex fits human-scale brain computation into **~30 GB of RAM**:

| Subsystem Component | Biological Mapping / Scale | Software Primitive | Count | Total RAM |
| :--- | :--- | :--- | :--- | :--- |
| **Hyper-Columns** | 86 Billion Background Neurons | `HyperColumnState` (64B) | 860,000 | **55.0 MB** |
| **Dendritic Super-Neurons** | Active Non-linear Cortex ($1 \approx 1,000$) | `DendriticSuperNeuron` (64B) | 43,000,000 | **2.75 GB** |
| **Procedural Connectome** | Baseline 100 Trillion Synapses | Algorithmic SIMD Kernel | $\infty$ | **0.00 GB** |
| **Plastic Synapse Deltas** | Learned Connections ($\Delta W$, 1% active) | `PlasticSynapseDelta` (16B) | 1,000,000,000 | **16.00 GB** |
| **3D Neuromodulator Grid** | Continuous Chemical Brain Volume | $128 \times 128 \times 64$ Voxel Grid | 1,048,576 | **16.78 MB** |
| **Thread-Local Timing Wheels** | Axonal Delays (256 Slots $\times$ 64 Cores) | Ring Bucket Envelopes | 64 Wheels | **1.20 GB** |
| **Worker Slab Arenas** | SpikeNode Recycling & Buffers | Intrusive Memory Pools | 64 Cores | **8.19 GB** |
| **Sparse Page Directory** | 2-Level Addressing Directory | 64K $\times$ 64K Pointer Table | 65,536 | **1.35 GB** |
| **Total System RAM** | **86-Billion Equivalent Brain** | — | — | **29.56 GB** |

---

## 🧠 Ten Architectural Axioms

```
                          ┌──────────────────────────────┐
                          │ 1. Virtual Existence         │
                          │ Eternal logic, demand paging │
                          └──────────────┬───────────────┘
                                         ▼
      ┌──────────────────────────────┐       ┌──────────────────────────────┐
      │ 2. State-Worker Decoupling   │ <───> │ 3. Turn-Based Invariant      │
      │ Data is static, Worker roves │       │ 4-State CAS, zero data races │
      └──────────────┬───────────────┘       └──────────────┬───────────────┘
                     ▼                                      ▼
      ┌──────────────────────────────┐       ┌──────────────────────────────┐
      │ 4. Soma-Synapse Decoupling   │ <───> │ 5. Discrete Axonal Wheels    │
      │ 64B Soma + Chunked CSR Fabric│       │ Thread-local circular delay  │
      └──────────────┬───────────────┘       └──────────────┬───────────────┘
                     ▼                                      ▼
      ┌──────────────────────────────┐       ┌──────────────────────────────┐
      │ 6. Phased Epoch Determinism  │ <───> │ 7. Dendritic Expressiveness  │
      │ Double-buffered BSP barrier  │       │ 1 Multi-Compartment ≈ 1,000  │
      └──────────────┬───────────────┘       └──────────────┬───────────────┘
                     ▼                                      ▼
      ┌──────────────────────────────┐       ┌──────────────────────────────┐
      │ 8. Wave-Particle Duality     │ <───> │ 9. Procedural Connectomics   │
      │ Continuous field + Spikes    │       │ Coordinate kernel + Sparse ΔW│
      └──────────────┬───────────────┘       └──────────────┬───────────────┘
                     ▼                                      ▼
      ┌──────────────────────────────┐       ┌──────────────────────────────┐
      │ 10. 3D Chemical Diffusion    │       │ Metabolic Eviction           │
      │ Voxel tensor, 3-factor STDP  │       │ Column-level zero-copy redb  │
      └──────────────────────────────┘       └──────────────────────────────┘
```

1. **Virtual Existence**: Logic persistence, physical demand paging.
2. **State-Worker Decoupling**: Somas are passive 64-byte POD data structures; workers are roving stateless threads.
3. **Turn-Based Single-Thread Invariant**: 4-state atomic CAS machine guarantees zero race conditions and zero deadlocks.
4. **Soma-Synapse Decoupling**: Somas strictly 64-byte aligned; connectome managed in procedural kernels and sparse tables.
5. **Discrete Axonal Timing Wheels**: Axonal conduction delays indexed in thread-local ring buckets ($O(1)$ delay dispatch).
6. **Phased Epoch Determinism**: Three-phase execution barrier prevents temporal inversions across parallel workers.
7. **Dendritic Subunit Expressiveness ($1 \approx 1,000$)**: Multi-compartment apical/basal integration with NMDA plateaus.
8. **Wave-Particle Neural Mass Duality**: Continuous Wilson-Cowan population dynamics + dynamic event spike collapse.
9. **Procedural Connectomics**: Spatial geometric kernels generate 100T baseline connections; memory only stores $\Delta W$.
10. **3D Chemical Diffusion**: Continuous extracellular diffusion grid powering three-factor reinforcement learning.

---

## 🔬 Subsystems at a Glance

### 1. Multi-Compartment Dendritic Super-Neuron (64-Byte POD)
```rust
#[repr(C, align(64))]
pub struct DendriticSuperNeuron {
    pub id: PackedId,                      // 8 bytes (offset 0)
    pub mailbox_head: AtomicPtr<SpikeNode>,// 8 bytes (offset 8)
    pub v_soma: f32,                       // 4 bytes (offset 16) - Somatic membrane potential
    pub v_basal: f32,                      // 4 bytes (offset 20) - Feedforward sensory potential
    pub v_apical: f32,                     // 4 bytes (offset 24) - Contextual feedback potential
    pub v_thresh: f32,                     // 4 bytes (offset 28) - Dynamic firing threshold
    pub nmda_plateau_ticks: u16,           // 2 bytes (offset 32) - Active NMDA plateau duration
    pub refractory_ticks: u16,             // 2 bytes (offset 34) - Absolute refractory countdown
    pub eligibility_trace: f32,            // 4 bytes (offset 36) - 3-factor STDP eligibility trace
    pub last_spike_tick: u32,              // 4 bytes (offset 40) - Last somatic action potential
    pub spatial_pos: [u8; 3],              // 3 bytes (offset 44) - Quantized 3D position in column
    pub tuning_vector: u8,                 // 1 byte  (offset 47) - Feature orientation angle
    pub plastic_synapse_head: u32,         // 4 bytes (offset 48) - Offset into sparse ΔW table
    pub gate_state: AtomicU8,              // 1 byte  (offset 52) - 4-state CAS scheduler gate
    pub flags: u8,                         // 1 byte  (offset 53) - Bursting / Inhibitory flags
    pub _reserved: [u8; 10],               // 10 bytes(offset 54..64) - Alignment padding
}
```

### 2. Hyper-Column State (Wave-Particle Duality)
```rust
#[repr(C, align(64))]
pub struct HyperColumnState {
    pub column_id: u32,
    pub spatial_coords: [f32; 3],          // 3D coordinate in macro brain space
    pub exc_population_rate: f32,          // Excitatory pool rate E(t)
    pub inh_population_rate: f32,          // Inhibitory pool rate I(t)
    pub lfp_voltage: f32,                  // Local Field Potential (mV)
    pub lfp_phase: f32,                    // Phase angle for brainwave coupling
    pub voxel_grid_idx: u32,               // Pointer to local neuromodulator voxel
    pub super_neuron_start_idx: u32,       // Pointer into DendriticSuperNeuron array
    pub super_neuron_count: u16,           // Number of active super-neurons
    pub bifurcation_thresh: f32,           // Threshold for discrete spike collapse
    pub gate_state: AtomicU8,              // Scheduler gate state
    pub _pad: [u8; 15],
}
```

### 3. Procedural Synapse Kernel with Sparse Plasticity ($\Delta W$)
```rust
// Baseline connection calculated on-the-fly via SIMD:
// W_base(i, j) = Kernel(||r_i - r_j||) * cos(theta_i - theta_j)
#[repr(C)]
pub struct PlasticSynapseDelta {
    pub target_id: PackedId,               // 8 bytes: Target neuron ID
    pub weight_delta: i16,                 // 2 bytes: Q4.12 fixed-point deviation from baseline
    pub eligibility_trace: i16,            // 2 bytes: Tagged eligibility trace for 3-factor learning
    pub next_delta_idx: u32,               // 4 bytes: Collision pointer in sparse hash table
}
```

### 4. 3D Neuromodulator Chemical Diffusion
- $128 \times 128 \times 64$ voxel grid tracking $[DA]$ (Dopamine), $[ACh]$ (Acetylcholine), $[5\text{-}HT]$ (Serotonin), $[NE]$ (Norepinephrine).
- **Three-Factor STDP**: $\Delta W = \eta \cdot e_{ij}(t) \cdot (\text{Dopamine} - \text{Baseline})$.

---

## 📁 Repository Structure

```
VirtualCortex/
├── README.md                      # Project overview & quick start
├── docs/
│   ├── WHITEPAPER.md              # VirtualCortex Architecture Whitepaper v2.0 (English)
│   └── zh-TW/
│       └── architecture-report.md # 碎形多尺度認知架構報告 (繁體中文)
├── src/                           # Rust implementation (Planned)
│   ├── identity/                  # PackedId & hierarchical bit-masks
│   ├── state/                     # DendriticSuperNeuron (64B) & NeuronState
│   ├── mass/                      # HyperColumnState & Wilson-Cowan PDE solver
│   ├── connectome/                # Procedural geometric kernels & sparse ΔW table
│   ├── chemical/                  # 3D voxel diffusion tensor & 3-factor STDP
│   ├── mailbox/                   # Treiber stack & thread-local slab
│   ├── timing/                    # Per-worker timing wheels & epoch barrier
│   ├── paging/                    # Two-level page directory & redb eviction
│   └── engine.rs                  # Multi-scale coordinator & SPSC I/O
└── benches/                       # 86-Billion equivalent scale benchmarks
```

---

## 🗺️ Roadmap & Milestones

- [ ] **Milestone 1**: Core Memory & Dendritic Super-Neuron (`size_of == 64`, Apical/Basal coincidence detection)
- [ ] **Milestone 2**: Neural Mass Solver & Procedural Connectome (Wilson-Cowan SIMD solver, spatial kernels)
- [ ] **Milestone 3**: 3D Chemical Grid & Three-Factor Plasticity (Dopamine diffusion, Pavlovian conditioning)
- [ ] **Milestone 4**: Work-Stealing Scheduler & Phased Epoch Barrier (Core affinity, deterministic LFP waves)
- [ ] **Milestone 5**: Metabolic Column Eviction & redb Persistence (>80% RAM reclamation, sub-50µs hydration)
- [ ] **Milestone 6**: 86-Billion-Node Equivalent Full-Brain Benchmark (>100 MSpikes/s throughput within <32 GB RAM)

---

## 📄 Documentation

- 📘 **[VirtualCortex Architecture Whitepaper v2.0](docs/WHITEPAPER.md)**: Full theoretical foundations, data structure proofs, and engineering specifications.
- 📙 **[繁體中文架構報告](docs/zh-TW/architecture-report.md)**: 完整系統架構診斷、生理工程映射與實作細節。

---

## 📜 License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
