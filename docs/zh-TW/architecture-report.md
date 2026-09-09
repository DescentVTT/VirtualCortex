# 單機神經擬態虛擬 Actor 引擎架構報告與工程實作規範

*(Single-Node Neuromorphic Virtual Actor Engine — Codename: VirtualCortex)*

---

## 1. 執行摘要與願景 (Executive Summary)

### 1.1 典範轉移：從同步矩陣相乘到稀疏事件流
傳統深度學習依賴於全域同步的張量乘法（GEMM）與反向傳播（Backpropagation）。這種架構高度適配於 GPU 批次處理，但需要消耗龐大的電能與靜態顯存，且與真實生物大腦的運作哲學背道而馳。

真實大腦具有三大根本性計算特徵：
1. **極度稀疏性（Extreme Sparsity）**：在任意瞬時，僅有約 1% 至 2% 的神經元處於動作電位激發狀態。
2. **非同步事件驅動（Asynchronous Event-Driven）**：無全域時鐘強制所有節點同步，計算僅在脈衝（Spike）抵達時發生。
3. **拓撲決定算力（Topology-Defined Computation）**：智慧蘊含於突觸連接權重、突觸前/後延遲與局部可塑性（Plasticity）之中，而非巨量全連通參數層。

### 1.2 VirtualCortex 核心定位
**VirtualCortex** 是一部專為單一物理伺服器打造的**軟體級神經擬態協同處理器**。它深度融合了雲端分散式系統的 **Virtual Actor 模型**（如 Orleans 的虛擬存在、按需加載）與計算神經科學的 **脈衝神經網路（Spiking Neural Networks, SNN）**。

透過摒棄跨機網路延遲、RPC 序列化、TCP/IP 協定棧與分散式共識演算法的巨大負擔，VirtualCortex 將現代多核心 CPU 的硬體潛能發揮至極致——包含 **CPU 快取行（64-byte Cache Line）、無鎖原子指令（CAS/AcqRel）、工作竊取執行緒調度（Work-Stealing）以及 NUMA 記憶體親和性**。

```
┌──────────────────────────────────────────────────────────────────────────────────┐
│                            VirtualCortex 設計指標                                │
├─────────────────────────┬─────────────────────────┬──────────────────────────────┤
│ 硬體基準規格            │ 目標吞吐量              │ 系統容量與能效               │
│ • 單台 64 核心 / 128 緒 │ • > 100 MSpikes / sec   │ • 10,000,000+ 活躍神經元     │
│ • 128 GB DDR5 RAM       │ • 中位數派發延遲 < 300ns│ • 100,000,000+ 休眠虛擬單元  │
│ • PCIe 5.0 NVMe SSD     │ • 微秒級時序決定性      │ • 零動態 GC，記憶體常駐受控  │
└─────────────────────────┴─────────────────────────┴──────────────────────────────┘
```

**技術棧約束 (Tech Stack Constraint)**：全系統基於純 **Rust** 實現。底層採用無鎖資料結構、侵入式指針與嵌入式零拷貝儲存（`redb` / 記憶體映射分頁）。全系統**嚴格禁止垃圾回收（GC）、非同步運行時堆疊濫用（如無節制的 async/await 協程分配）與多層動態分派（trait object dynamic dispatch）**。

---

## 2. 六大架構公理 (Architectural Axioms)

整套引擎的設計完全建立在以下六項不可妥協的底層公理之上：

```
                    ┌──────────────────────────────┐
                    │ 1. 虛擬存在 (Virtual Being)   │
                    │ 邏輯永存，物理按需加載         │
                    └──────────────┬───────────────┘
                                   ▼
┌──────────────────────────────┐       ┌──────────────────────────────┐
│ 2. 狀態與算力解耦 (Decoupling)│ <───> │ 3. 回合制單執行緒封閉 (Turn) │
│ 胞體是靜態數據，執行緒是巡迴算力│       │ 原子門控，零競態與零死鎖     │
└──────────────┬───────────────┘       └──────────────┬───────────────┘
               ▼                                      ▼
┌──────────────────────────────┐       ┌──────────────────────────────┐
│ 4. 胞體與突觸分離 (CSR Fabric)│ <───> │ 5. 離散時空輪盤 (Delay Wheel)│
│ 64B 胞體 + 分塊拓撲陣列      │       │ 軸突傳導延遲轉化為 O(1) 槽位 │
└──────────────┬───────────────┘       └──────────────┬───────────────┘
               ▼                                      ▼
┌──────────────────────────────┐       ┌──────────────────────────────┐
│ 6. 分相步進與時脈決定性 (BSP) │ <───> │ 7. 代謝式冷熱分層 (Eviction) │
│ 雙緩衝無鎖推進，消除時空因果悖論│       │ 像大腦一樣動態遺忘與固化     │
└──────────────────────────────┘       └──────────────────────────────┘
```

### 公理 1：虛擬存在（Virtual Existence）
神經單元在邏輯上永遠存在，但在物理記憶體中按需加載。未被啟動的單元僅是一個 64-bit 數字編號（`PackedId`）；只有當脈衝信號命中該編號時，引擎才會透過分頁目錄將其映射至 RAM 或從持久化儲存中惰性加載（Lazy Hydration）。長期未活動的單元將自動休眠卸載。

