# Proposal: PR 事後審查網 (add-pr-review-net)

## Why

撰寫時基線與 write-time skill 不能保證規格漏掉的缺陷被看見。需要一道 PR 上的可選網，但不能變成新人教材，也不能重造審查 MCP。

## What Changes

新增 GitHub Actions 工作流程，在 PR 上跑 Alibaba Open Code Review（有 LLM secret 才跑）。文件說明 Codex Security 由維護者在 ChatGPT 自行連倉庫，不接進 CI。不改遊戲、WASM、`dist/`。

## 信任邊界（本變更）

- **誰輸入：** PR diff；OCR 把 diff 與 LLM prompt 送到 `OCR_LLM_URL`。
- **誰執行：** GitHub-hosted runner；LLM 供應商。
- **資料在哪：** 公開 PR 內容會離開 GitHub 到所設定的 LLM endpoint。
- **後端／帳號／機密：** 本變更不新增應用後端。LLM token 只放在 GitHub Actions secrets。
- **不提供的能力：** 不是撰寫時安全、不是滲透測試、不是合併門檻、不審查 fork PR（無 secrets）。

## Capabilities

### Added
- `pr-ocr-review-net`: 可選 PR 評論網，secret 缺失則跳過。

## Impact

- **Affected Modules:** `.github/workflows/pr-review-net.yml`、`docs/security/review-net.md`
- **Performance targets:** 工作流程 timeout 30 分鐘；不影響 60 FPS 遊戲迴圈。
- **Breaking Changes:** 無。未設 secret 時 PR 行為與現在相同。
