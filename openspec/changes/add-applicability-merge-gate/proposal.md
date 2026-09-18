# Proposal: 適用表合併門檻與新人 README (add-applicability-merge-gate)

## Why

撰寫時基線與 Cursor skill 無法擋住「新人沒人帶、直接開寫」：README 沒寫 clone 後要做什麼，CI 也不檢查適用表。沒走 OpenSpec 的實作 PR 仍可合併。Open Code Review（阿里雲）是可選網，不是這道門。

## What Changes

1. README 寫清：新電腦裝 Cursor 後只要 clone、開整個資料夾；skill 在倉內；先開 OpenSpec change 並填適用表；Open Code Review（阿里雲）在 GitHub，不在筆電上。
2. PR CI 硬門：gated 路徑（程式、CI、`dist/`、skill、基線規格）有改動時，同一 PR 必須帶非祖父級 OpenSpec change，且 `design.md` 對七款都標適用或不適用加原因。缺表則工作流程失敗。
3. 既有歷史 change 不回溯。不改遊戲、WASM、`dist/` 執行期、deploy、不接審查 MCP。

## 信任邊界（本變更）

- **誰輸入：** PR 的檔名清單與倉內 markdown（適用表）。
- **誰執行：** GitHub-hosted runner 上的檢查腳本；本機可跑同一支 Python。
- **資料在哪：** 只讀公開倉檔案與 git diff 路徑；不把 diff 送到外部 LLM。
- **後端／帳號／機密：** 無。此 workflow 不使用 repository secrets。
- **不提供的能力：** 不是滲透測試、不是 secret scanning、不是 Open Code Review（阿里雲）。不阻擋本機「先改再說」寫在硬碟上；擋的是合進 `main`。不驗證適用表是否說謊（填了假「不適用」仍可能過門）。不阻止維護者在 Settings 關掉 ruleset。

## Capabilities

### Added
- `applicability-merge-gate`: PR 上檢查適用表；缺表則紅燈。
- `newcomer-readme`: clone + Cursor 開資料夾即可用專案 skill 與 OpenSpec 基線。

## Impact

- **Affected Modules:** `README.md`、`scripts/check-secure-by-construction-applicability.py`、`.github/workflows/applicability.yml`、`.cursor/skills/secure-by-construction/SKILL.md`、`docs/security/review-net.md`、`openspec/config.yaml`
- **Performance targets:** 檢查應在數秒內結束；timeout 5 分鐘。不影響 60 FPS 遊戲迴圈。
- **Breaking Changes:** 之後改 gated 路徑的 PR 若無適用表會失敗。歷史 change 目錄（見 design 祖父清單）不回溯。
