# Delta Spec: Applicability merge gate

Reference: `secure-by-construction`

本 change 對基線七款的標記見 `design.md` 適用表。

## ADDED Requirements

### Requirement: PR fails without an applicability table on gated changes
The repository SHALL run a pull_request check that fails when gated paths change without a complete 基線條款適用表 in a non-grandfathered OpenSpec change.

#### Scenario: Implementation PR without a table is red
- **GIVEN** 一個 PR 修改了 `.rs`、`dist/`、`.github/`、`.cursor/`、`scripts/` 或 `openspec/specs/`／`config.yaml`
- **WHEN** 同一 diff 沒有非祖父級 `openspec/changes/<id>/design.md` 對七款都標「適用」或「不適用」且原因非空
- **THEN** `applicability` 工作流程 SHALL 以非零結束
- **AND** SHALL NOT 使用 `pull_request_target`
- **AND** SHALL NOT 把 diff 送到外部 LLM
- **AND** 檢查腳本 SHALL 從 PR base（通常是 `main`）載入；base 尚無腳本時才允許用 PR 複本（僅首次落地）

#### Scenario: Docs-only PR is not blocked by the gate
- **GIVEN** 一個 PR 只改 `docs/` 或 `README.md`
- **WHEN** 沒有新增 gated 路徑
- **THEN** 適用表門檻 SHALL 不因「缺少新 change」而失敗
- **AND** 樹中既有的非祖父 change 若缺表仍 SHALL 失敗（防止基線 change 被拆表）

#### Scenario: Historical changes are grandfathered
- **GIVEN** `termforge-engine`、`add-web-wasm-support`、`enhanced-controls-and-abilities`、`add-in-page-controls-cheat-sheet`
- **WHEN** 檢查適用表
- **THEN** 不得因這些目錄沒有適用表而失敗

### Requirement: README tells a newcomer how the defenses load
Clone plus opening the folder in Cursor SHALL be documented as the way to get the write-time skill and OpenSpec baseline. Open Code Review（阿里雲）SHALL be documented as GitHub Actions, not a laptop install.

#### Scenario: Fresh machine instructions
- **GIVEN** 新人已裝 Cursor、尚未 clone
- **WHEN** 讀 README 的撰寫時安全段
- **THEN** 必須寫：用 Open Folder 開整個倉庫、不要把 skill 裝到個人全域、不要為這件事裝 MCP、Open Code Review（阿里雲）不在筆電上
- **AND** 必須連到 `openspec/specs/secure-by-construction.md` 與專案 skill 路徑
- **AND** 必須寫明 GitHub 網頁改檔、不用 Agent、「先改再說」不能直推 `main`

### Requirement: 信任邊界與誠實能力（本 change）
#### Scenario: Docs do not call the optional net a merge gate
- **GIVEN** README 或 `docs/security/review-net.md`
- **WHEN** 描述本檢查與 Open Code Review（阿里雲）
- **THEN** 必須區分：適用表檢查是門；Open Code Review（阿里雲）是可選網
- **AND** 必須寫明 GitHub 網頁改檔、不用 Agent、「先改再說」都不靠 skill，而靠 ruleset 鎖 `main`

### Requirement: 機密不得進倉（本 change）
#### Scenario: Gate workflow has no secrets
- **GIVEN** `.github/workflows/applicability.yml`
- **WHEN** 檢查 diff
- **THEN** 不得出現 `${{ secrets.* }}` 以外的權杖本體
- **AND** 本門檻 workflow SHALL 不需要任何 repository secret

### Requirement: 輸入與資源有界（本 change）
#### Scenario: Checker only reads paths and markdown
- **GIVEN** `scripts/check-secure-by-construction-applicability.py`
- **WHEN** 在 CI 執行
- **THEN** timeout SHALL 不超過 5 分鐘
- **AND** 只讀 git 檔名與倉內 `design.md`／`proposal.md`，不發送網路請求
