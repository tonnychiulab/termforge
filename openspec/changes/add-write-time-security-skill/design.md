# Design: Write-time security skill

## Context

基線規格已在 `openspec/specs/secure-by-construction.md`。這一刀只加「寫的時候」載入器，不重寫基線、不重跑審查。

## 基線條款適用表（本 change）

| 條款 | 適用？ | 原因 |
|---|---|---|
| 信任邊界與誠實能力 | 適用 | proposal 必須誠實寫無後端、非審計工具 |
| 託管與發布路徑一致 | 不適用 | 不改 Pages、workflow、README 發布敘事 |
| 第三方腳本不得無完整性載入 | 不適用 | 不改 `dist/` 或 HTML |
| 平台做不到的控制不得當成已生效 | 不適用 | 不改標頭或 CSP |
| 機密不得進倉 | 適用 | skill 正文不得含 token、內部報告路徑 |
| 輸入與資源有界 | 不適用 | 無新的執行時輸入面 |
| 不適用要寫下來 | 適用 | 本表即該要求 |

## Architectural Decisions

### Decision 1: Project skill, not a scanner
- **Decision:** 放 `.cursor/skills/secure-by-construction/SKILL.md`，描述只寫觸發條件。
- **Rationale:** 人與 Cursor agent  clone 倉庫即看得到；審查 CLI 是後門。
- **Alternatives considered:** 只寫進個人 `~/.cursor/skills/`（拒絕：新人 clone 不到）；做成 MCP（拒絕：撰寫時不該是工具呼叫）。

### Decision 2: Point at the spec, do not copy it
- **Decision:** skill 命令 agent 去讀 `openspec/specs/secure-by-construction.md` 與當前 change。
- **Rationale:** 避免兩份 SHALL 漂移。
- **Alternatives considered:** 把七條 SHALL 貼進 SKILL.md（拒絕：重複）。

```text
.cursor/skills/secure-by-construction/SKILL.md
        |
        v
openspec/specs/secure-by-construction.md
        ^
        |  current change 適用／不適用
openspec/changes/<this-work>/
```

## Memory / runtime

無新 binary、無新依賴。
