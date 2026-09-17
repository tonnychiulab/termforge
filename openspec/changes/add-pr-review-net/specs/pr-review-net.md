# Delta Spec: Optional PR review net

Reference: `secure-by-construction`

本 change 對基線七款的標記見 `design.md` 適用表。

## ADDED Requirements

### Requirement: Optional OCR net on pull requests
The repository SHALL provide a GitHub Actions workflow that may run Open Code Review on pull requests when an LLM token secret is present.

#### Scenario: Missing token skips the net
- **GIVEN** `OCR_LLM_AUTH_TOKEN` 未設定（含 fork PR）
- **WHEN** `pull_request` 工作流程執行
- **THEN** 不得呼叫 LLM
- **AND** 工作流程不得因此失敗

#### Scenario: Configured token reviews the PR diff
- **GIVEN** 維護者已設定 `OCR_LLM_URL` 與 `OCR_LLM_AUTH_TOKEN`
- **WHEN** 同源倉庫的 PR 開啟或更新
- **THEN** workflow SHALL 對該 PR 的 merge-base…head diff 跑 OCR
- **AND** SHALL NOT 使用 `pull_request_target`
- **AND** OCR 行程失敗 SHALL NOT 單獨阻擋合併（continue-on-error）

### Requirement: 信任邊界與誠實能力（本 change）
#### Scenario: Docs admit data leaves GitHub
- **GIVEN** 本 change 的 proposal／文件
- **WHEN** 描述 OCR
- **THEN** 必須寫明 PR 內容會送到所設定的 LLM endpoint
- **AND** 不得宣稱這是撰寫時安全或合併門檻

### Requirement: 機密不得進倉（本 change）
#### Scenario: Workflow references secrets only
- **GIVEN** `.github/workflows/pr-review-net.yml`
- **WHEN** 檢查 diff
- **THEN** 只能出現 `${{ secrets.* }}` 與 `${{ vars.* }}`
- **AND** 不得含 token 字面值

### Requirement: 輸入與資源有界（本 change）
#### Scenario: Job has a timeout and skip gate
- **GIVEN** OCR job
- **WHEN** 執行
- **THEN** timeout SHALL 為 30 分鐘
- **AND** 無 token 時 SHALL skip