### 公理 2：狀態與算力解耦（State-Worker Decoupling）
神經單元不是常駐的作業系統執行緒或高開銷協程。Actor 是「緊湊的靜態數據結構體（POD）」，Worker 是「無狀態的巡迴算力」。數千萬個神經單元共享與實體 CPU 核心數嚴格對齊的少數 Worker 執行緒。系統資源消耗僅與**當前瞬間活躍的神經元數量**成正比，與系統總配置容量無關。

### 公理 3：回合制單執行緒封閉（Turn-Based Invariant）
任何神經單元在任意給定微秒內，最多只能被一個 Worker 執行緒存取。透過四態原子比較並交換（CAS）機制，保證單元內部的膜電位演化、突觸塑性計算永遠是單向、循序且無資料競爭的，完全消滅全域互斥鎖（Mutex）與死鎖風險。

### 公理 4：胞體與突觸解耦佈局（Soma-Synapse Decoupling Fabric）
嚴格將神經元的「胞體狀態（Soma State）」與「下游突觸名單（Axon & Synapses）」在記憶體空間中解耦。胞體維持嚴格的 64-byte 快取行對齊 POD 結構；突觸名單則採用連續分塊壓縮稀疏陣列（Chunked CSR）集中管理，徹底解決高分支度（Fan-out）與快取行大小限制的物理衝突。

### 公理 5：離散時空輪盤（Discrete Axonal Delay）
生物神經訊號的軸突傳導並非即時抵達，而是存在 1 至數十毫秒的傳導延遲。引擎嚴格禁止使用作業系統定時器（OS Timer）或非同步等待（`thread::sleep`），而是將時間離散化為滴答（Ticks，如 $\Delta t = 0.5\,\text{ms}$），藉由各 Worker 私有的環形時間輪（Partitioned Timing Wheel）將複雜的時間延遲降維為常數時間複雜度 $O(1)$ 的記憶體槽位定址。

### 公理 6：分相時鐘決定性（Phased Epoch Determinism）
為避免多執行緒工作竊取引發的「時序倒流」與「因果衝突」（如 Tick $T+1$ 的事件在 Tick $T$ 被提前消費），系統採用雙緩衝分相步進協議（Phased Epoch Barrier）。脈衝的生成、派發與神經元積分演化嚴格在分相屏障內完成，兼具超高並發吞吐與時序精準度。

---

## 3. 核心子系統架構 (Core Subsystems)

VirtualCortex 劃分為五個高內聚、零阻抗的子系統：

```
┌────────────────────────────────────────────────────────────────────────────────────────┐
│ 1. 識別碼與記憶體排布 (Identity & State Fabric)                                        │
│    • 64-bit Packed ID 階層位元定址 (Region:16 | Column:16 | Neuron:32)                 │
│    • 64-byte Cache-line 對齊 POD 胞體結構 (NeuronState)                                 │
│    • 分塊壓縮稀疏拓撲陣列 (Chunked CSR Synapse Fabric)                                 │
└───────────────────────────────────────────┬────────────────────────────────────────────┘
                                            │
┌───────────────────────────────────────────▼────────────────────────────────────────────┐
│ 2. 侵入式信箱與四態門控排程器 (Intrusive Mailbox & 4-State Gated Scheduler)            │
│    • 8-byte 侵入式單原子指針 Treiber Stack (零空佇列記憶體浪費)                         │
│    • 原子四態轉換 (IDLE -> QUEUED -> RUNNING -> RECHECK)                               │
│    • 一次原子 SWAP 批次抽乾機制 (Batch Draining)                                       │
│    • 本地雙端工作竊取佇列 (Chase-Lev Work-Stealing Deque)                              │
└───────────────────────────────────────────┬────────────────────────────────────────────┘
                                            │
┌───────────────────────────────────────────▼────────────────────────────────────────────┐
│ 3. 分散式軸突時間輪與分相時序推進 (Partitioned Timing Wheel & Epoch Barrier)           │
│    • Per-Worker 私有環形槽位 (256/512 Buckets，消除跨核心鎖競爭)                       │
│    • 輕量原子分相屏障 (Phase 1: 倒出派發 -> Phase 2: 竊取計算 -> Phase 3: 時脈步進)    │
└───────────────────────────────────────────┬────────────────────────────────────────────┘
                                            │
┌───────────────────────────────────────────▼────────────────────────────────────────────┐
│ 4. 生物物理計算與在線可塑性引擎 (Biophysical & Plasticity Compute Engine)              │
│    • 查表化 (LUT) 指數衰減洩漏整合放電 (LIF Dynamics)                                   │
│    • 動態動態不反應期計數器與自適應閾值                                                │
│    • O(1) 在線雙跡線突觸塑性學習規則 (Pair-based Trace STDP)                           │
└───────────────────────────────────────────┬────────────────────────────────────────────┘
                                            │
┌───────────────────────────────────────────▼────────────────────────────────────────────┐
│ 5. 階層式分頁目錄與代謝持久化 (Hierarchical Paging & Metabolic Eviction)              │
│    • 兩級 Sparse Page Table (Region -> Column Chunk -> Neuron Array)                   │
│    • 時鐘指針巡檢器 (Clock Sweep Worker) 以皮質柱 (Column) 為單位淘汰                   │
│    • 零拷貝記憶體映射與本地嵌入式儲存 (redb Storage Engine)                            │
└────────────────────────────────────────────────────────────────────────────────────────┘
```

