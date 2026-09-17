# Proposal: 撰寫時安全基線 (add-secure-by-construction)

## Why

OpenSpec 目前規範效能、Gherkin 與模組邊界，但沒有撰寫時安全 SHALL。Web 變更因此能在規格裡寫「不必特殊 HTTP 標頭」、在 design 裡放 GitHub Pages 不會套用的 `_headers`，並從 CDN 載入腳本。安全變成寫完再審，新人與 coding agent 沒有同一份開工條件。

## What Changes

新增專案級規格 `secure-by-construction`，並在 `openspec/config.yaml` 要求每個 change 引用它。不改遊戲、WASM、`dist/` 或 Actions。

## 信任邊界（本變更）

- **誰輸入：** 作者與 coding agent 寫 OpenSpec markdown。
- **誰執行：** 人與 agent 閱讀規格；無新的執行時程式。
- **資料在哪：** 僅 git 內 markdown／YAML。
- **後端／帳號／機密：** 無。本變更不處理玩家資料、不新增網路服務。
- **不提供的能力：** 不是滲透測試、不是 CIA 全倉初測、不是 CI secret scanning 產品、不阻擋漏寫規格的 git commit（約束在 OpenSpec 流程，不在 hook）。

## Capabilities

### Added
- `secure-by-construction`: 七款撰寫時 SHALL（信任邊界、發布路徑、第三方腳本、平台控制誠實性、機密不進倉、輸入有界、不適用必填）。
- `openspec-security-rules`: 每個後續 change 的 proposal／specs／design／tasks 必須對七款標適用或不適用。

## Impact

- **Affected Modules:** `openspec/` only.
- **Breaking Changes:** 後續 OpenSpec change 若缺安全場景即不完整。既有已勾完的 historical changes 不回溯改寫。
