# Design: Secure by Construction baseline

## Context

TermForge 已用 OpenSpec 驅動引擎、WASM 與操作變更。安全審查發生在碼與 Pages 設定完成之後。要把約束前移，必須讓「change 完整」包含安全場景，而不是另做審查工具。

## 基線條款適用表（本 change）

| 條款 | 適用？ | 原因 |
|---|---|---|
| 信任邊界與誠實能力 | 適用 | 本 proposal／design 必須誠實寫無後端 |
| 託管與發布路徑一致 | 不適用 | 不改 Pages、workflow、README 發布敘事 |
| 第三方腳本不得無完整性載入 | 不適用 | 不改 `dist/`、HTML、vendor |
| 平台做不到的控制不得當成已生效 | 不適用 | 不改標頭、CSP、framebust |
| 機密不得進倉 | 適用 | 公開規格不得含內部報告路徑或 token |
| 輸入與資源有界 | 不適用 | 無新的執行時輸入面 |
| 不適用要寫下來 | 適用 | 本表即該要求的實現 |

## Architectural Decisions

### Decision 1: Spec in-repo, not a scanner
- **Decision:** 把 SHALL 寫進 `openspec/specs/secure-by-construction.md`，用 `config.yaml` rules 強制引用。
- **Rationale:** 人與 agent 已經在看 OpenSpec；審查 CLI／MCP 是事後網。
- **Alternatives considered:** Codex Security 或 Alibaba Open Code Review 當主路徑（拒絕：diff／歷史掃描，不是撰寫時）；只寫 Cursor skill（拒絕：人類新人不必載入 skill）。

### Decision 2: Current spec plus ADDED delta, same body
- **Decision:** 頂層 specs 與 change delta 的 Requirement 正文相同。
- **Rationale:** TermForge 先前沒有頂層 `openspec/specs/`；新增現行規格避免只活在 change 資料夾。
- **Alternatives considered:** 只放在 change 內（拒絕：後續 change 難引用穩定路徑）。

### Decision 3: No git hook in this slice
- **Decision:** 不裝 pre-commit 擋缺安全場景的 commit。
- **Rationale:** 第一刀只建立規格契約；hook 是額外執行時機制，超出本 change。
- **Alternatives considered:** 強制 hook（延後）。

## Module-ish layout

```text
openspec/specs/secure-by-construction.md
        ^
        |  referenced by
openspec/config.yaml rules
        ^
        |  obeyed by
openspec/changes/<future>/proposal.md + specs/ + design.md + tasks.md
```

## Memory / runtime

無。無新 binary、無新依賴。
