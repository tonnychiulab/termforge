# Design: Optional PR review net

## Context

第 1、2 層已存在。第 3 層必須是事後網：可關、可跳過、不可取代 OpenSpec。

## 基線條款適用表（本 change）

| 條款 | 適用？ | 原因 |
|---|---|---|
| 信任邊界與誠實能力 | 適用 | 必須寫明 PR diff 會送到外部 LLM |
| 託管與發布路徑一致 | 不適用 | 不改 Pages 或 deploy.yml |
| 第三方腳本不得無完整性載入 | 不適用 | 不改 `dist/`／HTML |
| 平台做不到的控制不得當成已生效 | 不適用 | 不改安全標頭 |
| 機密不得進倉 | 適用 | workflow 只引用 secrets；文件不含 token |
| 輸入與資源有界 | 適用 | OCR 有 timeout；fork 無 secret 則不跑 |
| 不適用要寫下來 | 適用 | 本表 |

## Architectural Decisions

### Decision 1: Open Code Review in Actions, Codex Security as docs
- **Decision:** CI 接 OCR；Codex Security 只文件化。
- **Rationale:** OCR 有可釘選的 GitHub Action。Codex Security 綁 ChatGPT 工作區，無法用公開 YAML 代替。
- **Alternatives considered:** 只接 Codex（拒絕：公開倉無法安裝）；自製審查 MCP（拒絕：設計禁止）。

### Decision 2: `pull_request` not `pull_request_target`
- **Decision:** 用 `pull_request`。無 token 則 skip。
- **Rationale:** `pull_request_target` 讓 fork 工作流程碰到 secrets，風險高。本網寧可不審 fork，也不把 LLM token 暴露給不信任的 workflow。
- **Alternatives considered:** 官方範例的 `pull_request_target`（拒絕）。

### Decision 3: Pin Action SHA and CLI version; do not block merge
- **Decision:** `alibaba/open-code-review@a66240084b382ed97a47590bdec13a6a34df0743` + `ocr_version: "1.10.0"`；`continue-on-error: true`。
- **Rationale:** 審查建議釘選 SHA；官方預設 `ocr_version: latest` 會讓行為漂移。網失敗不應擋住已通過 OpenSpec 的變更。
- **Alternatives considered:** `@main`（拒絕）；OCR 失敗即紅燈（拒絕：會變成唯一門檻）。

```text
PR opened
  -> gate: OCR_LLM_AUTH_TOKEN empty? skip
  -> ocr review (diff only) -> PR comments
OpenSpec / skill remain the write-time path
```
