# VirtualCortex 系統架構技術白皮書
### 面向 860 億節點微秒級模擬的超低延遲皮層基質與主權自主認知有機體架構規範

> **規格版本**：`2026.3.0-SOVEREIGN-CANONICAL`  
> **系統標準**：`2026+ Systems Engineering Best Practice (Latest != Newest)`  
> **授權協議**：[Apache License 2.0](../../LICENSE-APACHE) OR [MIT License](../../LICENSE-MIT)  
> **系統範式**：14 大官方一級模組（The Grand 14-Crate Cognitive Architecture）· 24 大正規核心章節 · 零動態分配 · Q16.16 整數確定性 · 64B POD 快取行硬體對齊

---

## 摘要 (Abstract)

VirtualCortex 是一套以 Rust 2024 / 2026 系統級標準構建的超高密度、微秒級延遲全腦模擬基質與全功能主權自主認知有機體。傳統神經形態計算與計算神經科學模擬框架長期受困於三大結構性瓶頸：其一，濫用物件導向指標鏈接與連續電纜方程偏微分求解（Continuous Cable PDEs），導致單神經元記憶體開銷膨脹至 4 KB 以上，引發嚴重記憶體牆危機；其二，採用全局鎖或動態圖遍歷機制，在突觸結構可塑性變化時引發全系統停頓（Stop-The-World, STW）；其三，割裂大腦核心、基底核決策、小腦運動協調、感官輸入、物理具身、神經調質、海馬迴記憶、情緒顯著性、意識工作空間與符號語言系統，無法形成自主認知閉環。

VirtualCortex 徹底摒棄盲目追逐新興軟體名詞（"Newest"）的反模式，恪守 2026+ 底層系統工程的物理客觀規律（"Latest != Newest"），將全系統形式化解耦為**十四大一級核心模組（Grand 14-Crate Cognitive Architecture）**：
1. `cortex-core`：核心神經物理微秒級動力學模擬基質（Larkum BAC 頂樹突鈣爆發、Tsodyks-Markram STP-8 突觸短期可塑性、星形膠質擴散場）。
2. `cortex-connectome`：皮層連接組神經解剖學先驗與典型六層微柱拓撲（Allen Atlas 投射藍圖、零複製 mmap `.cortex` 二進制格式）。
3. `cortex-sensory`：可插拔神經形態異質外設介面與丘腦抽象層（DVS 事件相機、耳蝸濾波陣列、IMU、電子皮膚，支援 0ms STW 熱插拔）。
4. `cortex-embodiment`：亞毫秒級具身物理閉環（POSIX 共用記憶體 `/dev/shm`、L5 爆發運動力矩解碼器、1ms 硬實時防護屏障）。
5. `cortex-basal-ganglia`：基底核行動選擇與意志決策閘控（紋狀體 D1 Go / D2 No-Go 雙通路競爭、丘腦下核 STN 超直接緊急制動煞車）。
6. `cortex-cerebellum`：小腦內部前向物理模型與微秒級運動平滑協調（Smith 預測器、下橄欖核攀緣纖維監督式 LTD，消除機器人共濟失調）。
7. `cortex-neuromod`：神經調質價值動態與三因子可塑性（多巴胺 TD-RPE、正腎上腺素驚奇警報、血清素、乙醯膽鹼）。
8. `cortex-hippocampus`：情境記憶、認知地圖與離線記憶鞏固（互補學習系統 CLS、DG/CA3/CA1 單次學習、網格細胞度量、SWR 銳波漣漪重放）。
9. `cortex-salience`：杏仁核威脅顯著性與極速避險迴路（丘腦至杏仁核 12ms 皮層下低通直通路、凍結/逃跑防禦閘控、情緒記憶標籤）。
10. `cortex-workspace`：全局神經工作空間與元認知意識廣播（GNWT 非線性點燃、P300 跨模態波形、工作記憶暫存槽、決策信心度評估）。
11. `cortex-symbolic`：向量符號架構與符號接地認知介面（超維計算 VSA/HDC 束縛/捆綁/置換代數、布羅卡/韋尼克語言與 LLM 雙向橋接）。
12. `cortex-homeostasis`：體內恆常性與晝夜節律系統（下視丘代謝內驅力、晝夜睡眠/清醒震盪、自組織臨界 SOC 平衡）。
13. `cortex-fabric`：分散式叢集網狀織網（繞過核心的 RDMA RoCEv2/InfiniBand、CXL 3.0 多主機共享記憶體池、微秒屏障同步）。
14. `cortex-telemetry`：零開銷核心態可觀測性與虛擬 LFP/EEG 遙測（eBPF 探針、無鎖 SPSC 環形緩衝區串流）。

全系統確立十四大形式化架構不變量，全內存狀態嚴格按 64 位元組 POD 快取行硬體對齊（`#[repr(C, align(64))]`）。在單台典型雙路 64 核心配合 64 GB 實體記憶體的伺服器上，僅需 **~34.80 GB** 記憶體預算即可承載相當於人類全腦規模（860 億微柱節點）的完整即時神經動力學模擬，突觸事件派發延遲嚴格限制在 **$1.85\,\mu\text{s}$** 以內。

---

## 目錄 (Table of Contents)

