# Menteion 架構白皮書導讀（繁體中文）

> **本文件是導讀，不是規格。** 唯一具規範效力的文件是英文原版 [`docs/WHITEPAPER.md`](../WHITEPAPER.md)。本導讀不重複任何結構體佈局、欄位偏移或效能數字；凡涉及這些內容，一律以英文原版為準。這樣做的原因記錄在 [ADR-0008](../adr/0008-documentation-governance.md)：舊版曾維護一份完整的平行譯本，結果兩份文件同步漂移、同步損毀，翻譯反而成了第二份互相矛盾的「合約」。

**專案現在走到哪裡**，請看 [`docs/STATUS.md`](../STATUS.md)（英文，每一輪都會整份改寫）。本導讀不記錄進度（[ADR-0197](../adr/0197-one-place-for-each-kind-of-fact.md)）。

## 這個專案是什麼

Menteion 是一個以 Rust 撰寫的**單機神經擬態虛擬 Actor 引擎**：把脈衝神經網路（SNN）建立在虛擬 Actor 模型之上，並嚴格限制在一台實體伺服器內執行。設計前提是：生物神經組織極度稀疏、事件驅動、非同步，而通用 CPU 只有在每一個資料結構都尊重硬體物理現實（64 位元組快取行、記憶體牆、分支預測器、堆配置與系統呼叫的代價）時，才能有效率地執行這樣的系統。

專案的工程信條是 **「Latest ≠ Newest」**：最新的技術很少是最被理解的技術。任何進入熱路徑的技術都必須具備穩定的規格、至少兩年的第三方生產使用紀錄、已知而非待發現的失效模式，以及能對應到 CPU 與核心實際行為的機械同理心論證（英文原版 §2.1）。

## 讀白皮書之前必須知道的四個狀態標籤

白皮書中每一個關於系統的敘述都帶有下列四種標籤之一，**沒有標籤的敘述視為缺陷**：

| 標籤 | 意義 |
| :--- | :--- |
| **Implemented（已實作）** | 今天就存在於 `crates/` 或 `runtime/`，且由編譯器、單元測試或文件內的可執行斷言檢查。 |
| **Specified（已規範）** | 設計已在白皮書或 ADR 中定案（介面、格式、不變量），但尚無程式碼。 |
| **Target（目標）** | 可量測的品質目標，附有量測方法；**尚未量測**。 |
| **Hypothesis（假設）** | 架構所依賴的研究假設；驗證之前，依賴它的任何敘述都不能升級為 Target。 |

兩條配套原則：

1. **文件與程式碼不一致時，以程式碼為準**，並把不一致記錄為 §11 的編號 finding，不得默默修掉。
2. **白皮書中沒有任何效能數字是量測過的。** §10 的每個數字都是附帶量測協定的 Target；舊版宣稱的「P99.99 < 35 ns」、「86 億節點 100 ms 冷啟動」等已明確撤回（見 [ADR-0010](../adr/0010-measured-or-target.md)）。

## 五大公理（設計骨幹）

白皮書 §4 重新以專案最初設計稿的五大公理為核心，32 個 crate 都建立在它們之上（其中 14 個由 [ADR-0016](../adr/0016-thirty-two-crate-architecture.md) 於 2026-09-10 納入）：

| 公理 | 內容 |
| :--- | :--- |
| A1 虛擬存在 | 神經單元在邏輯上永遠存在，只有脈衝命中時才佔用記憶體；冷單元被淘汰，需要時再惰性載入。 |
| A2 狀態與算力解耦 | 記錄是被動資料，Worker 是無狀態、綁核的執行緒；資源消耗與瞬時活躍度成正比，與總容量無關。 |
| A3 回合制單寫者 | 任一 tick 內，一個記錄的一般欄位最多只有一個 Worker 寫入，由所有權保證：每個 Worker 固定負責一段單元，只有它會跑這些單元的回合；其他 Worker 對同一筆記錄的存取（把訊息推進信箱、標記閘位元組、讀脈衝時戳）都被屏障隔在擁有者寫入的那個階段之外（ADR-0102，原本由 CAS 閘控保證）；無鎖、無死鎖、無資料競爭。 |
| A4 離散軸突延遲 | 時間是 tick，延遲是時間輪的索引，永不使用作業系統計時器；排程為 O(1)。 |
| A5 代謝分層 | 背景掃描把長期沉寂的組織淘汰到本地儲存，熱迴路留在快取友善的陣列中。 |

## 白皮書結構對照（arc42 範本）