---

### 3.1 識別碼與記憶體排布 (Identity & State Fabric)

#### 3.1.1 64-bit Packed ID 階層定址
摒棄字串與 UUID。`PackedId` 本質為一個 `u64`，直接對齊神經解剖學的空間層級結構：

```
 63          48 47          32 31                                       0
┌──────────────┬──────────────┬──────────────────────────────────────────┐
│  Region ID   │  Column ID   │             Local Neuron ID              │
│   (16 bits)  │   (16 bits)  │                (32 bits)                 │
└──────────────┴──────────────┴──────────────────────────────────────────┘
```
* **Region ID (16 bits, 0~65,535)**：大腦巨觀腦區（如 V1 視皮質、CA1 海馬迴、紋狀體、丘腦網狀核）。
* **Column ID (16 bits, 0~65,535)**：微皮質柱 / 局部微迴路（Cortical Minicolumn）。每個 Column 為記憶體分配、局部側向抑制與硬碟分頁置換的基本單元。
* **Neuron ID (32 bits, 0~4,294,967,295)**：柱內具體神經元編號。

```rust
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PackedId(pub u64);

impl PackedId {
    pub const REGION_SHIFT: u32 = 48;
    pub const COLUMN_SHIFT: u32 = 32;
    pub const NEURON_MASK: u64 = 0x0000_0000_FFFF_FFFF;
    pub const COLUMN_MASK: u64 = 0x0000_FFFF_0000_0000;
    pub const REGION_MASK: u64 = 0xFFFF_0000_0000_0000;

    #[inline(always)]
    pub const fn new(region: u16, column: u16, neuron: u32) -> Self {
        Self(((region as u64) << Self.REGION_SHIFT) 
           | ((column as u64) << Self.COLUMN_SHIFT) 
           | (neuron as u64))
    }

    #[inline(always)]
    pub const fn region(self) -> u16 { (self.0 >> Self.REGION_SHIFT) as u16 }
    #[inline(always)]
    pub const fn column(self) -> u16 { (self.0 >> Self.COLUMN_SHIFT) as u16 }
    #[inline(always)]
    pub const fn neuron(self) -> u32 { (self.0 & Self.NEURON_MASK) as u32 }
}
```

#### 3.1.2 64-byte Cache-Line 對齊胞體結構 (`NeuronState`)
神經元胞體必須精確佔用 **64 Bytes**（對齊現代 x86_64 / ARM64 的 L1/L2 快取行），消除跨快取行存取的效能懲罰，並杜絕多執行緒操作時的 False Sharing。

```rust
use std::sync::atomic::{AtomicPtr, AtomicU8};

#[repr(C, align(64))]
pub struct NeuronState {
    // [0..8] 唯一邏輯識別碼 (8 bytes)
    pub id: PackedId,

    // [8..16] 侵入式信箱鏈表頭指標 (8 bytes, 原子交換)
    pub mailbox_head: AtomicPtr<SpikeNode>,

    // [16..32] 生物物理狀態 (16 bytes)
    pub v_mem: f32,                // 當前膜電位 (mV)
    pub v_thresh: f32,             // 動態發放閾值 (mV)
    pub v_reset: f32,              // 靜息/重置電位 (mV)
    pub post_trace: f32,           // 突觸後在線 STDP 跡線

    // [32..48] 時序與拓撲指標 (16 bytes)
    pub last_spike_tick: u32,      // 上次放電 Tick (供 STDP 與衰減計算)
    pub last_active_tick: u32,     // 最後活躍時間戳 (供 Clock Sweep 淘汰評估)
    pub synapse_chunk_id: u32,     // 突觸拓撲塊的 Arena 索引
    pub synapse_count: u32,        // 下游突觸總數量 (支援高達 40 億條連接)

    // [48..54] 控制欄位與狀態機 (6 bytes)
    pub decay_lut_idx: u16,        // 時間常數 tau 對應的衰減查表索引
    pub refractory_ticks: u16,     // 剩餘不反應期 Ticks
    pub gate_state: AtomicU8,      // 4-State CAS 門控: 0=IDLE, 1=QUEUED, 2=RUNNING, 3=RECHECK
    pub flags: u8,                 // 旗標: bit0=興奮/抑制, bit1=熱/冷, bit2=凍結塑性

    // [54..64] 保留填充，精確補齊 64 位元組 (10 bytes)
    pub _reserved: [u8; 10],
}
```

**記憶體佈局驗證**：
$8 + 8 + 16 + 16 + 6 + 10 = 64$ 位元組。所有 8 位元組變數（`id`, `mailbox_head`）對齊於 8 倍數偏移量；所有 4 位元組變數（`f32`, `u32`）對齊於 4 倍數偏移量；`align(64)` 確保結構體在陣列中排列時，每個實例完美佔據單一 Cache Line。

