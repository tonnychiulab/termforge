# TermForge 原始碼安全初測（公開摘要）

| 欄位 | 內容 |
|---|---|
| 文件名稱 | TermForge 原始碼初測報告（公開摘要） |
| 專案 | [tonnychiulab/termforge](https://github.com/tonnychiulab/termforge) |
| 審查對象 | `main` commit `0dc0dba5fffbbf5b4475e635c1004e1dabf00429` |
| 審查範圍 | Rust crates、Asteroids 遊戲、WASM／前端、GitHub Actions、Cargo.lock |
| 文件性質 | 開源專案公開安全審查摘要 |
| 審查日期時間 | 2026-09-17 16:45（Asia/Taipei） |
| 公開稿日期 | 2026-09-17 16:55（Asia/Taipei） |
| 審查模型 | Cursor Grok 4.6 |
| 版本 | 公開稿 v0.1 |

追蹤討論：[#2](https://github.com/tonnychiulab/termforge/issues/2)。收口紀錄見當日工作日誌第六、七節。

**後續處理：** TF-01 已在 PR #3 對齊發布路徑。TF-02／TF-03 已在 PR #4 修復並由 Actions 寫入 `gh-pages`。xterm.js 自託管於 `dist/vendor/`；GitHub Pages 不會套用 `_headers`，改以 CSP／referrer meta 與 `framebust.js`（平台仍無法送出 `X-Frame-Options`）。發現項已全部處理；剩餘項目為工程強化，不是未修漏洞。

## 摘要

TermForge 是本機終端機遊戲引擎與靜態 WASM 示範，**沒有帳號、後端或機密資料流**。原生 Rust 路徑未發現可利用的記憶體安全或命令注入問題；第一方程式沒有 `unsafe`，也沒有 shell／檔案／環境變數解析。

公開試玩頁目前可以開啟。審查當時，倉庫裡宣告的 GitHub Actions 自動部署並沒有成為實際的 GitHub Pages 發布路徑；此項為當時唯一的**中**風險。

| 嚴重度 | 數量 |
|---|---|
| 嚴重 | 0 |
| 高 | 0 |
| 中 | 1 |
| 低 | 2 |

| 面向 | 評級 |
|---|---|
| 機密性 | 可接受 |
| 完整性 | 有條件可接受 |
| 可用性 | 有條件可接受 |
| NIST AI RMF（開發生命週期） | 部分對齊：模型揭露清楚，測試與部署治理不足 |
| OWASP LLM Top 10 2025 | 執行期多為不適用；開發期關注供應鏈、AI 產出審查、文件一致性 |

**整體驗收（公開表述）：** 引擎原始碼未見嚴重或高風險可利用缺陷。公開網頁版在「是否由當下 `main` 可靠自動發布」這點上，審查當時尚未對齊。

## 範圍與方法

靜態閱讀原始碼與鎖定依賴，並核對公開的 GitHub Pages 與 Actions 狀態。另以 [NIST AI RMF 1.0](https://doi.org/10.6028/NIST.AI.100-1)、[NIST AI 600-1](https://doi.org/10.6028/NIST.AI.600-1) 與 [OWASP Top 10 for LLM Applications 2025](https://genai.owasp.org/resource/owasp-top-10-for-llm-applications-2025/) 檢視 **AI 輔助開發**，不是把產品當成執行期 LLM 應用。

本專案 README 已說明由人類與 AI 助理（AGY 等）共同開發。執行期沒有 prompt、RAG 或模型推論，因此不將 Prompt Injection 列為產品漏洞。

審查機未執行 `cargo test`／`cargo audit`（當時無 Rust 工具鏈）。Actions 上同 commit 的 WASM **建置成功**、當時的 **deploy-pages 部署作業失敗**。依賴面以 lockfile 與公開 advisory 對照，未發現適用的已知漏洞。

## 發現

### TF-01 中 — 自動部署路徑與實際 Pages 來源不一致

- **面向：** 可用性、完整性
- **位置：** `.github/workflows/deploy.yml`；GitHub Pages 設定
- **影響：** 維護者可能以為推送 `main` 就會更新線上試玩。實際託管來源是 `gh-pages` 分支，修補若只進 `main`，玩家不一定拿得到。
- **處理：** 已改為 Actions 在 `main` 建置後把 `dist/` 發布到 `gh-pages`（PR #3）。

### TF-02 低 — 網頁版第三方腳本缺少完整性校驗

- **面向：** 完整性
- **位置：** `dist/index.html`（jsDelivr 上的 xterm.js）
- **影響：** 若該 CDN 物件被替換，訪客瀏覽器會在試玩頁執行非預期腳本。本頁沒有帳號或機密可偷，影響主要是頁面被竄改。
- **建議：** 改為專案內自託管，或為 CDN 資源加上 Subresource Integrity，並考慮 Content-Security-Policy。
- **處理：** 已自託管 `@xterm/xterm@5.5.0` 與 `@xterm/addon-fit@0.10.0` 於 `dist/vendor/`，並加上僅允許 `'self'` 的 CSP meta。

### TF-03 低 — 倉庫中的安全標頭檔在 GitHub Pages 未生效

- **面向：** 完整性
- **位置：** `dist/_headers`
- **影響：** 線上回應沒有該檔所宣稱的安全標頭；頁面可被其他網站嵌入。此示範沒有登入動作，點擊劫持價值有限。
- **建議：** 不要把 GitHub Pages 上的 `_headers` 當成已套用的控制。若需要 HTTP 標頭，改用支援該機制的託管，或在文件中說明限制。
- **處理：** 已刪除會造成誤解的 `dist/_headers`；改以 CSP／referrer meta、`framebust.js` 與 `dist/GITHUB-PAGES-HEADERS.md` 說明平台限制。

## NIST AI 與 OWASP AI（公開摘要）

TermForge **不是**生成式 AI 產品。下列檢視針對開發時使用的 coding agent 與文件品質。

**NIST AI RMF：** GOVERN 有模型與作者揭露，屬優點。MAP／MEASURE／MANAGE 不足：測試「通過」缺少 CI 複驗、部署敘事與實況不符。建議以 CI 綠燈為唯一通過證據，並為 AI 產出保留人類核可。

**OWASP LLM Top 10 2025：** 執行期 LLM01／04／07／08 等不適用，不是「已通過認證」。較相關的是開發期的供應鏈（LLM03）、把 AI 程式當可信輸出（LLM05）、agent 對發布流程權限過大（LLM06）、以及文件與徽章和實況不一致（LLM09）。這些說明了 TF-02、TF-03 與測試不可複驗，**不另上修為高或嚴重**。

## 工程建議

1. CI 增加 `cargo test --workspace`（以及可行時的依賴 advisory 檢查）。
2. 第三方 GitHub Action 儘量釘選 commit SHA。
3. `wasm-bindgen-cli` 安裝使用鎖定依賴。
4. 網頁按鍵與畫布尺寸設上限；原生輸入狀態補齊按鍵結束語意。
5. 遊戲邏輯單一來源，避免原生與 WASM 各維護一份完整實作。
6. README 的離開鍵、WASM 體積徽章、OpenSpec「完成」狀態與實際部署對齊。
7. PR 流程標明 AI 產出已經人類看過；coding agent 不要改 Pages environment 或 secrets。

## 限制聲明

本摘要由 **Cursor Grok 4.6** 輔助完成，是單次原始碼初測，不是滲透測試、不是 NIST／OWASP 認證。單次審查無法窮盡所有缺陷；未涵蓋 fuzz、惡意頁面實測與未公開漏洞。結論僅對上述 commit 與當時的公開託管／Actions 設定有效。標為「不適用」表示該風險類別目前沒有對應攻擊面。
