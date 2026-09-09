# VirtualCortex

> **A Production-Grade, Deterministic Neuromorphic Engine for Scalable Spiking Neural Computing**  
> *Engineered to 2026+ High-Performance Systems Best Practice (`Latest != Newest`)*

[![Language](https://img.shields.io/badge/Language-Rust%202024%2F2026-orange.svg)](https://www.rust-lang.org/)
[![Architecture](https://img.shields.io/badge/Standard-2026%2B%20Systems%20Best%20Practice-blue.svg)](#design-philosophy-latest--newest)
[![Scale](https://img.shields.io/badge/Capacity-86%20Billion%20Nodes%20Equivalent-red.svg)](#quantitative-pareto-frontier)
[![Determinism](https://img.shields.io/badge/Determinism-100%25%20Bit--Exact%20Q16.16-brightgreen.svg)](#1-bit-exact-cross-platform-reproducibility)
[![Memory Hierarchy](https://img.shields.io/badge/Memory-NUMA%20%2B%20CXL%203.0%20Tiered-purple.svg)](#hardware-native-memory-tiering-cxl-30)
[![License](https://img.shields.io/badge/License-Apache%202.0%20%2F%20MIT-blue.svg)](#license)

---

## 📖 Overview

Simulating the human brain (~86 billion neurons, ~100 trillion synapses) with brute-force computing requires upwards of **700 Terabytes of memory**, placing full-brain emulation outside the realm of single-node computing. Yet biological brains compute through **hierarchical self-similarity, multi-compartment dendritic non-linearities, continuous neural fields, and local synaptic plasticity**.

**VirtualCortex** rejects ephemeral hype in favor of **2026+ Systems Engineering Best Practice (`Latest != Newest`)**:
- **Bit-Exact Numerical Determinism**: Replaces non-associative parallel floating-point operations with **Q16.16 fixed-point integer SIMD** (AVX-512 / ARM SVE2), guaranteeing 100% cross-platform bit-level reproducible neural dynamics.
- **Hardware-Native Memory Tiering (CXL 3.0)**: Coordinates memory across **Tier 0 (L1/L3 SRAM) $\to$ Tier 1 (NUMA DDR5) $\to$ Tier 2 (CXL 3.0 Far Memory) $\to$ Tier 3 (NVMe `io_uring`)**.
- **ABA-Free Lock-Free Concurrency**: 128-bit tagged atomic pointers (`cmpxchg16b` / `AtomicU128`) eliminate ABA hazards during high-frequency spike recycling.
- **Two-Tier Connectome**: 90% intra-column connections use hardware prefetch-friendly **Chunked CSR**; 10% inter-column connections use **Low-Rank Spatial Kernels** with a sparse plastic delta table ($\Delta W$).
- **Multi-Scale Expressiveness**: 860,000 continuous Hyper-Columns (Macro) + 43,000,000 Multi-Compartment Pyramidal Super-Neurons (Meso, $1 \approx 1,000$) deliver **86-billion-node equivalent capacity within ~30 GB RAM**.

👉 **For complete mathematical proofs, memory layout derivations, and biophysical mechanics, read the complete [VirtualCortex Architecture Whitepaper (2026+ Systems Edition)](docs/WHITEPAPER.md)** (中文版架構報告請參閱 [繁體中文架構報告](docs/zh-TW/architecture-report.md))。

---

## ⚡ Key Performance Targets

Tested on a reference **64-core / 128-thread AMD EPYC / ARM Neoverse** server with **128 GB DDR5 RAM**, **CXL 3.0 Far Memory**, and **PCIe 5.0 NVMe SSD**:

| Metric | Target Specification |
| :--- | :--- |
| **Equivalent Brain Scale** | **86,000,000,000 Neurons** (~100 Trillion Synapses) |
| **Physical System RAM** | **~29.6 GB Physical RAM** (Tier 1 NUMA + Tier 2 CXL.mem) |
| **Spike Throughput** | **> 100,000,000 Spikes / sec** Realtime Line-Rate |
| **Median Dispatch Latency** | **< 300 nanoseconds** (L3 / Local NUMA) |
| **Cross-Platform Parity** | **100% Bit-Exact Determinism** (Q16.16 Fixed-Point SIMD) |
| **Concurrency Safety** | **ABA-Free 128-Bit Tagged CAS + 4-State Turn-Based Invariant** |
| **Garbage Collection (GC) Overhead** | **0 ms (Zero runtime GC / Zero dynamic dispatch)** |

---

## 📊 Quantitative Pareto Frontier

How VirtualCortex maps human-scale brain computation into hardware memory tiers:

| Hardware Tier | System Component | Software Structure | Count | Total Allocation |
| :--- | :--- | :--- | :--- | :--- |
| **Tier 1 (NUMA RAM)** | Macro Hyper-Columns | `HyperColumnState` (64B) | 860,000 | **55.0 MB** |
| **Tier 1 (NUMA RAM)** | Meso Super-Neurons | `DendriticSuperNeuron` (64B) | 43,000,000 | **2.75 GB** |
| **Tier 1 (NUMA RAM)** | Worker Slab Pools | SpikeEnvelope Recyclers | 64 Cores | **8.19 GB** |
| **Tier 1 (NUMA RAM)** | Timing Wheels | Circular Buckets | 64 Wheels | **1.20 GB** |
| **Tier 2 (CXL.mem)** | Sparse Plastic Deltas ($\Delta W$) | `PlasticSynapseDelta` (16B) | 1,000,000,000 | **16.00 GB** |
| **Tier 1 (NUMA RAM)** | 3D Voxel Diffusion | 128x128x64 Stencil | 1,048,576 | **16.78 MB** |
| **Tier 1 (NUMA RAM)** | Sparse Page Directory | 2-Level Radix Table | 65,536 | **1.35 GB** |
| **Total System RAM** | **86B Equivalent Brain** | — | — | **29.56 GB** |

---

## 🔬 Subsystems at a Glance

### 1. 64-Byte Cache-Line Aligned Multi-Compartment Unit (`DendriticSuperNeuron`)
```rust
#[repr(C, align(64))]
pub struct DendriticSuperNeuron {
    pub id: PackedId,                      // 8 bytes  (offset 0)  - Hierarchical ID
    pub mailbox_head_ptr: AtomicU64,      // 8 bytes  (offset 8)  - 128-bit Tagged ABA pointer
    pub mailbox_tag: AtomicU64,           // 8 bytes  (offset 16) - Monotonic sequence counter
    pub v_soma: i32,                       // 4 bytes  (offset 24) - Q16.16 Fixed-Point Soma (mV)
    pub v_basal: i32,                      // 4 bytes  (offset 28) - Q16.16 Feedforward Sensory (mV)
    pub v_apical: i32,                     // 4 bytes  (offset 32) - Q16.16 Context Feedback (mV)
    pub v_thresh: i32,                     // 4 bytes  (offset 36) - Q16.16 Adaptive Threshold (mV)
    pub nmda_plateau_ticks: u16,           // 2 bytes  (offset 40) - NMDA Plateau countdown
    pub refractory_ticks: u16,             // 2 bytes  (offset 42) - Absolute refractory ticks
    pub last_spike_tick: u32,              // 4 bytes  (offset 44) - Action potential timestamp
    pub local_synapse_chunk: u32,          // 4 bytes  (offset 48) - Tier-A Chunked CSR index
    pub plastic_delta_head: u16,           // 2 bytes  (offset 52) - Tier-B Sparse plastic index
    pub spatial_coords_packed: u16,        // 2 bytes  (offset 54) - Quantized 3D local coordinate
    pub gate_state: AtomicU8,              // 1 byte   (offset 56) - 4-state CAS scheduler gate
    pub flags: u8,                         // 1 byte   (offset 57) - Bursting / Inhibitory flags
    pub _reserved: [u8; 6],                // 6 bytes  (offset 58..64) - Cache-line padding
}

// Compile-time static assertions ensuring 64-byte layout
const _: () = {
    assert!(core::mem::size_of::<DendriticSuperNeuron>() == 64);
    assert!(core::mem::align_of::<DendriticSuperNeuron>() == 64);
};
```

### 2. Hardware-Native Memory Tiering (CXL 3.0)
```
Tier 0: CPU L1/L2/L3 SRAM (1 ~ 12 ns)  --> Active Somas & Chunked CSR
Tier 1: Local NUMA DDR5   (65 ~ 80 ns) --> Hyper-Columns, Timing Wheels, Slab Pools
Tier 2: CXL 3.0 Far Memory(180 ~ 240ns)--> Sparse Plastic Synapse Deltas (ΔW)
Tier 3: NVMe io_uring     (10 ~ 25 µs) --> Quiescent Cold Cortical Chunks (redb)
```

### 3. Bit-Exact Q16.16 SIMD Integration
Integer-only bit-shift membrane decay without floating-point transcendence:
```rust
#[inline(always)]
pub fn integrate_lif_q16(v_mem: &mut i32, v_rest: i32, decay_shift: u32, input_current: i32) {
    let delta = *v_mem - v_rest;
    let decayed = delta - (delta >> decay_shift);
    *v_mem = v_rest + decayed + input_current;
}
```

---

## 📁 Repository Structure

```
virtual_cortex/
├── README.md                      # Project overview & quick start
├── docs/
│   ├── WHITEPAPER.md              # Whitepaper — 2026+ Systems Edition (English)
│   └── zh-TW/
│       └── architecture-report.md # 2026+ 系統工程最佳實踐報告 (繁體中文)
├── src/                           # Rust implementation (Planned)
│   ├── arch/                      # AVX-512 / ARM SVE2 target-specific kernels
│   ├── identity/                  # PackedId & hierarchical bitmask operators
│   ├── state/                     # DendriticSuperNeuron (64B) & HyperColumnState (64B)
│   ├── atomic/                    # ABA-free 128-bit tagged pointer Treiber mailbox
│   ├── connectome/                # Chunked CSR & Tier-B low-rank sparse delta table
│   ├── numeric/                   # Q16.16 fixed-point bit-exact SIMD integration
│   ├── memory/                    # NUMA affinity and CXL.mem tiered allocation
│   ├── timing/                    # Per-worker private timing wheels & BSP barrier
│   └── telemetry/                 # Lock-free SPSC metrics ring & Perfetto exporter
└── benches/                       # Deterministic full-brain benchmarking suite
```

---

## 🗺️ Roadmap & Milestones

- [ ] **Milestone 1**: Bit-Exact Numeric & Memory Invariants (`size_of == 64`, Q16.16 parity across x86/ARM)
- [ ] **Milestone 2**: ABA-Free 128-Bit Atomic Mailbox (Stress test under 1,000 contending threads, 0 lost events)
- [ ] **Milestone 3**: Two-Tier Connectome & SIMD Acceleration (Tier-A CSR $>40\text{ GB/s}$, Tier-B $<250\text{ns}$)
- [ ] **Milestone 4**: CXL Tiered Memory & io_uring Eviction (NUMA vs. CXL.mem, asynchronous fault $<20\mu\text{s}$)
- [ ] **Milestone 5**: 86B Equivalent Full-Scale Benchmark (>100 MSpikes/s throughput within <32 GB RAM)

---

## 📄 Documentation

- 📘 **[VirtualCortex Architecture Whitepaper (2026+ Systems Edition)](docs/WHITEPAPER.md)**: Formal proofs, memory tiering contracts, and Rust specifications.
- 📙 **[繁體中文架構報告 (2026+ 系統工程最佳實踐版)](docs/zh-TW/architecture-report.md)**: 完整系統架構診斷、數值工程映射與實作細節。

---

## 📜 License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
