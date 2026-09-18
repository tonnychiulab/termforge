# Design: Applicability merge gate and newcomer README

## Context

Skill 只在 Cursor Agent 載入時有效。人類與不載 skill 的 agent 仍可直接改碼開 PR。要把「適用表」從契約變成合併門檻，並讓 README 成為無人帶隊時的入口。

## 基線條款適用表（本 change）

| 條款 | 適用？ | 原因 |
|---|---|---|
| 信任邊界與誠實能力 | 適用 | proposal／README 必須寫明檢查只讀路徑、無後端、不取代 Open Code Review（阿里雲） |
| 託管與發布路徑一致 | 不適用 | 不改 GitHub Pages 來源或 `deploy.yml`；新 workflow 只跑 PR 檢查、不發布 |
| 第三方腳本不得無完整性載入 | 不適用 | 不改 `dist/`、HTML、vendor |
| 平台做不到的控制不得當成已生效 | 不適用 | 不改 CSP、meta、framebust |
| 機密不得進倉 | 適用 | 檢查腳本與 workflow 不含 token；不新增 secrets |
| 輸入與資源有界 | 適用 | 只解析檔名清單與指定 markdown；祖父清單有限；timeout 5 分鐘 |
| 不適用要寫下來 | 適用 | 本表 |

## Architectural Decisions

### Decision 1: Fail the PR job; Branch protection is out of band
- **Decision:** workflow 缺表即 `exit 1`。不在本 change 用 API 改 repository ruleset。
- **Rationale:** 公開 workflow 不能假設有 admin。紅燈已讓審查看見；硬擋 Merge 需維護者勾 required check。
- **Alternatives considered:** 只寫 README（拒絕：仍可直接開寫）；用 Open Code Review（阿里雲）當門（拒絕：可選網、continue-on-error）。

### Decision 2: Grandfather historical OpenSpec changes
- **Decision:** `termforge-engine`、`add-web-wasm-support`、`enhanced-controls-and-abilities`、`add-in-page-controls-cheat-sheet` 不要求適用表。
- **Rationale:** 基線引入前的 change 沒有該表；回溯會變成假工。
- **Alternatives considered:** 全部回溯（拒絕）。

### Decision 3: Gate implementation paths, not every markdown file
- **Decision:** gated = Rust／Cargo、`dist/`、`.github/`、`.cursor/`、`openspec/specs/`、`openspec/config.yaml`、`scripts/`。`README.md` 與 `docs/` 不單獨觸發「必須有新 change」，但非祖父 change 只要存在於樹中仍須有完整表。
- **Rationale:** 工作日誌錯字不該被安全門擋住；程式與 CI 必須有表。
- **Alternatives considered:** 任何檔案都要 change（拒絕：過寬）。

```text
PR diff
  -> gated files? must include a non-grandfather OpenSpec change in the same diff
  -> every non-grandfather change in the tree: 七款 適用或 不適用 + 原因
README tells newcomers: clone, open folder, no MCP, no laptop Open Code Review
```

## Memory / runtime

Python 3 標準庫 only。無新 crate、無新 npm、無 secrets。