#### 3.1.3 分塊壓縮突觸拓撲陣列 (Chunked CSR Synapse Fabric)
神經元下游連接名單不在 `NeuronState` 內部，而是存儲於專屬的 **Connectome Arena**。每個突觸壓縮為 8 位元組（64-bit 打包）：

```
 63                            32 31                16 15        8 7        0
┌────────────────────────────────┬────────────────────┬───────────┬──────────┐
│      Target Neuron ID          │   Weight (Fixed)   │   Delay   │  Flags   │
│   (32 bits 柱內 / 相對定址)     │  (16-bit Q4.12)    │ (8 bits)  │ (8 bits) │
└────────────────────────────────┴────────────────────┴───────────┴──────────┘
```

* **Target ID (32 bits)**：指向目標神經元 ID。
* **Weight (16 bits, Q4.12 固定點)**：範圍 $[-8.0, +7.9997]$，步長 $\approx 0.00024$。正值代表興奮性（AMPA/NMDA-like），負值代表抑制性（GABA-like）。
* **Delay (8 bits, 0~255 Ticks)**：對應軸突傳導延遲（在 $\Delta t=0.5\,\text{ms}$ 下支援 $0 \sim 127.5\,\text{ms}$）。
* **Flags (8 bits)**：突觸類型（可塑性開關、調製通道標記）。

**拓撲塊管理（Chunking）**：突觸以 64 個突觸為一個 Chunk（$64 \times 8 = 512\,\text{Bytes}$，剛好 8 個快取行）。若神經元擁有 100 條下游連接，則分配 2 個連續 Chunk，讀取時發揮極致的 CPU 硬體預取（Hardware Stream Prefetching）效能。

---

### 3.2 侵入式信箱與四態門控排程器 (Mailbox & Gated Scheduler)

#### 3.2.1 侵入式單原子指針信箱 (Intrusive Treiber Stack)
為避免數千萬個活躍/休眠 Actor 分配空佇列物件而耗盡數十 GB 記憶體，本引擎採用 **侵入式單原子指針（Single Atomic Pointer）** 信箱架構。

每個傳遞中的脈衝信號即為鏈表節點 `SpikeNode`：
```rust
#[repr(C)]
pub struct SpikeNode {
    pub next: *mut SpikeNode,   // 8 bytes: 侵入式單向指針
    pub source_id: PackedId,    // 8 bytes: 發射端 ID
    pub weight: f32,            // 4 bytes: 突觸強度
    pub arrival_tick: u32,      // 4 bytes: 抵達滴答
}
```

* **發送端投遞（Enqueue）**：
  從 Worker 執行緒私有的物件池（Thread-Local Slab Arena）分配節點，並對目標 Actor 的 `mailbox_head` 進行原子操作：
  ```rust
  // 無鎖 Treiber Stack 壓入
  let mut current = target_neuron.mailbox_head.load(Ordering::Relaxed);
  loop {
      spike_node.next = current;
      match target_neuron.mailbox_head.compare_exchange_weak(
          current, spike_node, Ordering::Release, Ordering::Relaxed
      ) {
          Ok(_) => break,
          Err(actual) => current = actual,
      }
  }
  ```
* **記憶體開銷**：每個 Actor 的信箱僅佔用胞體內的 **8 個位元組**（`mailbox_head`），閒置節點記憶體開銷為零！

#### 3.2.2 四態原子門控狀態機 (4-State CAS Gate)
為保證公理 3 的「回合制單執行緒封閉」，神經元狀態機定義如下：

```
       [IDLE (0)] 
           │
           │ 上游脈衝抵達 (CAS 0 -> 1)
           ▼
      [QUEUED (1)] ── 推入 Worker 工作竊取佇列
           │
           │ Worker 取出並認領 (CAS 1 -> 2)
           ▼
     [RUNNING (2)] ── 一次 SWAP 抽乾信箱並執行生物物理積分
           │
           ├─── 信箱仍為空 (CAS 2 -> 0) ───> 返回 [IDLE (0)]
           │
           └─── 執行期間有新脈衝湧入 (CAS 2 -> 3) ──> [RECHECK (3)]
                                                         │
                                                         └──> 立即原地重抽，重回 [RUNNING (2)]
```

```rust
pub const GATE_IDLE: u8 = 0;
pub const GATE_QUEUED: u8 = 1;
pub const GATE_RUNNING: u8 = 2;
pub const GATE_RECHECK: u8 = 3;
```

#### 3.2.3 批次抽乾機制 (Batch Draining & Memory Reuse)
當 Worker 執行緒成功將門控置為 `GATE_RUNNING` 後，執行一次單原子 `swap`：
```rust
// 一次原子操作，瞬間剝離整個累積脈衝鏈表！
let head: *mut SpikeNode = neuron.mailbox_head.swap(std::ptr::null_mut(), Ordering::AcqRel);
```
Worker 獲取整條單向鏈表，在 CPU 暫存器中一口氣展開累加所有傳入電位。處理完畢後，節點批次歸還至 Worker 本地 Slab，全程**無全域鎖競爭、無系統呼叫、無 malloc/free 開銷**。

---

### 3.3 離散軸突時間輪與時序推進 (Timing Wheel & Epoch Barrier)