| 英文原版章節 | 內容 |
| :--- | :--- |
| [Executive summary](../WHITEPAPER.md#executive-summary) | 今天有什麼、設計了什麼、必須證明什麼。 |
| [§1 Introduction and goals](../WHITEPAPER.md#1-introduction-and-goals) | 問題陳述、功能需求、品質目標、非目標、**32 個 crate 的實作現況表**。 |
| [§2 Constraints](../WHITEPAPER.md#2-constraints) | 「Latest ≠ Newest」信條的可操作定義、技術限制（TC-1 至 TC-9）、書寫慣例。 |
| [§3 Context and scope](../WHITEPAPER.md#3-context-and-scope) | C4 第一層系統情境圖與對外介面。 |
| [§4 Solution strategy](../WHITEPAPER.md#4-solution-strategy) | 五大公理、品質目標的達成策略、分解原則。 |
| [§5 Building block view](../WHITEPAPER.md#5-building-block-view) | 分層圖，以及**每個 crate 的責任、公開 API、逐位元組的記錄佈局與狀態**——這是 ABI 合約所在。 |
| [§6 Runtime view](../WHITEPAPER.md#6-runtime-view) | 執行期情境 R-1 至 R-16：單一脈衝的生命週期、時間輪 tick、感官熱插拔、具身迴圈、動作選擇（R-5：作業、讀出與獎賞怎麼組起來）、睡眠與重播、冷開機、工具呼叫、語言、數學、文件閱讀與稽核，以及政策修正的試行。各輪量測的經過不寫在這一節，寫在 [`docs/research/`](../research/README.md)。 |
| [§7 Deployment view](../WHITEPAPER.md#7-deployment-view) | 參考平台（Target）、記憶體階層、行程與執行緒模型。 |
| [§8 Cross-cutting concepts](../WHITEPAPER.md#8-cross-cutting-concepts) | Q16.16 數值模型、記錄佈局規則 L-1 至 L-6、確定性、時間、並行、記憶體、序列化、**生物機制對照表與參考方程式**（§8.8）、錯誤處理、安全、可觀測性，以及 §8.12 至 §8.18：計算現象學、原生語言、社交分寸、再表徵、聲學合成、計算幽默與自我修正。 |
| [§9 Architecture decisions](../WHITEPAPER.md#9-architecture-decisions) | ADR 索引；決策本體在 [`docs/adr/`](../adr/README.md)。 |
| [§10 Quality requirements](../WHITEPAPER.md#10-quality-requirements) | 品質樹與 T-1 至 T-8 目標情境，每列都有量測協定與空白的「Measured」欄。 |
| [§11 Risks and technical debt](../WHITEPAPER.md#11-risks-and-technical-debt) | 編號 finding（F-n）：文件與程式碼不一致的地方、已知的錯誤，以及各自怎麼處置。§11.1 是假設總表，每個假設（H-1 至 H-45）一列，另列尚未編號的開放問題；每個假設的原文、停止規則與讀值在 [`docs/research/`](../research/README.md)。 |
| [§12 Glossary](../WHITEPAPER.md#12-glossary) | 術語表。 |
| [附錄 A](../WHITEPAPER.md#appendix-a-capacity-model) | 容量模型（是計畫，不是量測）。這張表是離線模式的計畫：一台伺服器裝得下多少，而不是能即時跑多少（ADR-0098）。ADR-0097 量出每個 tick 要服務的單元比例，並寫下單元數、即時 tick 與輸入密度三個目標必須一起滿足的預算；在 ADR-0044 的輸入密度下，離線跑這份計畫比即時慢約 1 700 倍，接上身體的即時模式則只能跑預算允許的單元數，約 2.6 萬個（Target）。表格本身沒改，F-49 已由這個決定解決。 |
| [附錄 B](../WHITEPAPER.md#appendix-b-verification-and-conformance) | 七層驗證 V-1 至 V-7（編譯期佈局斷言、spec-guard 文件對程式碼、spec-graph 文件對文件、brief 章節、manifest 依賴、PR 改動行的 mutation gate、兩種架構上的判定 pin）加上建置閘門（release profile 測試、rustdoc、MSRV、benchmark 冒煙）。 |
| [附錄 C](../WHITEPAPER.md#appendix-c-roadmap) | 里程碑 M1 至 M15，每個都有退出測試。 |
| [附錄 D](../WHITEPAPER.md#appendix-d-references) | 參考文獻。 |

## 如何在本機驗證文件與程式碼一致

```bash
cargo check --workspace --all-targets
cargo test --workspace
npm ci
npm run spec
```

`npm run spec` 會執行這些檢查：[`@spec-family/spec-guard`](https://www.npmjs.com/package/@spec-family/spec-guard) 執行白皮書與 README 中嵌入的 `<!-- @assert-* -->` 斷言，確認 `crates/` 仍然含有文件宣稱的型別、且不含被禁止的東西（`f32`、`f64`、`unsafe`、`Box<`、`Vec<`…）；[`@spec-family/spec-graph`](https://www.npmjs.com/package/@spec-family/spec-graph) 檢查所有 Markdown 文件之間的連結、ADR 狀態與開放問題是否一致。[`@spec-family/spec-brief`](https://www.npmjs.com/package/@spec-family/spec-brief) 依 `.spec-brief.json` 檢查每份 live brief 的必要章節，並確認已歸檔的 brief 自歸檔後沒被改過（ADR-0185）。三者都以精確版本釘在 `package.json`；倉庫內另有 `scripts/check-archive.mjs`（每份已歸檔的 brief 都帶有雜湊）、`scripts/check-deps.mjs`（state crate 不宣告任何依賴，TC-2）、`scripts/check-doc-shape.mjs`（新增或改動的文字不超過長度上限，已接受的 ADR 本文一字不動）與 `scripts/adr-index.mjs`（ADR 索引、§9 與假設總表都由 front matter 產生）等腳本。這些都在 CI 中作為阻斷性檢查執行。

## 如何貢獻

流程、提交訊息慣例、文件治理規則與「完成的定義」在 [`CONTRIBUTING.md`](../../CONTRIBUTING.md)；安全性回報在 [`SECURITY.md`](../../SECURITY.md)；變更紀錄在 [`CHANGELOG.md`](../../CHANGELOG.md)。給編碼代理的總機是 [`CLAUDE.md`](../../CLAUDE.md)。下一輪工作以編號的自足 prompt（brief）形式放在 [`briefs/`](../../briefs/README.md)，執行後凍結封存到 `briefs/archive/`；brief 是輸入，不是紀錄。

## 授權

Apache-2.0 OR MIT，由使用者自行選擇。
