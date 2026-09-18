# Delta Spec: Trusted applicability workflow

Reference: `secure-by-construction`

本 change 對基線七款的標記見 `design.md` 適用表。

## ADDED Requirements

### Requirement: Applicability job executes code from the PR base
The applicability check SHALL run the checker file from `github.event.pull_request.base.sha`. Files from the PR head SHALL be read as data only.

#### Scenario: PR cannot neuter the checker by rewriting it
- **GIVEN** 一個 PR 把 `scripts/check-secure-by-construction-applicability.py` 改成永遠成功
- **WHEN** `pull_request_target` 工作流程執行
- **THEN** SHALL 仍執行 base 上的檢查器
- **AND** SHALL NOT 執行 `pr/` 底下的 Python 檔

### Requirement: 信任邊界與誠實能力（本 change）
#### Scenario: Target trigger has no secrets and read-only contents
- **GIVEN** `.github/workflows/applicability.yml`
- **WHEN** 使用 `pull_request_target`
- **THEN** `permissions.contents` SHALL 為 `read`
- **AND** checkout SHALL `persist-credentials: false`
- **AND** SHALL NOT 引用 `${{ secrets.* }}`

### Requirement: 機密不得進倉（本 change）
#### Scenario: Workflow has no token body
- **GIVEN** 本 change 的 workflow diff
- **WHEN** 檢查
- **THEN** 不得含 token 字面值

### Requirement: 輸入與資源有界（本 change）
#### Scenario: Timeout unchanged
- **GIVEN** applicability job
- **WHEN** 執行
- **THEN** timeout SHALL 為 5 分鐘
