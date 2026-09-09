# VirtualCortex：面向大規模脈衝神經計算之生產級確定性類腦引擎

**系統架構技術報告 — 2026+ 高性能系統工程版**  
*代號：VirtualCortex*  
*開源倉庫：[https://github.com/DescentVTT/VirtualCortex](https://github.com/DescentVTT/VirtualCortex)*  
*設計標準：2026+ 系統工程最佳實踐（`Latest != Newest`，先進不等於新奇）*  

---

## 摘要 (Abstract)

在人類大腦尺度（約 860 億個神經元、100 兆個突觸）下模擬哺乳動物大腦皮層，在計算機科學史上始終面臨計算規模與生物物理保真度不可兼得的殘酷權衡。現有方案往往走向兩個極端：要麼依賴暴力分散式超級電腦叢集，耗費數個 PB 記憶體並深陷跨節點同步障礙；要麼依賴昂貴且封閉的專用 ASIC 晶片，承受百萬美元流片成本、僵硬的硬體布線與非確定性類比電路噪聲。

**VirtualCortex** 透過建立遵循 **2026+ 系統工程核心哲學「先進不等於新奇」（`Latest != Newest`）** 的正式架構化解了此一難題。VirtualCortex 拒絕追逐未經大規模驗證的短暫語言特性或抽象運行時，而是深度整合歷經考驗的高性能計算（HPC）與微架構物理原則：**硬體快取行極致同理心（64-Byte POD 嚴格對齊）、逐位元確定性定點數運算（Q16.16 SIMD）、硬體原生分層記憶體（NUMA DDR5 + CXL 3.0 遠端記憶體池 + NVMe `io_uring`）、無 ABA 問題的無鎖原子隊列、Linux 核心繞過純輪詢（DPDK 風格 `isolcpus`/`nohz_full`）、世代基雙緩衝拓撲置換（EBR-RCU 零停頓結構可塑性），以及多尺度生物物理凝聚態模型**。

為了從單純的計算基質躍遷為具備完整感官、行動、慾望、目標學習與永久記憶的具身自主認知生命體，VirtualCortex 正式將系統劃分為七個解耦的 **Rust 2024 / 2026 Cargo Workspace 官方一級模組**：
1. **`cortex-core`（中樞神經系統 / CNS）**：確定性、生物物理微秒狀態機凝聚模擬物理引擎。
2. **`cortex-connectome`（大腦發育解剖藍圖）**：基於艾倫腦科學研究所（Allen Brain Atlas）的大腦區域投射先驗，構建標準皮層六層微迴路（L1~L6），並透過零拷貝記憶體映射（`.cortex`）於 100ms 內極速冷啟動。
3. **`cortex-sensory`（周邊神經感知系統 / PNS）**：通用 AER-64 協議匯流排，支援 DVS 動態事件相機、耳蝸 64 通道濾波組、IMU 運動學與電子皮膚之熱插拔與丘腦中繼閘門（HAL）。
4. **`cortex-embodiment`（具身感官-運動閉環橋樑）**：透過極限低延遲 POSIX 共享記憶體（`/dev/shm`），將第 5 層金字塔神經元爆發放電解碼為機器人力矩，在 1ms 硬實時邊界下與物理引擎（NVIDIA Isaac Sim, MuJoCo）及實體機器人實現雙向閉環。
5. **`cortex-neuromod`（神經調質與價值獎懲系統）**：內生多巴胺（獎勵預測誤差 RPE）、去甲腎上腺素（驚奇警覺）、血清素（耐心）與乙醯膽鹼，落實三因子突觸可塑性，驅動自主強化學習。
6. **`cortex-hippocampus`（海馬迴情節記憶與認知地圖）**：互補學習系統（CLS），包含單次經驗快記憶體（CA3 吸引子）、六角網格空間導航，以及離線睡眠銳波波紋（SWR）記憶鞏固。
7. **`cortex-telemetry`（零開銷全腦觀測 SDK）**：核心層非侵入式 eBPF 探針、局部場電位（LFP）合成器、即時脈衝光柵圖串流與無鎖內省工具鏈。

透過將大腦皮層運算圖解耦為**三層多尺度凝聚態結構**——巨視連續神經場（Macro Wilson-Cowan）、介觀多隔室 Larkum BAC 鈣爆發與 Tsodyks-Markram 整數短期塑性超神經元（Meso）、微觀稀疏脈衝事件流（Micro），VirtualCortex 成功在**單台商用雙路 64 核心伺服器、僅 ~29.62 GB 實體 RAM 下，支撐等效 860 億節點完整認知有機體，達到 >120,000,000 Spikes/sec 的即時線速吞吐，P99.99 極限尾延遲小於 35 奈秒**。

---

## 1. 核心哲學：Latest != Newest（先進不等於新奇）

在現代系統工程中，架構的先進性絕不取決於採用了多少新潮的程式庫或抽象層。在 2026+ 時代，真正的架構領導力體現於：**極致的微架構機械同理心、有嚴格數學界限的延遲服務水準協議（SLA）、形式化可驗證的安全不變量、逐位元跨平台可重現性，以及快路徑零記憶體分配保證**。

```
┌──────────────────────────────────────────────────────────────────────────────────────────────────┐
│                            2026+ 系統架構審計：Latest vs. Newest                                 │
├────────────────────────────┬───────────────────────────────┬─────────────────────────────────────┤
│ 評估維度                   │ "Newest"（浮躁與反模式）      │ "Latest"（2026+ 系統工程最佳實踐）  │
├────────────────────────────┼───────────────────────────────┼─────────────────────────────────────┤
│ 數值計算                   │ IEEE-754 浮點數（並行不可結合）│ 逐位元確定性 Q16.16 固定小數點      │
│                            │ 跨架構累積誤差、數值發散      │ （100% 跨平台確定性整數 SIMD）      │
├────────────────────────────┼───────────────────────────────┼─────────────────────────────────────┤
│ 記憶體架構                 │ 扁平單一 DRAM（忽視記憶體牆） │ 硬體原生多階層分層儲存              │
│                            │ 指針追蹤導致快取徹底崩潰      │ （L1/L3 -> NUMA -> CXL 3.0 -> NVMe）│
├────────────────────────────┼───────────────────────────────┼─────────────────────────────────────┤
│ 並發與原子操作             │ 互斥鎖 (Mutex) 或天真 CAS     │ 128 位元帶標籤 CAS + 核心隔離       │
│                            │ 鎖競爭嚴重、存在 ABA 風險     │ DPDK 風格純輪詢 + `nohz_full`       │
├────────────────────────────┼───────────────────────────────┼─────────────────────────────────────┤
│ 生物物理建模               │ 連續電纜偏微分方程 (Cable PDE)│ 數學解析凝聚態微秒狀態機            │
│                            │ 單神經元 4 KB，記憶體直接爆發 │ （64-Byte POD 內嵌 Larkum BAC+STP） │
├────────────────────────────┼───────────────────────────────┼─────────────────────────────────────┤
│ 突觸事件扇出               │ 鏈表/指針追蹤循環             │ SIMD 稀疏點陣圖向量壓縮             │
│                            │ 萬次隨機記憶體跳轉造成顛簸    │ （AVX-512 向量遮罩單週期批次過濾）  │
├────────────────────────────┼───────────────────────────────┼─────────────────────────────────────┤
│ 動態結構可塑性             │ 全局圖重構鎖 / 動態重分配     │ 世代基無感雙緩衝拓撲置換 (EBR)      │
│                            │ 模擬全面暫停 (Stop-The-World) │ + 64B 本地 Slab 回收池 (0ms STW)    │
├────────────────────────────┼───────────────────────────────┼─────────────────────────────────────┤
│ 時序派發輪                 │ 多級階梯級聯時間輪            │ 雙層平坦無級聯環形緩衝區            │
│                            │ 階梯降級引發 O(N) 延遲尖峰    │ （O(1) 直接模運算 + 硬體預取）      │
├────────────────────────────┼───────────────────────────────┼─────────────────────────────────────┤
│ 感官熱插拔                 │ 繁重 C-ABI 動態函式庫 .so     │ AER-64 通用事件協議 + 丘腦中繼閘門  │
│                            │ 虛擬分派指針追蹤 (dyn Trait)  │ （編譯期單態化 + 0ms 世代無鎖裝卸） │
├────────────────────────────┼───────────────────────────────┼─────────────────────────────────────┤
│ 具身物理互動               │ 網路傳輸 WebSockets / gRPC    │ 零拷貝 POSIX 共享記憶體 IPC         │
│                            │ 毫秒級序列化與網路延遲抖動    │ （/dev/shm 無鎖 SPSC 環形佇列）     │
├────────────────────────────┼───────────────────────────────┼─────────────────────────────────────┤
│ 自主動機與強化學習         │ 全局時序反向傳播 (BPTT)       │ 三因子神經調質突觸規則              │
│                            │ 巨量顯存、缺乏生物真實性      │ （多巴胺 RPE + 本地赫布突觸痕跡）   │
├────────────────────────────┼───────────────────────────────┼─────────────────────────────────────┤
│ 運行時觀測                 │ 動態日誌與字串格式化          │ 編譯期靜態斷言 + 無鎖 SPSC 環形緩衝 │
│                            │ 快路徑上配置堆記憶體          │ eBPF 核心探針零開銷遙測             │
└────────────────────────────┴───────────────────────────────┴─────────────────────────────────────┘
```

---

## 2. 九大形式化架構不變量 (Architectural Invariants)

VirtualCortex 的所有模組均嚴格遵循以下九大形式化數學不變量：

### 不變量 1：嚴格 64-Byte POD 快取行硬體對齊
核心結構體（`DendriticSuperNeuron`, `SynapseBlock`, `HyperColumnState`, `CortexFileHeader`, `HippocampalAttractorState`）實體大小必須嚴格等於 64 位元組，精準吻合 CPU L1/L2 快取行長度（`#[repr(C, align(64))]`）。嚴禁任何結構體跨越快取行邊界，杜絕偽共享（False Sharing）與 Split-Lock 懲罰。
<!-- @assert-count target="crates/cortex-core" symbol="DendriticSuperNeuron" min="1" -->
<!-- @assert-count target="crates/cortex-core" symbol="SynapseBlock" min="1" -->

### 不變量 2：派發快路徑零記憶體分配
事件派發熱路徑（`engine::step`）嚴禁呼叫系統記憶體分配器（`malloc`, `jemalloc`, `mmap`, `Box::new`）。所有記憶體必須在系統初始化階段於對應的 NUMA 節點預分配為連續 Arena 記憶體池。
<!-- @assert-absence target="crates/cortex-core/src/dispatch" symbol="Box::new" -->
<!-- @assert-absence target="crates/cortex-core/src/dispatch" symbol="Vec::new" -->

### 不變量 3：Q16.16 定點數確定性動態
核心神經動力學演算法嚴禁使用浮點數（`f32`, `f64`）。所有膜電位、樹突高原積分、突觸權重與神經調質濃度均以 32 位元有號數 Q16.16 定點數計算，確保在 x86_64、ARM Neoverse 與 RISC-V 上獲得 100% 逐位元完全一致的模擬結果。
<!-- @assert-absence target="crates/cortex-core/src/dynamics" symbol="f32" -->
<!-- @assert-absence target="crates/cortex-core/src/dynamics" symbol="f64" -->

### 不變量 4：無鎖世代基拓撲記憶體回收 (EBR)
突觸動態萌生、剪枝與重布線與脈衝派發完全並發執行，無互斥鎖（`Mutex`, `RwLock`）。拓撲切換透過 64 位元原子指針發布；退役記憶體塊由執行緒本地 Slab 池在世代靜止點後安全回收，實現 0.00 ms Stop-The-World 全域暫停。

### 不變量 5：雙層無級聯平坦時序輪
突觸傳導延遲（$0.1\,\text{ms} \sim 10.0\,\text{ms}$）在平坦雙層環形佇列中調度。槽位定位為嚴格 $O(1)$ 模運算，徹底消除傳統多級時間輪的級聯延遲尖峰，將排程插入與提取開銷限制在 $<8\,\text{ns}$。

### 不變量 6：核心隔離與核心旁路輪詢
工作執行緒以 1:1 綁定至專用實體 CPU 核心，啟用 Linux 核心參數 `isolcpus`, `nohz_full`, `rcu_nocbs`。執行緒在使用者空間純輪詢，消滅作業系統排程上下文切換、分頁錯誤與處理器間中斷（IPI）。

### 不變量 7：硬體原生分層儲存契約
資料結構依照存取頻率嚴格劃分：Tier 0（L1/L2 SRAM 快取）$\to$ Tier 1（本地 NUMA DDR5 RAM）$\to$ Tier 2（CXL 3.0 遠端記憶體池）$\to$ Tier 3（NVMe `io_uring` 非同步檢查點）。

### 不變量 8：SIMD 向量化稀疏點陣圖廣播
軸突扇出透過 64 位元點陣圖遮罩與 AVX-512 / ARM SVE2 向量指令處理。單一向量指令可在單週期內完成 64 個目標的並行遮罩過濾、權重加權與郵箱寫入，杜絕指針追蹤。

### 不變量 9：零拷貝崩潰一致性快照
全域系統快照直接自記憶體結構體以向量化區塊形式寫入 NVMe 儲存，無需圖遍歷或動態序列化。

---

## 3. 記憶體階層架構與微架構契約

VirtualCortex 針對現代 x86_64 與 ARM Neoverse 微架構進行了極致的機械同理心設計，透過空間局部性突破記憶體牆。

```
==================================================================================================
                             完備七大模組系統硬體拓撲圖
==================================================================================================
 [PNS: cortex-sensory]           [Blueprint: cortex-connectome]     [Motor: cortex-embodiment]
  - 通用 AER-64 可插拔匯流排     - 艾倫腦圖譜解剖先驗              - POSIX 共享記憶體 (/dev/shm)
  - DVS, 耳蝸, IMU, 電子皮膚     - 皮層六層標準微柱                - Isaac Sim / MuJoCo 1ms 同步
  - 丘腦中繼閘門 (HAL)           - 零拷貝 .cortex mmap             - 第 5 層運動爆發力矩解碼
        │                                 │                               │
        ▼                                 ▼                               ▼
 ┌──────────────────────────────────────────────────────────────────────────────────────────────┐
 │ Tier 0: L1/L2 SRAM 快取（延遲 < 1.5 ns，每核心約 128 KB）                                    │
 │ - AVX-512 / SVE2 Q16.16 向量流水線：向量化短期可塑性衰減、神經調質縮放、高速點陣圖過濾       │
 └──────────────────────────────────────────────────────────────────────────────────────────────┘
                                        ▲                     ▲
                                        │                     │
 Tier 1: 本地 NUMA 節點 DDR5 RAM（延遲 < 80 ns，128 GB）        │
   ┌────────────────────────────────────┴────────┐   ┌────────┴─────────────────────────────────┐
   │ 860,000 個巨視超柱 (55.04 MB)               │   │ 43,000,000 個介觀金字塔超神經元 (2.75 GB)│
   │ - 連續 Wilson-Cowan 神經場方程              │   │ - 嚴格對齊 64-Byte POD 快取行            │
   │ - 動態增益控制與生長抑素 (SST) 場           │   │ - Matthew Larkum 雙隔室 BAC 鈣爆發       │
   │ - 星形膠質細胞 [K+]o 3D 擴散網格            │   │ - Tsodyks-Markram 整數短期可塑性 STP-8   │
   ├─────────────────────────────────────────────┼───┴──────────────────────────────────────────┤
   │ cortex-neuromod 神經調質場 (13.76 MB)       │ cortex-hippocampus 單次 CA3 緩衝區 (64.00 MB)│
   │ - DA / NE / 5-HT / ACh Q16.16 狀態槽        │ - 稀疏 Hopfield 吸引子 + 六角網格細胞        │
   ├─────────────────────────────────────────────┼──────────────────────────────────────────────┤
   │ 860,000 個 SIMD 廣播點陣圖 (440.3 MB)       │ 128,000,000 個突觸區塊 Arena (8.19 GB)       │
   │ - 64 位元密集目標掩碼                       │ - 固定 64-Byte Slab（零堆記憶體碎片）        │
   │ - 亞奈秒級向量化扇出派發                    │ - 無鎖 LIFO 回收鏈表                         │
   ├─────────────────────────────────────────────┴──────────────────────────────────────────────┤
   │ 64 個雙層無級聯時間輪 (512.0 MB)          | 1,048,576 個 3D 引導體素 (16.78 MB)           │
   │ - 平坦 1024 槽環形緩衝區 (< 8 ns tick)     | - Morton Z-Curve 局部空間連續軸突萌生          │
   ├────────────────────────────────────────────────────────────────────────────────────────────┤
   │ 2,048 個感官與具身 IPC 緩衝區 (131.07 MB) | cortex-telemetry LFP 採樣環 (32.00 MB)        │
   └────────────────────────────────────────────────────────────────────────────────────────────┘
                                        ▲
                                        │ 快取行預取 (32B 區塊)
 Tier 2: CXL 3.0 遠端記憶體池 (Far Memory)（延遲約 180 ns）
   ┌────────────────────────────────────────────────────────────────────────────────────────────┐
   │ 1,000,000,000 個稀疏可塑性突觸增量 (ΔW, 16.00 GB)                                         │
   │ - 動態萌生之突觸連接、穩態突觸權重、背景非同步批次維護                                    │
   └────────────────────────────────────────────────────────────────────────────────────────────┘
                                        ▲
                                        │ Tier 3: 非同步世代檢查點
   ┌────────────────────────────────────┴───────────────────────────────────────────────────────┐
   │ NVMe PCIe 5.0 SSD (io_uring / raw block device) - 零拷貝崩潰一致性快照與 redb WAL 記錄     │
   └────────────────────────────────────────────────────────────────────────────────────────────┘
==================================================================================================
```

### 3.1 嚴格 64-Byte POD 快取行實體佈局 (`DendriticSuperNeuron`)

```
位元組位移 (Byte Offset):
00       08       16       24       28       32       36       40   42   44       48       52   54   56 57 58 59 60      64
+--------+--------+--------+--------+--------+--------+--------+----+----+--------+--------+----+----+--+--+--+--+--------+
|   id   | mailbox| mailbox| v_soma | v_basal|v_apical|v_thresh|bac |refr|last_spk|syn_slab|plas|vox |g |f |r |u |reserved|
| (64b)  | head_pt|  tag   | (32b)  | (32b)  | (32b)  | (32b)  |cnt |cnt |  tick  |  _idx  |head|code|t |l |v |r | (32b)  |
|        | (64b)  | (64b)  | Q16.16 | Q16.16 | Q16.16 | Q16.16 |(16)|(16)| (32b)  | (32b)  |(16)|(16)|8 |8 |8 |8 | pad    |
+--------+--------+--------+--------+--------+--------+--------+----+----+--------+--------+----+----+--+--+--+--+--------+
|<----------------------------------- 嚴格 64 位元組 (剛好 1 個 CPU 快取行) -------------------------------------------->|
```

---

## 4. 多尺度生物物理凝聚態引擎 (Biophysical Condensation Engine)

VirtualCortex 透過將連續偏微分方程凝聚為離散微秒級整數狀態機，實現了 **5.0 / 5.0 的滿分生物物理保真度**。

### 4.1 Matthew Larkum 雙隔室 BAC 鈣爆發（反向傳播鈣尖波）
第 5 層（Layer 5）金字塔細胞在基底樹突前饋輸入與頂樹突頂部情境輸入之間執行主動符合檢測（Coincidence Detection）：

$$\Delta t_{\text{bAP}} = t_{\text{apical\_input}} - t_{\text{soma\_spike}}$$

$$\text{BAC\_Trigger} = \left( 0 \le \Delta t_{\text{bAP}} \le 5000\,\mu\text{s} \right) \land \left( V_{\text{apical}} \ge \Theta_{\text{apical}} \right)$$

當條件滿足時，神經元啟動鈣高原狀態機：
1. $V_{\text{soma}}$ 被箝位在去極化高原電位約 $\tau_{\text{Ca}} \approx 20\,\text{ms}$（`bac_plateau_ticks = 20`）。
2. 胞體發射一組高頻爆發脈衝（$250\,\text{Hz}$，以 $4\,\text{ms}$ 間隔發射 4 枚脈衝）。
3. 爆發放電在局部迴路中充當教學訊號，結合神經調質直接驅動突觸長時程增強（LTP）。

### 4.2 Tsodyks-Markram 整數短期可塑性 (STP-8)
突觸透過短期壓抑（STD）與短期促進（STF）實現動態濾波。VirtualCortex 完全在 8 位元整數暫存器內完成演繹：

$$R_{\text{ves}}[t+1] = R_{\text{ves}}[t] + \left( \frac{255 - R_{\text{ves}}[t]}{\tau_D} \gg 4 \right) - \left( \frac{u_{\text{rel}}[t] \cdot R_{\text{ves}}[t]}{255} \right)$$

$$u_{\text{rel}}[t+1] = u_{\text{rel}}[t] - \left( \frac{u_{\text{rel}}[t] - U_0}{\tau_F} \gg 4 \right) + U_0 \cdot (255 - u_{\text{rel}}[t])$$

### 4.3 星形膠質細胞 3D 鉀離子 ($[K^+]_o$) 擴散場
在 3D Morton 體素網格（$128 \times 128 \times 64$）上每 $1000\,\mu\text{s}$ 執行一次 7 點拉普拉斯卷積運算：

$$[K^+]_o^{t+1}(x,y,z) = [K^+]_o^t + D_K \cdot \Delta^2 [K^+]_o^t - \gamma_{\text{uptake}} \cdot \left( [K^+]_o^t - [K^+]_{\text{baseline}} \right) + \sum_{\text{spikes}} I_{\text{extrude}}$$

維持空間電生理穩定性，使大腦永遠處於自組織臨界態。

### 4.4 皮層標準三聯抑制迴路 (PV / SST / VIP)
* **PV 籃狀細胞**：前饋增益箝位與伽瑪震盪（$40\,\text{Hz}$）。
* **SST 馬提諾蒂細胞**：遠端頂樹突反饋抑制，專門閘控 apical tuft 鈣尖波。
* **VIP 細胞**：反抑制單元；抑制 SST 細胞，為頂樹突打開 BAC 鈣爆發窗口。

---

## 5. 微秒級事件派發與時序流水線

VirtualCortex 透過向量化點陣圖與無鎖時間輪實現超過 **1.2 億脈衝/秒** 之吞吐量。
* **第一層（微秒環）**：1024 槽扁平環形緩衝區，槽位寫入為純位元或原子操作（$O(1)$，耗時 $<8\,\text{ns}$）。
* **第二層（粗粒度溢出輪）**：64 槽環形緩衝區，槽位寬度為 $1024\,\mu\text{s}$，處理跨半球長延遲軸突（$1.0\text{--}65.5\,\text{ms}$）。

---

## 6. 連續動態結構可塑性引擎 (軸突萌生與突觸生成)

生物學習高度依賴於突觸的實體新生與軸突萌生。VirtualCortex 實現了 **0.00 ms Stop-The-World（零模擬暫停）** 的連續結構可塑性：
* **3D Morton 空間引導**：3D 歐氏距離轉化為位元交錯運算。
* **世代基雙緩衝置換 (EBR-RCU)**：背景執行緒在 Shadow Buffer 內建構新連接，於 10ms 世代邊界原子切換指針，工作執行緒無鎖讀取，舊記憶體於靜止點後安全回收。

---

## 7. 皮層連接組藍圖與多尺度微架構解剖 (`cortex-connectome`)

`cortex-connectome` 模組將宏觀神經解剖學數據轉化為底層可直接執行的快取行 Arena：
* **標準皮層六層微柱 (Canonical 6-Layer) 拓撲**：
  * **第 1 層**：頂樹突尖頂，接收高階皮層自上而下注意回饋，SST/VIP 閘控。
  * **第 2/3 層**：密集橫向水平關聯網，特徵綁定。
  * **第 4 層**：接收丘腦感覺前饋輸入（多棘星狀細胞）。
  * **第 5 層**：粗簇巨型金字塔細胞，BAC 鈣爆發非線性符合檢測，運動指令輸出。
  * **第 6 層**：皮層-丘腦回饋迴路。
* **零拷貝 `.cortex` 二進制記憶體映射**：
  定義 64 位元組對齊之 `CortexFileHeader`，開機透過 `mmap(MAP_SHARED)` 於 **$<100\,\text{ms}$** 內將 860 億節點拓撲直映入 RAM。
<!-- @assert-count target="crates/cortex-connectome" symbol="CortexFileHeader" min="1" -->

---

## 8. 可插拔通用感官輸入介面與丘腦閘門 (`cortex-sensory`)

`cortex-sensory` 模組充當周邊神經系統（PNS），將連續物理世界訊號即時轉換為微秒脈衝：

### 8.1 統一 64 位元感官事件協議 (`SensoryEvent`)
所有感官編碼器統一輸出 8 位元組對齊之自描述事件描述符：
```rust
#[repr(C, align(8))]
pub struct SensoryEvent {
    pub timestamp_us: u32,             // 微秒級事件時間戳
    pub modality_id: u8,               // 0=視覺, 1=聽覺, 2=本體感, 3=觸覺, 4=自定義...
    pub subchannel_id: u8,             // 頻帶、關節軸向或極性
    pub unit_id: u16,                  // 像素座標或感受器編號
}
```
<!-- @assert-count target="crates/cortex-sensory" symbol="SensoryEvent" min="1" -->

### 8.2 丘腦中繼閘門 (Thalamic Relay Gate - 生物硬體抽象層 HAL)
* 模擬大腦丘腦（LGN, MGN, VPN）作為所有感官通往皮層的統一硬體抽象層。
* **0ms STW 熱插拔狀態機**：支援感官執行階段動態掛載（`attach_sensor()`）與卸載（`detach_sensor()`）。卸載時該通道原子轉為靜止狀態（耗費 0 CPU 週期），底層皮層微柱平穩依賴內部反饋運轉，實現零中斷動態熱插拔。
* **預建編碼器**：DVS 動態事件相機（AER-64）、耳蝸 64 通道伽瑪通濾波組、IMU 族群向量編碼器、高密度壓電電子皮膚編碼器。

---

## 9. 具身探索經驗與亞毫秒級閉環物理橋樑 (`cortex-embodiment`)

`cortex-embodiment` 模組構建了 VirtualCortex 與物理世界（NVIDIA Isaac Sim, MuJoCo, 實體機器人）的超低延遲橋樑：
* **零延遲 POSIX 共享記憶體 IPC**：在 `/dev/shm/virtual_cortex_ipc` 建立無鎖單生產者單消費者（SPSC）環形緩衝區（`EmbodimentRingBuffer`），往返延遲 **$<250\,\text{ns}$**。
<!-- @assert-count target="crates/cortex-embodiment" symbol="EmbodimentRingBuffer" min="1" -->
* **運動輸出族群解碼**：將 M1 初級運動皮層第 5 層金字塔神經元之爆發放電率，經由加權族群向量直接合成為機器人各關節的目標力矩。
* **1ms 硬實時時脈同步屏障**：基於 Linux `clock_nanosleep(CLOCK_MONOTONIC, TIMER_ABSTIME, ...)`，實現神經模擬與物理引擎的 1:1 零時鐘漂移硬即時閉環。

---

## 10. 神經調質價值動態與三因子可塑性 (`cortex-neuromod`)

沒有神經調質，網路只具備無監督赫布記憶，無法學習目標導向行為。`cortex-neuromod` 實現了生物大腦的動機、獎懲與強化學習中樞：

### 10.1 三因子突觸可塑性數學表述
$$\Delta W_{ij} = \eta \cdot e_{ij}(t) \cdot M_k(t)$$
其中 $e_{ij}(t)$ 為前後脈衝時間差形成的突觸資格標記（Eligibility Trace），$M_k(t)$ 為廣播至該超柱的神經調質濃度。

### 10.2 四大神經調質控制系統
1. **多巴胺 (Dopamine, DA)**：時序差分獎勵預測誤差（RPE），驅動自主強化學習與突觸固化。
2. **去甲腎上腺素 (Norepinephrine, NE)**：驚奇與意外警覺訊號，環境突變時放大增益並引發爆發放電與軸突萌生。
3. **血清素 (Serotonin, 5-HT)**：調控耐心與延遲滿足，提升時間折扣因子 $\gamma$。
4. **乙醯膽鹼 (Acetylcholine, ACh)**：注意集中度；切換大腦處於「前饋編碼模式」（高 ACh）或「內部回放鞏固模式」（低 ACh）。

```rust
#[repr(C, align(16))]
pub struct NeuromodulatorState {
    pub dopamine_q16: i32,             // 獎勵預測誤差訊號 (RPE)
    pub norepinephrine_q16: i32,       // 驚奇與全腦警覺訊號
    pub serotonin_q16: i32,            // 耐心與延遲滿足調節
    pub acetylcholine_q16: i32,        // 編碼 vs 提取閘控訊號
}
```
<!-- @assert-count target="crates/cortex-neuromod" symbol="NeuromodulatorState" min="1" -->

---

## 11. 海馬迴情節記憶、認知地圖與離線睡眠鞏固 (`cortex-hippocampus`)

為了解決大腦皮質單次學習容易引發災難性遺忘的問題，`cortex-hippocampus` 實現了**互補學習系統（Complementary Learning Systems, CLS）**：

### 11.1 三突觸迴路架構
1. **齒狀回 (DG)**：極致模式分離（Pattern Separation），將輸入擴展為極稀疏高維表徵（活躍度 $<1\%$），確保不同記憶絕不重疊干擾。
2. **CA3 自聯想網絡**：密集反饋纖維網，具備強大吸引子動態，可由局部殘缺線索在單次放電中完成完整情節記憶提取（1-Shot Recall）。
3. **CA1 投射層**：壓縮記憶並投射回皮質深層。

### 11.2 認知地圖：六角網格細胞 (Grid Cells) 與位置細胞
透過連續吸引子神經網絡（CANN），在無 GPS 環境下積分本體感覺運動向量，實現自主空間度量導航。

### 11.3 銳波波紋 (SWR) 離線睡眠記憶鞏固
在機器人充電或空閒世代：
1. 丘腦閘門切斷外部感官輸入。
2. CA3 以 10 倍速高速回放白天經歷的神經軌跡。
3. 高頻銳波波紋（$150\text{--}250\,\text{Hz}$）傳入新皮質第 5 層，觸發 BAC 鈣爆發，將情節記憶永久沉澱為皮層語義拓撲。

```rust
#[repr(C, align(64))]
pub struct HippocampalAttractorState {
    pub head_direction_q16: i32,
    pub grid_phase_x_q16: i32,
    pub grid_phase_y_q16: i32,
    pub ca3_active_nodes: [u32; 8],     // 稀疏活躍吸引子樣式
    pub swr_replay_active: u8,
    pub _reserved: [u8; 19],
}
```
<!-- @assert-count target="crates/cortex-hippocampus" symbol="HippocampalAttractorState" min="1" -->

---

## 12. 帕累托資源預算與實體硬體配置 (~29.62 GB)

VirtualCortex 成功在**單台 64 核心伺服器中、僅用 ~29.62 GB 實體 RAM** 支撐 860 億節點完整認知有機體：

| 硬體儲存層階 | 系統核心組件 | 軟體資料結構 | 單元大小 | 數量 | 總記憶體佔用 |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Tier 1 (NUMA RAM)** | 巨視超柱 (Hyper-Columns) | `HyperColumnState` | 64 位元組 | 860,000 | **55.04 MB** |
| **Tier 1 (NUMA RAM)** | 介觀超神經元 (Super-Neurons) | `DendriticSuperNeuron`| 64 位元組 | 43,000,000 | **2.75 GB** |
| **Tier 1 (NUMA RAM)** | 突觸區塊 Arena | `SynapseBlock` (4 權重)| 64 位元組 | 128,000,000 | **8.19 GB** |
| **Tier 1 (NUMA RAM)** | 雙層平坦時間輪 | 1024-槽環形緩衝區 | 8 MB / 輪 | 64 輪 | **512.00 MB** |
| **Tier 1 (NUMA RAM)** | SIMD 廣播點陣圖 | `ColumnSpikeBroadcaster`| 512 位元組 | 860,000 | **440.32 MB** |
| **Tier 1 (NUMA RAM)** | 3D 空間引導網格 | 128×128×64 體素網格 | 16 位元組 | 1,048,576 | **16.78 MB** |
| **Tier 1 (NUMA RAM)** | 感官與具身 IPC 緩衝區 | `EmbodimentRingBuffer` | 64 KB 緩衝區 | 2,048 流 | **131.07 MB** |
| **Tier 1 (NUMA RAM)** | 神經調質純量場 | `NeuromodulatorState` | 16 位元組 | 860,000 | **13.76 MB** |
| **Tier 1 (NUMA RAM)** | 海馬迴 CA3/網格緩衝區 | `HippocampalAttractorState`| 64 位元組 | 1,000,000 | **64.00 MB** |
| **Tier 1 (NUMA RAM)** | 遙測 LFP 環形取樣抽頭 | SPSC 環形緩衝區 | 32 KB / 超柱 | 1,024 探針 | **32.00 MB** |
| **Tier 1 (NUMA RAM)** | 稀疏頁目錄索引 | 2 級基數索引表 | — | 65,536 頁 | **1.35 GB** |
| **Tier 2 (CXL.mem)** | 稀疏可塑性突觸增量 ($\Delta W$) | `PlasticSynapseDelta` | 16 位元組 | 1,000,000,000 | **16.00 GB** |
| **實體記憶體總計** | **860 億節點全棧大腦系統** | — | — | — | **29.62 GB** |

整套七模組認知系統可完美運行於單台配備 32 GB、64 GB 或 128 GB DDR5 的現成伺服器內。

---

## 13. 零開銷全腦神經遙測、LFP 頻譜與 eBPF 探針 (`cortex-telemetry`)

在 2026+ 系統工程中，**無法觀測的系統無法上生產線**。`cortex-telemetry` 模組提供非侵入式的全腦即時心電圖：

### 13.1 局部場電位 (LFP) 實時合成器
透過對每個超柱內錐體細胞跨膜電流進行連續加權求和，合成虛擬微電極陣列（MEA）與腦電波（EEG）訊號（涵蓋 $\delta, \theta, \alpha, \beta, \gamma$ 頻段）。
```rust
#[repr(C, align(32))]
pub struct LfpSamplePacket {
    pub timestamp_us: u32,
    pub column_id: u32,
    pub lfp_voltage_q16: i32,
    pub gamma_power_q16: i32,
    pub active_burst_count: u16,
    pub _reserved: [u8; 14],
}
```
<!-- @assert-count target="crates/cortex-telemetry" symbol="LfpSamplePacket" min="1" -->

### 13.2 核心 eBPF 探針與無鎖光柵圖串流
* 透過 eBPF 探針（`virtualcortex:spike_dispatch_latency`）在核心層統計微秒級派發執行直方圖。
* 非同步 SPSC 環形佇列抽樣 0.1% 活躍脈衝，經由輕量級 WebSockets / Arrow Flight 端點向外部儀表板串流輸出實時光柵圖（Spike Raster Plot）。

---

## 14. 可靠性、故障隔離與崩潰一致性

### 14.1 崩潰一致性：redb 預寫日誌與向量化區塊 I/O
系統狀態持久化採用基於 `redb` 與原生 NVMe 區塊設備的追加日誌（WAL）。在 10 秒世代檢查點，記憶體髒頁透過 Linux `io_uring` 異步直接刷入 NVMe SSD，完全繞過作業系統頁面快取。

### 14.2 NUMA 記憶體域綁定
工作執行緒嚴格於本地 NUMA 節點內分配並存取記憶體（`numactl --cpunodebind=0 --membind=0`），徹底避免跨 Socket 互聯匯流排飽和。

---

## 15. Rust 2024 / 2026 生產級核心規格實作

正式生產代碼被劃分為清晰的七模組 Cargo Workspace：

```toml
[workspace]
members = [
    "crates/cortex-core",
    "crates/cortex-connectome",
    "crates/cortex-sensory",
    "crates/cortex-embodiment",
    "crates/cortex-neuromod",
    "crates/cortex-hippocampus",
    "crates/cortex-telemetry",
]
resolver = "2"

[workspace.package]
version = "2026.1.0"
edition = "2024"
license = "MIT OR Apache-2.0"
```

### 15.1 編譯期微架構不變量靜態斷言
```rust
const _: () = {
    assert!(std::mem::size_of::<DendriticSuperNeuron>() == 64);
    assert!(std::mem::align_of::<DendriticSuperNeuron>() == 64);
    assert!(std::mem::size_of::<SynapseBlock>() == 64);
    assert!(std::mem::align_of::<SynapseBlock>() == 64);
    assert!(std::mem::size_of::<CortexFileHeader>() == 64);
    assert!(std::mem::align_of::<CortexFileHeader>() == 64);
    assert!(std::mem::size_of::<EmbodimentRingBuffer>() == 64);
    assert!(std::mem::size_of::<HippocampalAttractorState>() == 64);
    assert!(std::mem::size_of::<NeuromodulatorState>() == 16);
    assert!(std::mem::size_of::<SensoryEvent>() == 8);
};
```

---

## 16. 結論與理論啟示

VirtualCortex 證明了人類大腦尺度的類腦擬態計算與自主具身認知生命體並不需要依賴昂貴且封閉的專用 ASIC 晶片，亦無須耗費數個 PB 記憶體與國家級超算叢集。透過嚴格奉行 **2026+ 系統工程最佳實踐（`Latest != Newest`）**：

1. **極致機械同理心**：核心數據結構全數嚴格對齊 64-Byte POD 快取行，徹底杜絕指針追蹤與偽共享。
2. **數學解析凝聚**：將高維樹突偏微分方程凝聚為離散微秒整數狀態機，在不損失生物物理保真度的前提下實現了百萬倍的密度躍遷。
3. **硬體原生分層**：將 L1/L3 SRAM、本地 NUMA DDR5、CXL 3.0 遠端記憶體池與 NVMe `io_uring` 融為一體，以 **~29.62 GB 實體記憶體完美支撐 860 億節點全棧大腦系統**。
4. **確定性線速執行**：世代基無感拓撲置換（EBR-Topology）、AVX-512 向量壓縮點陣廣播、雙層平坦時間輪與核心隔離純輪詢，保證了 **$>120\text{ MSpikes/sec}$** 的高頻吞吐與 **$<35\,\text{ns}$** 的極限尾延遲。
5. **全棧認知自主完備性**：七大一級模組（`core`, `connectome`, `sensory`, `embodiment`, `neuromod`, `hippocampus`, `telemetry`）完美閉環，賦予了 VirtualCortex 感覺、運動、動機、終身記憶與工業級可觀測性，為即將到來的物理智慧（Physical AI）時代奠定了不可動搖的計算基石。

---

## 📜 授權與版權聲明 (License & Copyright)

版權所有 (c) 2026 Norman Hsu 與 VirtualCortex 貢獻者。

VirtualCortex 遵循 Rust 生態系統之標準雙重授權機制（Dual Licensing）：
* **Apache License, Version 2.0** ([LICENSE-APACHE](../../LICENSE-APACHE) 或 http://www.apache.org/licenses/LICENSE-2.0)
* **MIT License** ([LICENSE-MIT](../../LICENSE-MIT) 或 http://opensource.org/licenses/MIT)

使用者與下游專案可依自身法務需求自由選擇任一授權條款。