#### 3.3.1 Per-Worker 私有分區時間輪 (Thread-Local Timing Wheel)
全域共用時間輪在 64 核環境下會引發嚴重的快取行跳躍（Cache Bouncing）。VirtualCortex 為每個 Worker 執行緒配置一個私有時間輪：

```
Worker 0 Timing Wheel (Thread-Local)
[Slot 0] [Slot 1] ... [Slot 255] (環形陣列，容量 256 Ticks)
   │
   └── 指向待發送的 Spike 連接列表 (延遲到期即刻投遞)
```

* 當 Worker $W_i$ 處理神經元放電時，查得下游連線延遲為 $D$ Ticks。
* $W_i$ 直接將延遲脈衝壓入 $W_i$ 自身的私有槽位：
  $$\text{Target\_Slot} = (\text{Current\_Tick} + D) \pmod{256}$$
* **完全無跨執行緒鎖爭奪**。

#### 3.3.2 雙緩衝分相步進協議 (Phased Epoch Barrier Protocol)
為確保全系統在離散時鐘推進時具備嚴格的時序決定性，系統在 Tick 推進時遵循三相步進（BSP-like Epoch）：

```
═════════════════════════════════════════════════════════════════════════════════════
時脈滴答 T (Tick T)
─────────────────────────────────────────────────────────────────────────────────
【Phase 1: 軸突事件倒出 (Axonal Delivery Phase)】
  • 每個 Worker 遍歷本地時間輪槽位 (Tick T % 256)。
  • 將到期的 Spike 投遞至目標神經元的侵入式信箱。
  • 若觸發 IDLE -> QUEUED 轉換，將 Actor 指標推入 Worker 本地 Work-Stealing 佇列。
  ▼
【Phase 2: 平行積分與放電 (Parallel Compute Phase)】
  • 64 個 Worker 全速並行處理佇列中的 Actor。
  • 若本地佇列耗盡，從其他 Worker 佇列進行無鎖工作竊取（Work-Stealing）。
  • 神經元積分、放電、計算 STDP，並將產生之延遲 Spike 存入各 Worker 的時間輪中。
  ▼
【Phase 3: 輕量同步與步進 (Tick Barrier & Advance)】
  • 當所有 Worker 佇列均為空且所有 Actor 處於非 RUNNING 態：
  • 執行緒池發起輕量原子計數屏障 (`fetch_add` / Sense-reversing barrier)。
  • 全域時鐘推進：Current_Tick += 1，進入 Tick T+1。
═════════════════════════════════════════════════════════════════════════════════════
```

---

### 3.4 生物物理機制的工程映射 (Biophysical SNN Engine)

#### 3.4.1 查表化洩漏整合放電 (LIF Dynamics with Precomputed LUT)
生物膜電位連續微分方程為：
$$\tau_m \frac{dV(t)}{dt} = -(V(t) - V_{\text{rest}}) + R \cdot I(t)$$

在離散時間步長 $\Delta t$ 下，其解析解表現為指數衰減：
$$V(t + \Delta t) = V_{\text{rest}} + (V(t) - V_{\text{rest}}) \cdot e^{-\Delta t / \tau_m} + \sum W_{\text{in}}$$

**工程優化策略**：
在熱路徑中進行浮點 `exp()` 計算極其昂貴。我們預先建立靜態查找表（Look-Up Table, LUT）：
$$\text{LUT}[\Delta \text{ticks}] = e^{-\Delta \text{ticks} \cdot \Delta t / \tau_m}$$
對於 256 項的微型快取陣列（僅佔用 1 KB，恆駐 L1 Data Cache），膜電位衰減直接降維為一次陣列索引與單次乘加運算（FMA）：

```rust
// 零超越函數的微秒級膜電位更新
let elapsed_ticks = (current_tick - neuron.last_spike_tick).min(255) as usize;
let decay_factor = DECAY_LUT[neuron.decay_lut_idx as usize][elapsed_ticks];

// 膜電位衰減並融合批次輸入突觸電流
neuron.v_mem = neuron.v_reset + (neuron.v_mem - neuron.v_reset) * decay_factor + total_input_current;
```

#### 3.4.2 動態不反應期與發放閾值
* **絕對不反應期（Refractory Countdown）**：
  若 `refractory_ticks > 0`，神經元處於生物不應期。外部傳入脈衝僅被直接忽略或低權重吸收，`v_mem` 強制鎖定在 `v_reset`，防止異常高頻發放引發突觸風暴。
* **自適應發放閾值（Adaptive Threshold）**：
  放電後，閾值瞬時提升 $V_{\text{thresh}} \leftarrow V_{\text{thresh}} + \beta$，隨時間逐步衰減回基準閾值，賦予虛擬神經元生物學中的頻率適應（Spike-Frequency Adaptation）特性。

#### 3.4.3 $O(1)$ 在線雙跡線 STDP 學習規則 (Pair-based Trace STDP)
傳統 STDP 需回溯神經元的所有歷史脈衝，開銷為 $O(N)$ 且違反 64-byte 記憶體限制。VirtualCortex 實現嚴格的**在線跡線模型（Online Trace-based STDP）**：

