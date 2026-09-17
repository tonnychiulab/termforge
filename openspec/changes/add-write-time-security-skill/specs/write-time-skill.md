# Delta Spec: Write-time security skill

Reference: `secure-by-construction`

本 change 對基線七款的標記見 `design.md` 適用表。下列場景只描述**本變更**。

## ADDED Requirements

### Requirement: Write-time skill gates implementation
The repository SHALL include a Cursor project skill that agents load when implementing, adding web/frontend, adding CI or GitHub Actions, handling input, WASM, or writing an OpenSpec change.

#### Scenario: Missing security scenes block implementation
- **GIVEN** 工作是改程式、`dist/`、CI、WASM 或開 OpenSpec change
- **WHEN** 當前 change 缺少七款 適用／不適用標記，或 適用款沒有本變更的 GIVEN／WHEN／THEN
- **THEN** agent SHALL NOT 寫實作
- **AND** SHALL 要求先補場景

#### Scenario: Review requests do not use this skill
- **GIVEN** 使用者要求 code review、安全審計或滲透測試
- **WHEN** agent 選擇 skill
- **THEN** 不得把本 skill 當成審查流程
- **AND** 不得因此重跑 CIA 報告

### Requirement: 信任邊界與誠實能力（本 change）
#### Scenario: Skill does not pretend to be a scanner
- **GIVEN** 本 change 只新增 `.cursor/skills/secure-by-construction/SKILL.md`
- **WHEN** proposal 描述能力
- **THEN** 必須列出無後端、非滲透測試、非 CIA 初測
- **AND** 不得宣稱已阻擋未讀 skill 的 git commit

### Requirement: 機密不得進倉（本 change）
#### Scenario: Skill text has no secrets
- **GIVEN** SKILL.md 將被 push 到公開倉
- **WHEN** 檢查 diff
- **THEN** 不得含 token 字面值、`.env`、私鑰、內部報告路徑
