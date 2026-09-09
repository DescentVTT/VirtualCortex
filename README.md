# VirtualCortex

> **A Single-Node Neuromorphic Virtual Actor Engine for Ultra-Dense Spiking Neural Networks**

[![Language](https://img.shields.io/badge/Language-Rust%202021-orange.svg)](https://www.rust-lang.org/)
[![Architecture](https://img.shields.io/badge/Architecture-Virtual%20Actor%20%2B%20SNN-blue.svg)](#core-architectural-axioms)
[![Performance](https://img.shields.io/badge/Target->100%20MSpikes%2Fsec-brightgreen.svg)](#performance-targets)
[![License](https://img.shields.io/badge/License-Apache%202.0%20%2F%20MIT-blue.svg)](#license)

---

## 📖 Overview

Modern deep learning relies on dense, synchronous matrix multiplication (GEMM) across GPU clusters. In contrast, biological brains compute through **extreme spatio-temporal sparsity (~1–2% instantaneous activation), asynchronous event-driven spike dynamics, and local synaptic plasticity**.

**VirtualCortex** brings the **Virtual Actor model** (originating from distributed cloud systems like Microsoft Orleans) to computational neuroscience's **Spiking Neural Networks (SNNs)**, strictly bounded to a **single high-core NUMA server**.

By eliminating network latency, RPC serialization, and distributed consensus protocols, VirtualCortex optimizes directly for:
- **64-byte CPU Cache Lines** (Zero false sharing, POD layout).
- **Intrusive Lock-Free Mailboxes** (8-byte Treiber Stack, single-atomic batch draining).
- **Partitioned Axonal Timing Wheels** (Thread-local circular delay queues).
- **Double-Buffered Phased Epoch Barriers** (Deterministic discrete temporal progression).
- **Metabolic Paging** (Minicolumn-level clock sweep into embedded zero-copy `redb` storage).

👉 **For full mathematical proofs, memory layout breakdowns, and biophysical mechanics, read the complete [VirtualCortex Architecture Whitepaper](docs/WHITEPAPER.md)** (中文版架構報告請參閱 [繁體中文架構報告](docs/zh-TW/architecture-report.md))。

---

## ⚡ Key Performance Targets

Tested on a reference **64-core / 128-thread AMD EPYC / ARM Neoverse** server with **128 GB DDR5 RAM** and **PCIe 5.0 NVMe SSD**:

| Metric | Target Specification |
| :--- | :--- |
| **Spike Ingestion & Dispatch** | **> 100,000,000 Spikes / sec** |
| **Median Event Dispatch Latency** | **< 300 nanoseconds** |
| **Active In-Memory Neurons** | **10,000,000+** concurrent soma states |
| **Dormant Virtual Capacity** | **100,000,000+** virtual units via metabolic paging |
| **Garbage Collection (GC) Overhead** | **0 ms (Zero runtime GC / Zero dynamic dispatch)** |
| **Turn-Based Safety** | **Strict single-thread invariant per soma (Lock-free 4-state CAS)** |

---

## 🧠 Core Architectural Axioms

```
                    ┌──────────────────────────────┐
                    │ 1. Virtual Existence         │
                    │ Eternal logic, demand paging │
                    └──────────────┬───────────────┘
                                   ▼
┌──────────────────────────────┐       ┌──────────────────────────────┐
│ 2. State-Worker Decoupling   │ <───> │ 3. Turn-Based Invariant      │
│ Soma = Data, Worker = Compute│       │ 4-State CAS, zero data races │
└──────────────┬───────────────┘       └──────────────┬───────────────┘
               ▼                                      ▼
┌──────────────────────────────┐       ┌──────────────────────────────┐
│ 4. Soma-Synapse Decoupling   │ <───> │ 5. Discrete Axonal Wheels    │
│ 64B Soma + Chunked CSR Fabric│       │ Thread-local circular delay  │
└──────────────┬───────────────┘       └──────────────┬───────────────┘
               ▼                                      ▼
┌──────────────────────────────┐       ┌──────────────────────────────┐
│ 6. Phased Epoch Determinism  │ <───> │ 7. Metabolic Paging          │
│ Double-buffered BSP barrier  │       │ Column-level storage tiering │
└──────────────────────────────┘       └──────────────────────────────┘
```

1. **Virtual Existence**: Neurons logically exist permanently as 64-bit `PackedId`s. Physical memory is allocated on-demand upon incoming spike impact.
2. **State-Worker Decoupling**: Neurons are passive 64-byte POD data structures; workers are roving, stateless threads pinned to CPU cores.
3. **Turn-Based Single-Thread Invariant**: A 4-state atomic CAS machine (`IDLE -> QUEUED -> RUNNING -> RECHECK`) guarantees zero race conditions and zero deadlocks without mutexes.
4. **Soma-Synapse Decoupling**: Soma states (`NeuronState`) are strictly 64 bytes (1 cache line). Downstream synaptic connections are maintained in a separate chunked Compressed Sparse Row (CSR) arena.
5. **Discrete Axonal Timing Wheels**: Axonal conduction delays are quantized into discrete ticks and queued in thread-local circular ring buffers ($O(1)$ delay dispatch).
6. **Phased Epoch Determinism**: Three-phase execution barrier (`Delivery -> Parallel Compute -> Advance`) prevents causality violations while enabling lock-free work-stealing.

---

## 🔬 Subsystems at a Glance

### 1. 64-byte Cache-Line Aligned Soma (`NeuronState`)
```rust
#[repr(C, align(64))]
pub struct NeuronState {
    pub id: PackedId,                      // 8 bytes (offset 0)
    pub mailbox_head: AtomicPtr<SpikeNode>,// 8 bytes (offset 8)
    pub v_mem: f32,                        // 4 bytes (offset 16)
    pub v_thresh: f32,                     // 4 bytes (offset 20)
    pub v_reset: f32,                      // 4 bytes (offset 24)
    pub post_trace: f32,                   // 4 bytes (offset 28)
    pub last_spike_tick: u32,              // 4 bytes (offset 32)
    pub last_active_tick: u32,             // 4 bytes (offset 36)
    pub synapse_chunk_id: u32,             // 4 bytes (offset 40)
    pub synapse_count: u32,                // 4 bytes (offset 44)
    pub decay_lut_idx: u16,                // 2 bytes (offset 48)
    pub refractory_ticks: u16,             // 2 bytes (offset 50)
    pub gate_state: AtomicU8,              // 1 byte  (offset 52)
    pub flags: u8,                         // 1 byte  (offset 53)
    pub _reserved: [u8; 10],               // 10 bytes (offset 54..64)
}
```

### 2. Intrusive Single-Atomic Mailbox
- Idle memory footprint: **8 bytes** (`AtomicPtr<SpikeNode>`).
- Enqueue: Lock-free Treiber stack prepend via atomic CAS.
- Batch Drain: Single atomic `swap(null)` decouples the entire accumulated spike list directly into registers.

### 3. LUT-Accelerated Leaky Integrate-and-Fire (LIF)
Exponential membrane potential decay is precomputed into a 1 KB L1-cached Look-Up Table:
```rust
let elapsed = (current_tick - neuron.last_spike_tick).min(255) as usize;
let factor = DECAY_LUT[neuron.decay_lut_idx as usize][elapsed];
neuron.v_mem = neuron.v_reset + (neuron.v_mem - neuron.v_reset) * factor + total_current;
```

### 4. Online Pair-Based STDP
Local unsupervised Hebbian learning using scalar pre/post eligibility traces. Computes in $O(1)$ constant time without maintaining spike history buffers.

---

## 📁 Repository Structure

```
VirtualCortex/
├── README.md                      # Project overview & documentation index
├── docs/
│   ├── WHITEPAPER.md              # Complete Architecture Whitepaper (English)
│   └── zh-TW/
│       └── architecture-report.md # 架構分析與工程規格報告 (繁體中文)
├── src/                           # Rust implementation (Planned)
│   ├── identity/                  # PackedId & hierarchical bit-masks
│   ├── state/                     # NeuronState & Chunked CSR Connectome
│   ├── mailbox/                   # Treiber stack & thread-local slab
│   ├── timing/                    # Per-worker timing wheels & epoch barrier
│   ├── compute/                   # LUT-accelerated LIF & online STDP
│   ├── paging/                    # Two-level page directory & redb eviction
│   └── engine.rs                  # Runtime engine coordinator & SPSC I/O
└── benches/                       # 100M spike benchmarking suite
```

---

## 🗺️ Roadmap & Milestones

- [ ] **Milestone 1**: Core Memory & Gating (64-byte `NeuronState`, 4-state CAS, Treiber Mailbox)
- [ ] **Milestone 2**: Topology & Local Timing Wheels (Chunked CSR Connectome, Thread-local delay wheels)
- [ ] **Milestone 3**: Work-Stealing Scheduler & Phased Epoch Barrier (Core affinity, deterministic oscillator tests)
- [ ] **Milestone 4**: Biophysics & Online STDP (1KB LUT decay, pair-based Hebbian plasticity)
- [ ] **Milestone 5**: Hierarchical Paging & 100M Spike Benchmark (Column eviction to `redb`, >100 MSpikes/s throughput)

---

## 📄 Documentation

- 📘 **[English Architecture Whitepaper](docs/WHITEPAPER.md)**: Full theoretical foundations, data structure proofs, and engineering specifications.
- 📙 **[繁體中文架構報告](docs/zh-TW/architecture-report.md)**: 完整系統架構診斷、生理工程映射與實作細節。

---

## 📜 License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
