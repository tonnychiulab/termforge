# Design: Trusted applicability workflow

## Context

Ruleset 擋直推。下一步擋「PR 改自己的 workflow 讓檢查永遠過」。

## 基線條款適用表（本 change）

| 條款 | 適用？ | 原因 |
|---|---|---|
| 信任邊界與誠實能力 | 適用 | 必須寫明 pull_request_target 只讀、不執行 PR 程式、無權杖 |
| 託管與發布路徑一致 | 不適用 | 不改 Pages 或 deploy.yml |
| 第三方腳本不得無完整性載入 | 不適用 | 不改 dist／HTML |
| 平台做不到的控制不得當成已生效 | 不適用 | 不改 CSP |
| 機密不得進倉 | 適用 | 無 secrets；credentials 不持久化 |
| 輸入與資源有界 | 適用 | 只解析 PR 檔名與 markdown；timeout 5 分鐘 |
| 不適用要寫下來 | 適用 | 本表 |

## Architectural Decisions

### Decision 1: pull_request_target for YAML trust, not for secrets
- **Decision:** 用 `pull_request_target` 讓 YAML 來自 base。禁止 checkout 後執行 `pr/` 內任何腳本。
- **Rationale:** 個人倉的 ruleset 不接受 `workflows` 規則（API 422）。這是同等效果。Open Code Review（阿里雲）仍禁用 `pull_request_target`，因為它有 token。
- **Alternatives considered:** 只靠 required_status_checks（拒絕：PR 可改 YAML）；org required workflow（拒絕：此倉非 org）。

### Decision 2: Keep pull_request until this lands
- **Decision:** 兩個 trigger 暫時並存。
- **Rationale:** 若這次 PR 刪掉 `pull_request`，而 main 上還沒有 `pull_request_target`，required check 不會跑、PR 合並不了。
- **Alternatives considered:** 一次切換（拒絕：bootstrap 會死鎖）。

```text
ruleset: PR required + applicability must pass + no force-push
workflow on base SHA -> python checker --root pr
PR tree is data, never executed
```
