# Proposal: 撰寫時安全 skill (add-write-time-security-skill)

## Why

`secure-by-construction` 已是 OpenSpec 基線，但 coding agent 只有在自己去翻 `openspec/` 時才會看到。新人與 agent 在「實作、加網頁、加 CI、處理輸入」當下需要自動載入同一份契約，而不是事後 code review。

## What Changes

新增專案 skill `.cursor/skills/secure-by-construction/SKILL.md`：實作前讀基線與當前 change 的 適用／不適用表；缺場景就停。不改遊戲、WASM、`dist/`、Actions。不接 OCR／Codex、不做審查 MCP。

## 信任邊界（本變更）

- **誰輸入：** 作者與 coding agent 讀 skill 與 OpenSpec。
- **誰執行：** Cursor／其他載入 `.cursor/skills/` 的 agent；無新的執行時程式。
- **資料在哪：** git 內 markdown。
- **後端／帳號／機密：** 無。
- **不提供的能力：** 不是 CIA 初測、不是滲透測試、不是 secret scanning、不阻擋未讀 skill 的 git commit。

## Capabilities

### Added
- `write-time-security-skill`: 實作／網頁／CI／輸入／WASM／OpenSpec change 時載入；code review／安全審計／滲透測試不載入。

## Impact

- **Affected Modules:** `.cursor/skills/secure-by-construction/`
- **Performance targets:** 無執行時開銷。
- **Breaking Changes:** 無。未載入 skill 的 agent 仍受 `openspec/config.yaml` 約束。