```
突觸前脈衝 (Pre-Spike) 抵達時：
  1. 突觸後跡線 y(t) 隨時間衰減：y(t) = y · e^(-Δt / τ_-)
  2. 觸發長時程抑制 (LTD)：W = W - A_- · y(t)
  3. 突觸前跡線 x(t) 瞬時遞增：x(t) = x(t) + 1

突觸後神經元放電 (Post-Fire) 時：
  1. 突觸前跡線 x(t) 隨時間衰減：x(t) = x · e^(-Δt / τ_+)
  2. 觸發長時程增強 (LTP)：W = W + A_+ · x(t)
  3. 突觸後跡線 y(t) 瞬時遞增：y(t) = y(t) + 1
```

胞體只需在 64-byte 狀態內存儲單一純量 `post_trace: f32` 與 `last_spike_tick: u32`，即可以 $O(1)$ 複雜度完成局部突觸權重自主演化，**完全不依賴全域梯度反向傳播**。

---

### 3.5 階層式分頁目錄與代謝持久化 (Paging & Eviction)

#### 3.5.1 兩級稀疏虛擬頁表 (Hierarchical Sparse Page Directory)
將 64-bit `PackedId` 轉換為記憶體指針的過程，完全映射作業系統虛擬記憶體分頁機制：

```
PackedId: [Region: 16b] -> [Column: 16b] -> [Neuron: 32b]
                 │               │                │
                 ▼               ▼                ▼
        ┌────────────────┐
        │  Region Table  │ (65,536 項，二級指針陣列)
        └───────┬────────┘
                │ 定址
                ▼
        ┌────────────────┐
        │  Column Table  │ (65,536 項，指向 Column Chunk 的實體指標)
        └───────┬────────┘
                │ 定址
                ▼
        ┌──────────────────────────────────────────────────┐
        │ Column Chunk (微皮質柱連續實體記憶體分頁)          │
        │ • 包含 1,024 ~ 4,096 個 NeuronState (連續陣列)    │
        │ • 局部突觸拓撲塊 (Connectome Chunks)              │
        └──────────────────────────────────────────────────┘
```

* **常數時間定址**：透過兩次陣列解引用（位移與遮罩）直接命中記憶體，尋址延遲 $< 5\,\text{ns}$，消滅全域 HashMap 的雜湊碰撞與隨機尋址開銷。

#### 3.5.2 以皮質柱為單位的時鐘掃描 (Column-level Clock Sweep)
若以單個神經元為單位進行記憶體淘汰，會產生極嚴重的記憶體碎片與龐大元數據。因此，**代謝淘汰的最小物理單元為「微皮質柱（Column Chunk）」**。

1. **巡檢執行緒（Clock Sweep Worker）**：常駐於最低優先級核心，循環遍歷活躍的 Column 陣列。
2. **淘汰判定**：若柱內所有神經元均滿足 `(Current_Tick - last_active_tick) > EVICTION_THRESHOLD` 且信箱全空。
3. **分頁持久化**：
   - 將整塊 Column Chunk（包含 1024 個神經元胞體及對應突觸）透過零拷貝方式寫入嵌入式儲存引擎（如 `redb`）。
   - 將二級目錄對應項置為空指針（`null`），釋放實體記憶體分頁。
4. **惰性加載（Lazy Hydration）**：
   當後續脈衝命中已釋放的 Column 時，觸發「軟體分頁中斷（Software Page Fault）」，從磁碟循序讀取 64KB~256KB 的整塊分頁，重新掛載至目錄。

---

## 4. 動態資訊流：單一脈衝的生命週期與全域時序

```
══════════════════════════════════════════════════════════════════════════════════════════════════════════════════
                                    單一脈衝傳導與時序演化時序圖
══════════════════════════════════════════════════════════════════════════════════════════════════════════════════

  [突觸前放電 (Pre-Neuron Fire)]
               │
               ▼
  [查詢下游突觸拓撲 (Lookup Synapse Fabric)]
               │ 遍歷 Target_Id, Weight, Delay
               ├─── 帶延遲 (Delay > 0) ───> 存入發送端 Worker 本地時間輪 [Slot: (T + Delay) % 256]
               │                                      │
               │                                      ▼ (等待時鐘滴步進至 T + Delay)
               │                             [時間輪倒出 Phase 1]
               │                                      │
               └─── 零延遲 (Delay = 0) ───────────────┤
                                                      ▼
                                       [投遞至目標信箱 (Enqueue Target Mailbox)]
                                         • 原子 Treiber Stack 插入
                                         • CAS 檢查目標神經元 gate_state
                                                      │
                       ┌──────────────────────────────┴──────────────────────────────┐
                       ▼ (CAS 0 -> 1 成功: 原為 IDLE)                                ▼ (已在 QUEUED/RUNNING 態)
          【推入 Worker 本地工作竊取佇列】                                 【發送端直接返回，零等待】
                       │
                       ▼
          【Phase 2: Worker 領取執行權 (CAS 1 -> 2)】
                       │
                       ▼
          【原子 SWAP 抽乾信箱所有 Spike】 ──> 批次節點回收至 Local Slab
                       │
                       ▼
          【LIF 查表指數衰減 + 向量化膜電位累加】
                       │
                       ├─── 未達閾值 (V_mem < V_thresh) ───> 轉回 IDLE (CAS 2 -> 0)，釋放算力
                       │
                       └─── 突破閾值 (V_mem >= V_thresh) ──> 【產生動作電位 (FIRE!)】
                                                                  │
                                            ┌─────────────────────┴─────────────────────┐
                                            ▼                                           ▼
                                  [重置電位與進入不反應期]                    [在線更新突觸後 STDP 跡線]
                                            │                                           │
                                            └─────────────────────┬─────────────────────┘
                                                                  ▼
                                                      【向突觸拓撲廣播下游脈衝】(進入下一輪循環)
══════════════════════════════════════════════════════════════════════════════════════════════════════════════════
```