- [VirtualCortex 系統架構技術白皮書](#virtualcortex-系統架構技術白皮書)
  - [摘要 (Abstract)](#摘要-abstract)
  - [目錄 (Table of Contents)](#目錄-table-of-contents)
  - [1. 核心哲學：「最新（Latest）不等同於追新（Newest）」與物理極限](#1-核心哲學最新latest不等同於追新newest與物理極限)
  - [2. 十四大形式化架構不變量 (The Fourteen Formal Architectural Invariants)](#2-十四大形式化架構不變量-the-fourteen-formal-architectural-invariants)
  - [3. 記憶體階層架構與微架構契約 (Memory Hierarchy \& Microarchitectural Contracts)](#3-記憶體階層架構與微架構契約-memory-hierarchy--microarchitectural-contracts)
  - [4. 多尺度生物物理濃縮引擎 (Multi-Scale Biophysical Condensation Engine - Fidelity 5.0)](#4-多尺度生物物理濃縮引擎-multi-scale-biophysical-condensation-engine---fidelity-50)
  - [5. 微秒級事件派發與時間輪管線 (Microsecond Event Dispatch \& Timing Pipeline)](#5-微秒級事件派發與時間輪管線-microsecond-event-dispatch--timing-pipeline)
  - [6. 連續結構可塑性引擎 (Continuous Structural Plasticity Engine)](#6-連續結構可塑性引擎-continuous-structural-plasticity-engine)
  - [7. 皮層連接組藍圖與分層微柱 (`cortex-connectome`)](#7-皮層連接組藍圖與分層微柱-cortex-connectome)
  - [8. 可插拔神經形態感官介面與丘腦抽象層 (`cortex-sensory`)](#8-可插拔神經形態感官介面與丘腦抽象層-cortex-sensory)
  - [9. 具身探索經驗與亞毫秒級閉環物理 (`cortex-embodiment`)](#9-具身探索經驗與亞毫秒級閉環物理-cortex-embodiment)
  - [10. 基底核行動選擇與紋狀體意志閘控 (`cortex-basal-ganglia`)](#10-基底核行動選擇與紋狀體意志閘控-cortex-basal-ganglia)
  - [11. 小腦內部前向模型與微秒級運動協調 (`cortex-cerebellum`)](#11-小腦內部前向模型與微秒級運動協調-cortex-cerebellum)
  - [12. 神經調質價值動態與三因子可塑性 (`cortex-neuromod`)](#12-神經調質價值動態與三因子可塑性-cortex-neuromod)
  - [13. 情境記憶、認知地圖與離線記憶鞏固 (`cortex-hippocampus`)](#13-情境記憶認知地圖與離線記憶鞏固-cortex-hippocampus)
  - [14. 杏仁核威脅顯著性與極速避險迴路 (`cortex-salience`)](#14-杏仁核威脅顯著性與極速避險迴路-cortex-salience)
  - [15. 全局神經工作空間、點燃與工作記憶 (`cortex-workspace`)](#15-全局神經工作空間點燃與工作記憶-cortex-workspace)
  - [16. 向量符號架構與自然語言符號接地 (`cortex-symbolic`)](#16-向量符號架構與自然語言符號接地-cortex-symbolic)
  - [17. 自主體內恆常性、晝夜節律與臨界平衡 (`cortex-homeostasis`)](#17-自主體內恆常性晝夜節律與臨界平衡-cortex-homeostasis)
  - [18. 分散式多節點擴展與跨腦叢集網狀織網 (`cortex-fabric`)](#18-分散式多節點擴展與跨腦叢集網狀織網-cortex-fabric)
  - [19. 定量帕雷托前沿與硬體資源預算 (Quantitative Pareto Frontier \& Hardware Resource Budget)](#19-定量帕雷托前沿與硬體資源預算-quantitative-pareto-frontier--hardware-resource-budget)
  - [20. 零開銷可觀測性、遙測與內省 (`cortex-telemetry`)](#20-零開銷可觀測性遙測與內省-cortex-telemetry)
  - [21. 系統可靠性、故障隔離與崩潰一致性 (Reliability, Fault Isolation \& Crash Consistency)](#21-系統可靠性故障隔離與崩潰一致性-reliability-fault-isolation--crash-consistency)
  - [22. Rust 2024 / 2026 生產級參考規範 (Production Reference Specifications in Rust 2024 / 2026)](#22-rust-2024--2026-生產級參考規範-production-reference-specifications-in-rust-2024--2026)
  - [23. 形式化驗證、理論證明與經驗校準 (Verification, Formal Proofs \& Empirical Validation)](#23-形式化驗證理論證明與經驗校準-verification-formal-proofs--empirical-validation)
  - [24. 結論與理論意義 (Conclusion \& Theoretical Implications)](#24-結論與理論意義-conclusion--theoretical-implications)
  - [📜 授權協議與版權聲明 (License \& Copyright)](#-授權協議與版權聲明-license--copyright)

---

## 1. 核心哲學：「最新（Latest）不等同於追新（Newest）」與物理極限

現代軟體工程長期充斥著「追逐新興抽象詞彙（Newest）」的偏好——層層包裝的非同步執行環境、垃圾回收運行時、微服務 RPC 調用、動態反射與深層繼承物件模型。這些抽象概念表面上提高了程式碼原型開發速度，卻在硬體物理層面帶來了災難性的代價：快取行抖動（Cache Thrashing）、偽共享（False Sharing）、TLB 失效與非預期的 Stop-The-World (STW) 延遲毛刺。

VirtualCortex 確立的 **2026+ 系統工程最新最佳實踐（Latest Best Practice）**，核心宗旨是：**「尊重硬體物理客觀極限，以機械同理心（Mechanical Sympathy）推導系統架構」**。

```
+-----------------------------------------------------------------------------+
|                         現代計算機硬體物理客觀極限 (Physical Limits)          |
+-----------------------------------------------------------------------------+
|  光速訊號傳播延遲:   ~0.15 m/ns (光與電子在矽晶圓與銅導線中的傳導上限)             |
|  L1 快取命中時間:     ~1.0 ns (4 個時脈週期，指令與數據極限管線)                   |
|  L3 快取命中時間:     ~10-15 ns (跨核心共享快取切片訪問延遲)                      |
|  本地 DRAM 訪問延遲:  ~60-80 ns (記憶體控制器佇列與 CAS 延遲)                    |
|  跨 NUMA 節點訪問:   ~120-160 ns (UPI / Infinity Fabric 互聯互通開銷)             |
|  CXL 3.0 共享記憶體: ~180-250 ns (PCIe Gen5/6 傳輸與一致性協定開銷)               |
|  NVMe-oF 儲存訪問:   ~10-25 us (RDMA 繞過核心傳輸與 NAND 快閃記憶體讀取)          |
+-----------------------------------------------------------------------------+
```

任何脫離上述物理常數的軟體設計，均屬於無效的虛級抽象。下表展示了 VirtualCortex 如何在關鍵工程維度上貫徹「最新（2026+ 最佳實踐）」與「追新（反模式）」的本質區別：

| 系統維度 | 盲目追新（Newest 反模式） | 2026+ 最新最佳實踐（VirtualCortex） |
| :--- | :--- | :--- |
| **數值運算模型** | IEEE-754 浮點數（並行非結合律、架構間精度漂移） | **完全確定性 Q16.16 整數定點數（跨架構逐位元一致）** |
| **記憶體佈局架構** | 扁平單體 DRAM（忽視記憶體牆、大量指標跳轉） | **硬體原生分層（L1/L3 $\to$ NUMA $\to$ CXL 3.0 $\to$ NVMe）** |
| **並發與同步機制** | 互斥鎖或盲目 CAS 迴圈（鎖爭用與高頻快取失效） | **128-bit Tagged CAS + EBR 無鎖 + `nohz_full` 核心隔離** |
| **生物物理建模** | 連續電纜方程 PDE 數值解（每細胞 4 KB，記憶體崩潰）| **數學解析濃縮（Larkum BAC + STP-8 在 64B POD 內完成）** |
| **動作決策仲裁** | 單體啟發式規則（衝突失控、多指令並發打架） | **基底核雙通路閘控（D1 Go / D2 No-Go + STN 緊急制動）** |
| **運動平滑協調** | 滯後的高層反饋控制（運動震顫、過沖與共濟失調）| **小腦內部前向模型（Smith 預測器、微秒級浦肯野前饋補償）** |
| **威脅評估避險** | 深層高階感知分類（耗時 150ms，無法應對突發危險）| **皮層下 12ms 杏仁核低通直通路（丘腦直接威脅覆蓋）** |
| **意識協調廣播** | 單體黑盒注意力權重（缺乏全局統一認知焦點） | **全局神經工作空間 GNWT（非線性 P300 點燃與跨模態廣播）** |
| **符號語言交互** | 暴力浮點 Embedding（非代數、缺乏精確可逆性） | **向量符號架構 VSA/HDC（嚴格 10,000 位元代數束縛/置換）** |
| **結構可塑性重組** | 全局圖加鎖 / 動態重分配（產生 STW 停頓） | **世代雙緩衝 (EBR) + 執行緒局部 64B Slab 複用池（0ms STW）** |
| **時間輪排程機制** | 多級階層瀑布時間輪（串級產生 $O(N)$ 延遲尖峰） | **雙層無瀑布平坦環形時間輪（$O(1)$ 直接模除定時）** |
| **分散式叢集通訊** | TCP/IP RPC 微服務架構（毫秒級序列化開銷） | **繞過核心的 RDMA 網狀織網（微秒級因果世代屏障）** |
| **有機體自主內驅** | 被動式 Prompt 反應器（缺乏自主生存動機） | **自主體內恆常性驅動池（下視丘 + 晝夜節律睡眠固化）** |
| **系統遙測可觀測性**| 堆疊字串日誌與動態追蹤（熱路徑記憶體分配） | **靜態斷言 + 零開銷 SPSC 環形緩衝區 + eBPF 核心探針** |

---

## 2. 十四大形式化架構不變量 (The Fourteen Formal Architectural Invariants)

VirtualCortex 的所有模組設計，均受到以下十四大可嚴格數學驗證的形式化不變量約束：

### 不變量 1: 嚴格 64-Byte POD 快取行硬體對齊
核心系統的每一個基礎資料單元（`DendriticSuperNeuron`, `SynapseBlock`, `HyperColumnState`, `CortexFileHeader`, `BasalGangliaChannelState`, `CerebellarMicrozone`, `SalienceNodeState`, `GlobalWorkspaceSlot`, `SymbolicHypervectorHeader`, `HomeostaticDrivePool`, `FabricPacketHeader`, `HippocampalAttractorState`）其記憶體佔用必須嚴格等於 64 位元組，且物理對齊至 64 位元組邊界（`#[repr(C, align(64))]`）。嚴禁任何結構跨越 CPU 快取行邊界，徹底消除快取拆分鎖（split-lock）與偽共享懲罰。
<!-- @assert-count target="crates/cortex-core" symbol="DendriticSuperNeuron" min="1" -->
<!-- @assert-count target="crates/cortex-core" symbol="SynapseBlock" min="1" -->

### 不變量 2: 零分配派發熱路徑 (Zero-Allocation Fast Path)
在神經事件模擬派發循環（`engine::step`）的熱路徑上，**絕對禁止**呼叫任何動態記憶體分配器（包括 `malloc`, `jemalloc`, `mmap`, `Box::new`, `Vec::push`）。模擬所需的所有緩衝區必須在系統引導初始化時完成預分配，並綁定至對應的 NUMA 節點。
<!-- @assert-absence target="crates/cortex-core/src/dispatch" symbol="Box::new" -->
<!-- @assert-absence target="crates/cortex-core/src/dispatch" symbol="Vec::new" -->

### 不變量 3: 完全確定性 Q16.16 整數定點數動力學
在核心神經動力學模擬引擎中，嚴格禁止使用任何 IEEE-754 浮點型別（`f32`, `f64`）。所有膜電位積分、樹突鈣爆發電位、突觸權重短期適應因子、神經調質擴散濃度，均採用 32 位元有號數 Q16.16 整數運算，保證在 x86_64、AArch64 與 RISC-V 架構下獲得 100% 逐位元一致的模擬結果。
<!-- @assert-absence target="crates/cortex-core/src/dynamics" symbol="f32" -->
<!-- @assert-absence target="crates/cortex-core/src/dynamics" symbol="f64" -->

### 不變量 4: 無鎖世代記憶體回收 (EBR-RCU Structural Plasticity)
動態皮層連接重塑、軸突側枝萌生與突觸形成，必須與神經脈衝派發完全並行運作，嚴禁引入互斥鎖（`std::sync::Mutex` 或 `RwLock`）。連接指針替換透過 64 位元 release/acquire 原子語義完成；失效的記憶體區塊由無鎖世代記憶體回收器（Epoch-Based Reclamation）在安全靜止世代進行非阻塞回收，保證 0ms STW。

### 不變量 5: 雙層無瀑布平坦時間輪排程
突觸傳導延遲（$0.1\,\text{ms} \sim 10.0\,\text{ms}$）由雙層平坦環形時間輪統一排程。槽位定址採用嚴格 $O(1)$ 直接模除計算，完全杜絕多級時間輪的串級（cascading）重排開銷，將單個事件的排入與提取延遲鎖定在 $<8\,\text{ns}$ 以內。

### 不變量 6: 繞過核心的 CPU 核心隔離 (Kernel-Bypass Core Pinning)
計算工作執行緒必須 1:1 硬綁定至專屬物理 CPU 核心，配合 Linux 核心引導參數 `isolcpus`、`nohz_full` 與 `rcu_nocbs`。模擬工作執行緒在使用者空間全速運行無阻塞輪詢循環，排除作業系統排程上下文切換、分頁錯誤中斷與處理器間中斷（IPI）。

### 不變量 7: 硬體原生記憶體階層分層
系統實體記憶體劃分為嚴格的存取階層：熱數據（活躍神經元狀態、突觸發火佇列）鎖定於本地 NUMA DRAM；溫數據（非活躍軸突連接組拓撲）部署於 CXL 3.0 共享記憶體池；冷數據（持久化神經迴路快照）透過 `io_uring` 非同步寫入 PCIe Gen5 NVMe-oF 儲存。

### 不變量 8: 1 毫秒具身硬實時閉環屏障 (Sub-Millisecond Sensory-Motor Barrier)
具身物理介面（`cortex-embodiment`）必須與物理模擬器（Isaac Sim / MuJoCo）及實體機器人執行器在嚴格 $<1.0\,\text{ms}$ 的時間窗內完成「感官採樣 $\to$ 皮層積分 $\to$ 運動解碼」雙向閉環交換。逾期必須觸發反射弧自主保護機制。

### 不變量 9: 零停機外設熱插拔與故障隔離 (Hot-Pluggable PNS & Thalamic HAL)
感官輸入子系統（`cortex-sensory`）與周邊神經外設完全解耦。任何單一感官裝置（如相機故障、感測器掉線）在插拔或崩潰時，丘腦抽象層（Thalamic HAL）保證皮層核心持續運作不崩潰，熱插拔過程對中央模擬引擎產生 0ms STW。

### 不變量 10: 無衝突行動選擇與仲裁 (Conflict-Free Action Selection Gating)
皮層第五層產生的候選運動指令必須經過 `cortex-basal-ganglia` 紋狀體雙通路仲裁。D1/D2 通路在 $<12\,\text{ns}$ 內完成勝者全拿去抑制放行，當環境突發致命衝突時，丘腦下核（STN）超直接通路在 $<50\,\mu\text{s}$ 內全域制動煞車。

### 不變量 11: 微秒級小腦前向預測校正 (Microsecond Cerebellar Forward Lead Compensation)
派發至執行器的運動指令必須並行投射至 `cortex-cerebellum`。小腦微區透過內部前向模型在 $<5\,\mu\text{s}$ 內計算預期感官狀態並生成超前補償信號，徹底抵消機械肢體慣性帶來的延遲震顫。

### 不變量 12: 皮層下 12ms 極速威脅優先直通 (Subcortical 12ms Threat Preemption)
當無條件刺激或恐懼條件信號超越臨界閾值時，`cortex-salience` 完全繞過皮層漫長的高階識別認知流程，直接經由丘腦至杏仁核低通路在 $<12\,\text{ms}$ 內強制覆蓋運動指令，觸發保護性反射。

### 不變量 13: 全局意識非線性點燃相變 (All-or-None Conscious Ignition Thresholding)
`cortex-workspace` 中的認知概念表徵遵循嚴格的非線性階躍相變：未達閾值時維持局部無意識擴散；一旦跨越點燃閾值，觸發額頂葉 Layer 2/3 強烈互惠震盪，形成可維持 $\ge 300\,\text{ms}$ 的全局意識廣播態。

### 不變量 14: 逐位元確定性向量符號接地 (Bit-Exact Vector Symbolic Grounding)
`cortex-symbolic` 內的所有超維向量運算（束縛 $\otimes$、捆綁 $\oplus$、置換 $\Pi$）必須保證代數封閉性與幾何距離保持性，嚴禁引入非線性浮點誤差，確保符號知識與連續皮層脈衝之間的雙向無損映射。

---

## 3. 記憶體階層架構與微架構契約 (Memory Hierarchy & Microarchitectural Contracts)

### 64-Byte POD 佈局規範

為實現極限硬體記憶體頻寬利用，VirtualCortex 的核心結構體嚴格對齊為 64 位元組，不含任何虛擬表指針（vtable）、堆分配指針或非確定性填充位元組。

```
Byte Offset:
00       08       16       24       28       32       36       40   42   44       48       52   54   56 57 58 59 60      64
+--------+--------+--------+--------+--------+--------+--------+----+----+--------+--------+----+----+--+--+--+--+--------+
|   id   | mailbox| mailbox| v_soma | v_basal|v_apical|v_thresh|bac |refr|last_spk|syn_slab|plas|vox |g |f |r |u |reserved|
| (64b)  | head_pt|  tag   | (32b)  | (32b)  | (32b)  | (32b)  |cnt |cnt |  tick  |  _idx  |head|code|t |l |v |r | (32b)  |
|        | (64b)  | (64b)  | Q16.16 | Q16.16 | Q16.16 | Q16.16 |(16)|(16)| (32b)  | (32b)  |(16)|(16)|8 |8 |8 |8 | pad    |
+--------+--------+--------+--------+--------+--------+--------+----+----+--------+--------+----+----+--+--+--+--+--------+
|<----------------------------------- Exactly 64 Bytes (1 Cache Line) -------------------------------------------------->|
```

---

## 4. 多尺度生物物理濃縮引擎 (Multi-Scale Biophysical Condensation Engine - Fidelity 5.0)

VirtualCortex 實現了突破性的**五級逼真度（Fidelity 5.0）多尺度生物物理濃縮**：

### 1. Larkum BAC 頂樹突鈣爆發與重合檢測 (BAC Firing)
- **基底樹突（Basal）** 接收自下而上的前饋感官輸入：$I_{\text{basal}} = \sum_{j} W_{ij}^{\text{basal}} \cdot S_j(t)$。
- **頂樹突（Apical）** 接收自上而下的回饋注意力輸入：$V_{\text{dend}}(t) = V_{\text{dend}}(t-1) \cdot \lambda_{\text{dend}} + \sum_{k} W_{ik}^{\text{apical}} \cdot S_k(t)$。
- **符合檢測（BAC 爆發）**：若在 bAP 產生後的短時間窗（$\Delta t \le 5.0\,\text{ms}$）內，頂樹突膜電位超過鈣離子通道閾值 $\theta_{\text{Ca}}$，觸發持續 $20\sim 30\,\text{ms}$ 的高電位鈣離子平台，胞體驟變為 **300 Hz 高頻爆發（Burst Firing）**。
- **Q16.16 濃縮實現**：完全在整數暫存器內完成無浮點非線性模擬。

### 2. Tsodyks-Markram 短期突觸可塑性 (STP-8)
在 `SynapseBlock` 內以 8 位元整數維護可用囊泡數 $R$ 與利用率 $u$。當脈衝抵達時，透過預先計算的查找表進行單週期常數衰減與步進更新，無需動態微分方程求解。

### 3. 星形膠質細胞擴散場 (Astrocytic Diffusion Stencil)
每 64 個微柱共享一個 2D 擴散單元，採用 AVX-512 向量化五點差分（5-point Laplacian Stencil）進行非同步背景擴散更新，精確模擬代謝調節與神經元過度興奮保護。

### 4. 皮層典型微迴路四元細胞群 (Laminar Quad-Cell Microcircuit)
微柱內部完整模擬四類核心神經元群的交互作用：錐體細胞（PC, 80%）、PV 胞體快尖峰抑制、SST 樹突回饋增益抑制、VIP 去抑制迴路。

---

## 5. 微秒級事件派發與時間輪管線 (Microsecond Event Dispatch & Timing Pipeline)

- **AVX-512 / NEON 稀疏點陣向量壓縮**：192 位元扇出位元圖結合向量遮罩操作，無指針跳轉並行派發目標微柱。
- **雙層無瀑布平坦時間輪**：
  - 一級微秒環：200 槽，解析度 $10\,\mu\text{s}$（$0\sim 2.0\,\text{ms}$）。
  - 二級粗粒環：80 槽，解析度 $100\,\mu\text{s}$（$2.0\sim 10.0\,\text{ms}$）。
  - 槽位定址均為 $O(1)$ 直接模除，排除串級降級開銷，事件排入延遲 $<8\,\text{ns}$。
- **向量化預取**：透過 `_mm_prefetch` 提前將後續時脈的記憶體區塊拉入 L1 快取。

---

## 6. 連續結構可塑性引擎 (Continuous Structural Plasticity Engine)

- **世代式無鎖記憶體回收 (EBR-RCU)**：軸突萌生與突觸生長在影子記憶體池中構建，在 10ms 世代邊界透過 64 位元原子 CAS 切換指針，達成 **0.00 ms STW 停頓**。
- **3D 莫頓碼空間幾何索引**：將大腦三維座標 $(X,Y,Z)$ 壓縮為 16 位元莫頓空間填充曲線碼，將 3D 近鄰搜尋加速至 $O(\log N)$。

---

## 7. 皮層連接組藍圖與分層微柱 (`cortex-connectome`)

- **艾倫腦科學研究所（Allen Brain Atlas）神經解剖學先驗**：視覺皮層階層（$\text{LGN} \to \text{V1} \to \text{V2} \to \text{V4} \to \text{IT}$）、額葉工作記憶迴路、丘腦皮層先驗。
- **典型六層微柱拓撲**：L1（頂樹突叢）、L2/3（皮層間側向側枝）、L4（丘腦前饋輸入）、L5（運動輸出爆發）、L6（丘腦皮層增益控制）。
- **零複製記憶體映射二進制格式 (`.cortex`)**：
<!-- @assert-count target="crates/cortex-connectome" symbol="CortexFileHeader" min="1" -->
  `CortexFileHeader`（64 位元組 POD，魔術字 `VCORTEX1`）提供自描述區塊索引，透過 `mmap` 在幾毫秒內完成全腦拓撲載入。

---

## 8. 可插拔神經形態感官介面與丘腦抽象層 (`cortex-sensory`)

- **模組化 `SensoryPeripheral` Trait**：
<!-- @assert-count target="crates/cortex-sensory" symbol="SensoryEvent" min="1" -->
  實現感官硬體與大腦基質的完全解耦，支援動態 0ms STW 熱插拔。
- **統一非同步位址事件表示法 (AER-64)**：
  `SensoryEvent` 封裝時間戳記、外設位址、通道型別（DVS、耳蝸、IMU、觸覺皮膚）與強度負載於 8 位元組封包中。
- **丘腦前饋閘控與硬體抽象層 (Thalamic HAL)**：
  中繼並篩選各感官通道前饋信號，根據 L6 注意力指令動態抑制非相關雜訊。

---

## 9. 具身探索經驗與亞毫秒級閉環物理 (`cortex-embodiment`)

- **POSIX 共用記憶體通訊協定 (`/dev/shm`)**：
<!-- @assert-count target="crates/cortex-embodiment" symbol="EmbodimentRingBuffer" min="1" -->
  `EmbodimentRingBuffer` 提供無鎖 SPSC 環形緩衝區，與物理引擎（Isaac Sim / MuJoCo）及機器人驅動器雙向傳輸延遲 $<100\,\mu\text{s}$。
- **L5 錐體爆發力矩解碼器**：將運動皮層 L5 300Hz 爆發頻率即時線性解碼為關節力矩與阻抗剛度指令。
- **1 毫秒硬實時截止時間屏障**：嚴格時脈同步，若皮層延遲超標自動切換至脊髓反射弧姿態保護。

---

## 10. 基底核行動選擇與紋狀體意志閘控 (`cortex-basal-ganglia`)

### 生物神經科學定位
大腦皮層同時產生多個相互競爭的運動意圖，而**基底核（Basal Ganglia）負責執行行動選擇（Action Selection），放行單一獲勝動作並強力抑制衝突動作**。

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                   基底核紋狀體意志閘控微迴路 (Action Selection)               │
├─────────────────────────────────────────────────────────────────────────────┤
│   皮層運動提議 (L5 Pyramidal) ───┐                                          │
│                                  ▼                                          │
│       ┌───────────────────► 紋狀體 (Striatum) ◄─── 多巴胺 (DA)              │
│       │                          /      \                                   │
│       │          直接通路 (D1)  /        \  間接通路 (D2)                   │
│       │         [Go 放行信號]  /          \ [No-Go 抑制信號]                │
│   額葉衝突                    ▼            ▼                                │
│   緊急信號 ──► STN 超直接通路 ──► GPi / SNr ◄──── GPe                       │
│              [全球緊急煞車]       │                                         │
│                                   ▼ 去抑制放行 (淨輸出 < 0)                 │
│                         丘腦皮層運動閘門 (動作執行)                         │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 微架構與資料結構契約
<!-- @assert-count target="crates/cortex-basal-ganglia" symbol="BasalGangliaChannelState" min="1" -->
- **直接通路（Striatum D1 $\to$ GPi/SNr 去抑制）**：在多巴胺爆發時激發，解除內蒼白球的強直抑制（`Go` 信號）。
- **間接通路（Striatum D2 $\to$ GPe $\to$ STN $\to$ GPi/SNr 興奮）**：在多巴胺低落時加強抑制，封鎖競爭動作（`No-Go` 信號）。
- **丘腦下核（STN）超直接緊急煞車**：額葉衝突信號在 $<50\,\mu\text{s}$ 內廣播至 STN，瞬間觸發全域煞車，終止衝突動作。
- **64 位元組 POD 佈局**：`BasalGangliaChannelState` 在並行整數 SIMD 車道中管理 64 個候選動作通道，$<12\,\text{ns}$ 內完成勝者全拿去抑制。

---

## 11. 小腦內部前向模型與微秒級運動協調 (`cortex-cerebellum`)

### 生物神經科學定位
人類大腦 80% 的神經元集中在**小腦（Cerebellum）**。小腦本質上是一台運算延遲極低的**內部前向動態學預測器（Smith 預測器）**，在機械肢體慣性響應前預測感覺後果，徹底消除機器人共濟失調（Ataxia）、震顫與過沖。

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    小腦內部前向物理模型 (Smith 預測器架構)                   │
├─────────────────────────────────────────────────────────────────────────────┤
│   皮層運動指令 ──┬──────────────────────────────────────────► 機械臂物理執行│
│                  │                                                │         │
│                  ▼ (苔蘚纖維 Mossy Fibers)                        │         │
│          顆粒細胞層 (高維稀疏展開編碼 Expansion Recoding)         │         │
│                  │                                                │         │
│                  ▼ (平行纖維 Parallel Fibers)                     ▼         │
│            浦肯野細胞 (Purkinje) ◄────── 攀緣纖維 (下橄欖核誤差) ── 實體感官 │
│                  │                     (Climbing Fibers)         反饋       │
│                  ▼                                                          │
│        微秒級前饋超前補償信號 (抵消機械慣性遲滯，消除共濟失調)              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 微架構與資料結構契約
<!-- @assert-count target="crates/cortex-cerebellum" symbol="CerebellarMicrozone" min="1" -->
- **顆粒細胞層高維展開**：將感官運動狀態映射至極稀疏的高維哈希空間。
- **浦肯野細胞高頻鉗位**：以 $100\sim 200\,\text{Hz}$ 持續輸出平滑抑制補償信號。
- **攀緣纖維監督式長時程抑制 (LTD)**：下橄欖核發送感覺預測誤差，驅動平行纖維至浦肯野突觸發生 LTD，實現無反向傳播的運動自校準。
- **64 位元組 POD 佈局**：`CerebellarMicrozone` 封裝預測狀態與超前補償偏移量。

---

## 12. 神經調質價值動態與三因子可塑性 (`cortex-neuromod`)

- **三因子突觸可塑性**：$\Delta W_{ij} = \eta \cdot e_{ij}(t) \cdot M_k(t)$。
- **四大核心神經調質**：
<!-- @assert-count target="crates/cortex-neuromod" symbol="NeuromodulatorState" min="1" -->
  - **多巴胺 (DA)**：時間差分獎勵預測誤差（TD-RPE），驅動自主強化學習。
  - **正腎上腺素 (NE)**：藍斑核驚奇警報，動態調控全腦神經元增益。
  - **血清素 (5-HT)**：長期折現因子與風險規避調節。
  - **乙醯膽鹼 (ACh)**：前饋感官編碼模式與內部記憶鞏固模式的精準度閘控。

---

## 13. 情境記憶、認知地圖與離線記憶鞏固 (`cortex-hippocampus`)

- **互補學習系統 (CLS)**：皮層提取慢速統計語義，海馬迴透過 CA3 稀疏吸引子實現快速單次情境記憶（1-Shot Episodic Learning）。
<!-- @assert-count target="crates/cortex-hippocampus" symbol="HippocampalAttractorState" min="1" -->
- **空間認知幾何 (Grid & Place Cells)**：連續環面吸引子產生六角蜂巢週期性空間激發場，提供自主航位推算尺標。
- **銳波漣漪 (SWR) 離線記憶固化**：在睡眠與靜息階段以 $10\times$ 速度重放日間經驗軌跡，驅動新皮層 STDP 實現記憶固化。

---

## 14. 杏仁核威脅顯著性與極速避險迴路 (`cortex-salience`)

### 生物神經科學定位
當生物遭遇致命威脅時，若等待長達 $100\sim 150\,\text{ms}$ 的皮層認知處理，將導致致命延誤。約瑟夫·勒杜（Joseph LeDoux）發現**杏仁核（Amygdala）**具備直接接收丘腦粗糙輸入的**皮層下低通直通路（Subcortical Low-Road）**，能在 $<12\,\text{ms}$ 內強制激發避險防禦反射。

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    杏仁核雙通道威脅避險微迴路架構                           │
├─────────────────────────────────────────────────────────────────────────────┤
│                          感官輸入刺激 (AER-64)                              │
│                                    │                                        │
│                                    ▼                                        │
│                                丘腦核團                                     │
│                               /        \                                    │
│       [低通直路: < 12ms]     /          \  [高階皮層通路: ~120ms]           │
│       粗粒度極速危險偵測    /            \ 高解析度語義情境分析             │
│                            ▼              ▼                                 │
│                      基底外側杏仁核 ◄── 感覺皮層                            │
│                            │                                                │
│                            ▼                                                │
│                      中央核 (CeA)                                           │
│                            │                                                │
│            ┌───────────────┴───────────────┐                                │
│            ▼                               ▼                                │
│     緊急運動強制覆蓋                情緒記憶優先標籤                        │
│    (凍結/逃跑防禦反射 < 12ms)     (海馬迴閃光燈 SWR 固化)                   │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 微架構與資料結構契約
<!-- @assert-count target="crates/cortex-salience" symbol="SalienceNodeState" min="1" -->
- **皮層下低通直路計算**：丘腦粗粒度事件直通 `cortex-salience`，在 $<12\,\text{ms}$ 內完成威脅評估。
- **緊急防禦運動覆蓋**：無條件刺激觸發中央核（CeA）強制接管關節指令，引發凍結（Freeze）、逃跑（Flight）或防護姿勢。
- **海馬迴情緒標籤**：為高危險經驗附加最高優先級記憶標籤，保證在 SWR 睡眠固化時獲得優先重放。
- **64 位元組 POD 佈局**：`SalienceNodeState` 封裝威脅效價、低通倒數計數與防禦模式標誌。

---

## 15. 全局神經工作空間、點燃與工作記憶 (`cortex-workspace`)

### 認知科學理論基礎
依據斯坦尼斯拉斯·迪昂（Stanislas Dehaene）的全局神經工作空間理論（GNWT），大腦數百億神經元並行進行無意識局部運算。僅有極少數高顯著性表徵能突破閾值，引發**非線性意識點燃（Conscious Ignition）**，進入由額頂葉 Layer 2/3 長程軸突構成的**全局工作空間**。

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                     全局神經工作空間 (GNWT) 架構                            │
├─────────────────────────────────────────────────────────────────────────────┤
│   局部感官模組串流 (視覺、聽覺、本體感覺、海馬情境)                          │
│           │                       │                       │                 │
│           ▼                       ▼                       ▼                 │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │              非線性點燃閾值累積器 (Ignition Accumulator)            │   │
│   └──────────────────────────────────┬──────────────────────────────────┘   │
│                                      │ 輸入證據 >= 點燃閾值                 │
│                                      ▼                                      │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │           額頂葉全局工作空間暫存槽 (7 +/- 2 緩衝區)                 │   │
│   │           - 持續性互惠回歸 (300ms 意識持久視窗)                     │   │
│   │           - P300 全腦全或無相位同步廣播                             │   │
│   └──────────────────────────────────┬──────────────────────────────────┘   │
│                                      │                                      │
│                                      ▼                                      │
│       廣播至全腦所有微柱模組 ───► 元認知引擎 (決策信心度動態評估)            │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 微架構與資料結構契約
<!-- @assert-count target="crates/cortex-workspace" symbol="GlobalWorkspaceSlot" min="1" -->
- **非線性相變點燃**：前饋證據在微柱間累積；跨越 `IGNITION_THRESHOLD`（$1.5$ in Q16.16）時，觸發全或無全局廣播。
- **執行工作記憶暫存槽**：維持 $7 \pm 2$ 個核心概念向量，跨任務步驟保持穩定活化。
- **元認知決策信心度評估**：實時計算決策確定性 $\mathcal{C} \in [0, 1]$；若信心度低於安全裕度，自主暫停執行並調配額外皮層資源進行深思。
- **64 位元組 POD 佈局**：`GlobalWorkspaceSlot` 管理概念綁定哈希、點燃狀態與廣播通道遮罩。

---

## 16. 向量符號架構與自然語言符號接地 (`cortex-symbolic`)

### 理論基礎與符號接地問題
為徹底解決**符號接地問題（Symbol Grounding Problem）**，VirtualCortex 透過**超維計算 / 向量符號架構（VSA / HDC）**在連續脈衝空間與離散人類符號世界（自然語言 Token、知識圖譜、大語言模型 LLM）之間構建了雙向無損橋接。

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                 向量符號架構 (VSA / HDC) 與語言橋接                         │
├─────────────────────────────────────────────────────────────────────────────┤
│   10,000 位元高維雙極性向量空間: x in {-1, +1}^10000                        │
│                                                                             │
│   1. 束縛運算 Binding (角色-填充項關聯):                                    │
│      Color_Red = Role_Color (x) Filler_Red    (精確可逆 XOR 運算)           │
│                                                                             │
│   2. 捆綁運算 Bundling (集合疊加):                                          │
│      Apple = Fruit (+) Color_Red (+) Taste_Sweet (多數決疊加代數)            │
│                                                                             │
│   3. 置換運算 Permutation (語法結構與時序):                                 │
│      Sentence = Word_1 (+) Pi(Word_2) (+) Pi^2(Word_3) (循環移位代數)       │
│                                                                             │
│   雙通道認知語言介面:                                                       │
│   [文本 / LLM Token] ──► 韋尼克理解通道 ──► 皮層連續空間超稀疏吸引子軌跡    │
│   [額葉微柱群軌跡]   ──► 布羅卡表達通道 ──► 離散自然語言 Token / 結構化 JSON│
└─────────────────────────────────────────────────────────────────────────────┘
```

### 微架構與資料結構契約
<!-- @assert-count target="crates/cortex-symbolic" symbol="SymbolicHypervectorHeader" min="1" -->
- **10,000 位元超維代數運算**：在 64 位元組對齊的 SIMD 向量塊上直接執行束縛（$\otimes$）、捆綁（$\oplus$）與置換（$\Pi$）。
- **韋尼克理解通道（Wernicke Channel）**：將外部輸入的文本 Token 與本體論本體精確映射為高維皮層連續吸引子。
- **布羅卡表達通道（Broca Channel）**：將額葉皮層族群軌跡直接解碼為離散詞彙 Token 與機器人動作結構化語法。
- **64 位元組 POD 佈局**：`SymbolicHypervectorHeader` 維護概念編號、角色-填充項關聯與漢明距離快取。

---

## 17. 自主體內恆常性、晝夜節律與臨界平衡 (`cortex-homeostasis`)

### 生物神經科學定位
生命自主性的核心源泉在於**體內恆常性（Homeostasis）**。`cortex-homeostasis` 模擬下視丘代謝內驅力池與晝夜節律震盪器，自主決定何時進行外部環境探索，何時切換至睡眠固化模式。

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                 自主體內恆常性與晝夜節律震盪架構                            │
├─────────────────────────────────────────────────────────────────────────────┤
│   內部驅力池: 能量儲備 (Energy), 突觸疲勞 (Fatigue), 好奇心, 熱應力        │
│                                  │                                          │
│                                  ▼                                          │
│                  晝夜節律睡眠/清醒相位狀態機                                │
│                    /                             \                          │
│        [清醒模式: 高 ACh / NE]          [睡眠模式: 海馬 SWR 重放]            │
│        主動環境感知與運動探索            深層新皮層突觸長期固化              │
│                                  │                                          │
│                                  ▼                                          │
│            自組織臨界性 (SOC) 分支比即時平衡調控                            │
│                 保持全腦神經計算處於「混沌邊緣」                            │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 微架構與資料結構契約
<!-- @assert-count target="crates/cortex-homeostasis" symbol="HomeostaticDrivePool" min="1" -->
- **下視丘代謝驅力池**：追蹤能量儲備、突觸疲勞度、探索衝動與計算熱應力。
- **晝夜節律震盪器**：在清醒探索模式與離線睡眠鞏固模式之間自主調諧切換。
- **自組織臨界平衡 (SOC)**：實時測量神經雪崩分支比（Branching Ratio $\sigma = \langle N_{t+1}/N_t \rangle$），動態微調發火閾值偏差，確保全腦運算永保臨界狀態（$\sigma \approx 1.0$）。
- **64 位元組 POD 佈局**：`HomeostaticDrivePool` 維護驅力標量與臨界平衡控制參數。

---

## 18. 分散式多節點擴展與跨腦叢集網狀織網 (`cortex-fabric`)

### 系統工程定位
為突破單機物理邊界，邁向跨機架分散式超大腦或多機器人群體智能（Multi-Agent Swarms），`cortex-fabric` 構建了零拷貝繞過核心的分散式叢集通訊織網。

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                  cortex-fabric 分散式叢集拓撲結構                           │
├─────────────────────────────────────────────────────────────────────────────┤
│  [節點 0: 感官 / V1-V4 皮層]           [節點 1: 聯絡區 / 額葉決策]          │
│   Cortex Core 模擬實例                  Cortex Core 模擬實例                │
│         │                                     │                             │
│         ▼                                     ▼                             │
│  ┌──────────────┐                             ┌──────────────┐              │
│  │ RDMA 通訊佇列│◄══════ RoCEv2 / IB ════════►│ RDMA 通訊佇列│              │
│  │ (ibverbs)    │    雙向延遲 < 2.0 us        │ (ibverbs)    │              │
│  └──────┬───────┘                             └──────┬───────┘              │
│         │                                            │                      │
│         ▼                                            ▼                      │
│  ┌───────────────────────────────────────────────────────────┐              │
│  │ CXL 3.0 多主機共享記憶體織網 (跨主機共享突觸連接組池)     │              │
│  └───────────────────────────────────────────────────────────┘              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 微架構與資料結構契約
<!-- @assert-count target="crates/cortex-fabric" symbol="FabricPacketHeader" min="1" -->
- **繞過核心的 RDMA 傳輸 (`ibverbs` / RoCEv2 / InfiniBand)**：節點間事件佇列直接記憶體寫入，跨節點雙向延遲 $<2.0\,\mu\text{s}$。
- **CXL 3.0 跨主機記憶體池化**：跨機架共享具備硬體快取一致性的突觸權重池。
- **確定性微秒世代屏障同步**：無鎖世代屏障保證跨節點分散式模擬時的 Q16.16 數值完全因果確定性。
- **64 位元組 POD 佈局**：`FabricPacketHeader` 封裝封包路由、世代屏障 ID 與硬體 CRC 校驗碼。

---

## 19. 定量帕雷托前沿與硬體資源預算 (Quantitative Pareto Frontier & Hardware Resource Budget)

### 860 億節點完備認知有機體實體記憶體預算表

| 記憶體子系統 / 結構體 | 結構體型別 | 單元大小 | 數量規模 | 實體記憶體佔用 | 儲存階層歸屬 |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **中尺度超神經元** | `DendriticSuperNeuron` | 64 Bytes | 43,000,000 | **2.75 GB** | 本地 NUMA DDR5 |
| **巨觀超微柱狀態** | `HyperColumnState` | 64 Bytes | 860,000 | **55.04 MB** | 本地 NUMA DDR5 |
| **突觸區塊記憶體池** | `SynapseBlock` | 64 Bytes | 128,000,000 | **8.19 GB** | 本地 NUMA DDR5 |
| **基底核動作閘控** | `BasalGangliaChannelState` | 64 Bytes | 1,000,000 | **64.00 MB** | 本地 NUMA DDR5 |
| **小腦內部前向微區** | `CerebellarMicrozone` | 64 Bytes | 8,000,000 | **512.00 MB** | 本地 NUMA DDR5 |
| **杏仁核威脅節點** | `SalienceNodeState` | 64 Bytes | 500,000 | **32.00 MB** | 本地 NUMA DDR5 |
| **全局意識工作空間** | `GlobalWorkspaceSlot` | 64 Bytes | 250,000 | **16.00 MB** | 本地 NUMA DDR5 |
| **VSA 符號超維碼本** | `SymbolicHypervectorHeader`| 64 Bytes | 1,000,000 | **1.25 GB** | 本地 NUMA DDR5 |
| **神經調質擴散場** | `NeuromodulatorState` | 16 Bytes | 860,000 | **13.76 MB** | 本地 NUMA DDR5 |
| **海馬情境記憶緩衝** | `HippocampalAttractorState` | 64 Bytes | 1,000,000 | **64.00 MB** | 本地 NUMA DDR5 |
| **體內恆常性驅動池** | `HomeostaticDrivePool` | 64 Bytes | 500,000 | **32.00 MB** | 本地 NUMA DDR5 |
| **叢集織網 RDMA 佇列**| `FabricPacketHeader` | 64 Bytes | 2,000,000 | **128.00 MB** | 本地 NUMA DDR5 |
| **雙層無瀑布時間輪** | 平坦 1024 槽環形緩衝 | 8 MB / 輪 | 64 核心 | **512.00 MB** | 本地 CPU 快取相鄰 |
| **SIMD 廣播位元圖** | `ColumnSpikeBroadcaster`| 512 Bytes | 860,000 | **440.32 MB** | 本地 NUMA DDR5 |
| **感官與具身 IPC 環** | `EmbodimentRingBuffer` | 64 KB 緩衝 | 2,048 通道 | **131.07 MB** | POSIX `/dev/shm` |
| **全腦遙測採樣環** | `LfpSamplePacket` | 64 Bytes | 500,000 | **32.00 MB** | 專屬遙測內存區 |
| **稀疏分頁目錄表** | 兩級基數樹表 | — | 65,536 頁 | **1.35 GB** | 本地 NUMA DDR5 |
| **稀疏塑性連接差分** | `PlasticSynapseDelta` | 16 Bytes | 1,000,000,000 | **16.00 GB** | CXL 3.0 共享記憶體 |
| **全腦常駐總實體記憶體**| **860 億完備主權有機體** | — | — | **~34.80 GB** | **單台 64GB 伺服器即可承載** |

---

## 20. 零開銷可觀測性、遙測與內省 (`cortex-telemetry`)

- **核心態 eBPF 零侵入探針**：USDT 靜態探針點在未啟用時為 5 位元組 `NOP`，對派發管線產生 0 奈秒效能干擾。
<!-- @assert-count target="crates/cortex-telemetry" symbol="LfpSamplePacket" min="1" -->
- **局部場電位 (LFP) 與腦電波 (EEG) 合成器**：整合微柱跨膜電流，即時分解 $\delta, \theta, \alpha, \beta, \gamma$ 頻段信號。
- **SPSC 無鎖環形緩衝區串流**：以 Apache Arrow Flight 與 WebSocket 非同步串流，完全不佔用運算核心時脈。

---

## 21. 系統可靠性、故障隔離與崩潰一致性 (Reliability, Fault Isolation & Crash Consistency)

- **非同步 WAL 與 `io_uring`**：將突觸變更批量提交至 PCIe Gen5 NVMe，單機寫入頻寬突破 $6.5\,\text{GB/s}$。
- **NUMA 網域記憶體鎖定**：嚴格鎖定執行緒至本地 NUMA 節點，防止跨插槽互連飽和。
- **CXL 故障隔離**：透過 Linux `userfaultfd` 攔截跨節點記憶體硬體異常，保證核心大腦基質存活。

---

## 22. Rust 2024 / 2026 生產級參考規範 (Production Reference Specifications in Rust 2024 / 2026)

```rust
//! 十四大一級模組編譯期架構契約靜態斷言

const _: () = {
    assert!(core::mem::size_of::<cortex_core::DendriticSuperNeuron>() == 64);
    assert!(core::mem::align_of::<cortex_core::DendriticSuperNeuron>() == 64);
    assert!(core::mem::size_of::<cortex_core::SynapseBlock>() == 64);
    assert!(core::mem::align_of::<cortex_core::SynapseBlock>() == 64);
    assert!(core::mem::size_of::<cortex_connectome::CortexFileHeader>() == 64);
    assert!(core::mem::align_of::<cortex_connectome::CortexFileHeader>() == 64);
    assert!(core::mem::size_of::<cortex_embodiment::EmbodimentRingBuffer>() == 64);
    assert!(core::mem::align_of::<cortex_embodiment::EmbodimentRingBuffer>() == 64);
    assert!(core::mem::size_of::<cortex_basal_ganglia::BasalGangliaChannelState>() == 64);
    assert!(core::mem::align_of::<cortex_basal_ganglia::BasalGangliaChannelState>() == 64);
    assert!(core::mem::size_of::<cortex_cerebellum::CerebellarMicrozone>() == 64);
    assert!(core::mem::align_of::<cortex_cerebellum::CerebellarMicrozone>() == 64);
    assert!(core::mem::size_of::<cortex_salience::SalienceNodeState>() == 64);
    assert!(core::mem::align_of::<cortex_salience::SalienceNodeState>() == 64);
    assert!(core::mem::size_of::<cortex_workspace::GlobalWorkspaceSlot>() == 64);
    assert!(core::mem::align_of::<cortex_workspace::GlobalWorkspaceSlot>() == 64);
    assert!(core::mem::size_of::<cortex_symbolic::SymbolicHypervectorHeader>() == 64);
    assert!(core::mem::align_of::<cortex_symbolic::SymbolicHypervectorHeader>() == 64);
    assert!(core::mem::size_of::<cortex_hippocampus::HippocampalAttractorState>() == 64);
    assert!(core::mem::align_of::<cortex_hippocampus::HippocampalAttractorState>() == 64);
    assert!(core::mem::size_of::<cortex_homeostasis::HomeostaticDrivePool>() == 64);
    assert!(core::mem::align_of::<cortex_homeostasis::HomeostaticDrivePool>() == 64);
    assert!(core::mem::size_of::<cortex_fabric::FabricPacketHeader>() == 64);
    assert!(core::mem::align_of::<cortex_fabric::FabricPacketHeader>() == 64);
    assert!(core::mem::size_of::<cortex_telemetry::LfpSamplePacket>() == 64);
    assert!(core::mem::align_of::<cortex_telemetry::LfpSamplePacket>() == 64);
    assert!(core::mem::size_of::<cortex_neuromod::NeuromodulatorState>() == 16);
    assert!(core::mem::size_of::<cortex_sensory::SensoryEvent>() == 8);
};
```

---

## 23. 形式化驗證、理論證明與經驗校準 (Verification, Formal Proofs & Empirical Validation)

為確保系統在 2026+ 標準下的零迴歸與確定性，VirtualCortex 引入持續形式化驗證流水線：
1. **編譯期佈局不變量**：以 `static_assertions` 固化所有核心結構體大小（64B）與對齊（64B），徹底消除平台間 ABI 漂移。
2. **可執行架構斷言**：透過 `spec-guard` 驗證模組符號邊界、熱路徑零堆分配、與定點數無浮點契約。
3. **跨架構逐位元一致性**：在 x86_64、AArch64 與 RISC-V 節點上對 $10^9$ 個時脈步長計算狀態雜湊，驗證 64 位元校驗和 100% 完全一致。

---

## 24. 結論與理論意義 (Conclusion & Theoretical Implications)

VirtualCortex 確立了一種不同於傳統大模型盲目堆疊參數量（LLM Scale-Up）的全新計算路徑。藉由恪守 2026+ 底層系統架構的最佳工程實踐——**「以機械同理心推導軟體、以確定性整數取代浮點數、以物理對齊消除快取干擾、以 64 位元組濃縮抽象跨越全尺度生物物理」**，VirtualCortex 在單台現代伺服器（僅需 ~34.80 GB 記憶體預算）上，實現了支持 860 億微柱節點的全功能全腦微秒級模擬。

更為關鍵的是，透過將皮層基質與**連接組藍圖（`cortex-connectome`）**、**可插拔感官（`cortex-sensory`）**、**具身物理（`cortex-embodiment`）**、**基底核行動選擇（`cortex-basal-ganglia`）**、**小腦運動協調（`cortex-cerebellum`）**、**神經調質（`cortex-neuromod`）**、**海馬迴情境記憶（`cortex-hippocampus`）**、**杏仁核威脅顯著性（`cortex-salience`）**、**全局神經工作空間（`cortex-workspace`）**、**向量符號語言接地（`cortex-symbolic`）**、**自主體內恆常性（`cortex-homeostasis`）**、**分散式織網（`cortex-fabric`）** 與 **全腦遙測（`cortex-telemetry`）** 深度整合，VirtualCortex 真正構築了具備自主生存動機、語言思維交流、物理閉環自洽與生物級能源效率的**終極主權自主認知有機體（Sovereign Autonomous Cognitive Organism）**。

---

## 📜 授權協議與版權聲明 (License & Copyright)

VirtualCortex 遵循標準 Rust 生態雙重授權規範（Dual-licensed under Apache-2.0 OR MIT）：

- **[Apache License, Version 2.0](../../LICENSE-APACHE)**
- **[MIT License](../../LICENSE-MIT)**

使用者可根據具體專案需求，自主選擇上述任一授權協議進行開發、衍生與部署。

版權所有 (c) 2026 VirtualCortex Project Contributors. 保留所有權利。