---

## 5. 外部感官輸入與動態輸出介面 (Sensory I/O Fabric)

神經擬態引擎必須與外部現實世界的即時資料流無縫對接。

### 5.1 感官事件注入 (Sensory Ingestion Pipeline)
外部感官來源（如動態視覺感測器 DVS Event Camera、神經音訊特徵流、連續向量 Embedding）透過無鎖環形通道（SPSC RingBuffer）注入：

1. **泊松編碼器（Poisson Rate Encoder）**：將浮點強度值轉化為服從泊松分佈的隨機脈衝序列。
2. **延遲編碼器（Time-to-First-Spike / Latency Encoder）**：強度越高，放電時間越早（延遲越短），以極致的脈衝時序稀疏性表達連續數值。
3. 注入的脈衝直接打入特定輸入腦區（Sensory Cortex）神經元的時間輪槽位，**外部 I/O 執行緒與核心運算 Worker 執行緒之間完全透過無鎖單向緩衝隔離**。

### 5.2 決策與運動讀出 (Motor & Decision Readout)
1. **群體發放率解碼（Population Rate Decoding）**：統計特定讀出核團（Readout Nucleus）在特定時間窗口（如 10 Ticks）內的總放電次數。
2. **首脈衝贏者全拿解碼（First-to-Spike Decoding）**：監控一組競爭神經元，以最先產生動作電位的神經元代表引擎的即時分類與控制決策，延遲低於 1 毫秒。

---

## 6. 架構優勢、物理界限與技術權衡 (Trade-Offs & Boundaries)

### 6.1 本架構的決定性優勢
1. **極致能效比與超高密度**：
   - 相比於傳統 Actor 框架（Akka / Orleans 每個實例需幾百位元組至數千位元組），VirtualCortex 胞體**嚴格 64 Bytes**，記憶體密度提升 **20 至 50 倍**。
   - 單台 128GB 主機可同時維護 **1 億個虛擬神經元** 與數十億條突觸連線。
2. **硬即時微秒級時序**：
   - 杜絕一切 GC Pause 與動態反射。端到端脈衝傳導延遲穩定保持在 **數十至數百奈秒（ns）**，達到工業自動化與機器人高頻運動控制的硬即時要求。
3. **零全域反向傳播的自主適應**：
   - 依托在線 STDP 與局部側向抑制，神經網路可在運行過程中依據外部即時反饋進行持續終身學習（Continual Learning），徹底告別巨額重訓練停機。

### 6.2 系統架構邊界與禁忌 (What NOT to do)
1. **嚴禁全連通稠密矩陣相乘**：
   - 本架構不適用於 Dense LLM（如標準 Transformer）。若網路為全連通圖（All-to-all），信箱與時間輪將瞬間爆滿，記憶體頻寬將徹底枯竭。系統嚴格依賴於真實大腦的**小世界網路（Small-World）與高度稀疏性（活躍度 $\le 2\%$）**。
2. **不引進分散式共識協議**：
   - 本系統堅持單機物理邊界。不內建 Raft 或 Paxos 等跨機共識。容錯與持久化依賴本地 NVMe 高速快照，而非跨節點網絡複本。
3. **嚴禁在 Worker 熱路徑進行阻塞性 I/O**：
   - 所有硬碟 I/O（分頁換出/換入）與外部網路互動均交由背景專屬 I/O 執行緒處理，絕不阻塞 64 個核心 Worker 的時鐘推進。

---

## 7. 生產級工程交付藍圖 (Handoff Blueprint for AI Agents & Engineers)

本章節為後續交由 **Claude Code**、工程團隊或自研編譯器進行具體專案落地時的模組劃分與里程碑驗收規格：

### 專案工程骨架 (Crate Directory Structure)
```
virtual_cortex/
├── Cargo.toml                     # 依賴: crossbeam, redb, core_affinity, bitflags
├── src/
│   ├── lib.rs
│   ├── identity/                  # 模組 1: 識別碼與階層定址
│   │   ├── mod.rs
│   │   └── packed_id.rs           # PackedId 實作與位元遮罩操作
│   ├── state/                     # 模組 2: 記憶體排布與胞體
│   │   ├── mod.rs
│   │   ├── neuron.rs              # 64-byte 對齊 NeuronState
│   │   └── connectome.rs          # Chunked CSR 突觸拓撲陣列
│   ├── mailbox/                   # 模組 3: 侵入式信箱與門控
│   │   ├── mod.rs
│   │   ├── treiber.rs             # 單原子指針無鎖 Treiber Stack
│   │   └── slab.rs                # Thread-Local SpikeNode 記憶體池
│   ├── timing/                    # 模組 4: 時間輪與時序步進
│   │   ├── mod.rs
│   │   ├── wheel.rs               # Per-Worker 環形時間輪
│   │   └── barrier.rs             # 雙緩衝 Phased Epoch 屏障
│   ├── compute/                   # 模組 5: 生物物理與學習規則
│   │   ├── mod.rs
│   │   ├── lif.rs                 # 查表化 LIF 動態更新
│   │   └── stdp.rs                # 在線雙跡線 STDP 算法
│   ├── paging/                    # 模組 6: 階層分頁與代謝持久化
│   │   ├── mod.rs
│   │   ├── directory.rs           # 兩級 Sparse Page Table
│   │   └── clock_sweep.rs         # 微皮質柱代謝淘汰與 redb 儲存
│   └── engine.rs                  # 引擎主調度入口與 I/O 介面
└── benches/
    └── spike_throughput.rs        # 1 億脈衝基準效能壓測
```

---

### Milestone 1: 記憶體基石與原子門控 (Core Memory & Gating)
* **產出物**：`packed_id.rs`, `neuron.rs`, `treiber.rs`。
* **驗收標準**：
  1. `assert_eq!(std::mem::size_of::<NeuronState>(), 64);`
  2. `assert_eq!(std::mem::align_of::<NeuronState>(), 64);`
  3. 單元測試：1,000 個並發執行緒同時向單一 Actor 發送 1,000,000 個脈衝，驗證門控狀態機（`IDLE -> QUEUED -> RUNNING`）完全無競態條件，單原子抽乾後事件無遺漏。

### Milestone 2: 拓撲架構與 Per-Worker 時間輪 (Topology & Local Wheels)
* **產出物**：`connectome.rs`, `wheel.rs`, `slab.rs`。
* **驗收標準**：
  1. 實作 `SynapseChunk` 記憶體分配器，驗證 100,000 個下游突觸遍歷速度達到了硬體記憶體頻寬極限（SIMD 預取有效）。
  2. 構建 Worker 私有時間輪，單元測試驗證 $D \in [1, 255]$ 滴答延遲事件的精確投放與槽位迴圈。

### Milestone 3: 工作竊取排程與分相推進 (Scheduler & Phased Epoch)
* **產出物**：`barrier.rs`, `engine.rs`。
* **驗收標準**：
  1. 綁定 64 個 Worker 至實體 CPU 核心（啟用 Core Affinity）。
  2. 搭建由 3 個神經元構成的循環脈衝振盪迴路（Neuron A $\xrightarrow{2\text{ms}}$ Neuron B $\xrightarrow{2\text{ms}}$ Neuron C $\xrightarrow{2\text{ms}}$ Neuron A）。
  3. 驗證全系統在 10,000 個連續 Ticks 推進過程中，振盪週期誤差嚴格為 **0 滴答**（絕對確定性時序）。

### Milestone 4: 生物物理查表化與在線 STDP (Biophysics & STDP)
* **產出物**：`lif.rs`, `stdp.rs`。
* **驗收標準**：
  1. 膜電位衰減運算呼叫次數達 1 億次時，LUT 查表吞吐量較標準 `f32::exp()` 提升 **5 倍以上**。
  2. 模擬典型雙神經元前-後激發場景，驗證突觸權重依據脈衝時差 $\Delta t$ 呈現標準非對稱雙指數 STDP 學習曲線。

### Milestone 5: 階層分頁、代謝淘汰與極限壓測 (Eviction & 100M Spike Benchmark)
* **產出物**：`directory.rs`, `clock_sweep.rs`, `benches/spike_throughput.rs`。
* **驗收標準**：
  1. 初始化 10,000,000 個虛擬神經元，記憶體佔用控制在 20GB 以內。
  2. 啟動背景時鐘掃描，驗證長期沉寂的 Column 自動被安全序列化至 `redb`，記憶體釋放率 $> 80\%$。
  3. 對休眠神經元注入脈衝，驗證其在 $< 50\,\mu\text{s}$ 內完成透明加載並正常參與後續計算。
  4. 最終綜合壓測：在 64 核伺服器上達成 **> 100 MSpikes / sec** 穩定處理能力，中位數事件派發延遲 $< 300\,\text{ns}$。

---

## 8. 結論 (Conclusion)

VirtualCortex 突破了傳統深度學習對大型同步張量計算的依賴，亦擺脫了分散式微服務 Actor 框架在單機維度上的沉重包袱。

藉由將**胞體狀態壓縮為精確的 64-byte 快取行、拓撲結構分塊管理、信箱侵入式單原子指針化、軸突時間輪分散私有化，以及雙緩衝分相步進推進**，VirtualCortex 在通用 x86_64 / ARM64 伺服器架構上，重新打造出了一座高密度、硬即時、事件驅動的軟體神經擬態大腦。這份架構規範提供了嚴謹的物理與工程依據，足供現代高效能計算團隊與自主演化系統工程直接落實。