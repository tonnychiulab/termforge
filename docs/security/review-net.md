# 事後審查網（不是撰寫時教材）

TermForge 的安全順序是：

1. **撰寫時：** `openspec/specs/secure-by-construction.md` + OpenSpec change 的適用表  
2. **動手時：** `.cursor/skills/secure-by-construction/SKILL.md`  
3. **合併門檻：** GitHub ruleset 鎖 `main`（必須走 PR、禁止 force-push）。跑 **default branch 上的** `.github/workflows/applicability.yml` 檢查適用表；腳本從 base 取出，PR 不能把腳本改成永遠通過。GitHub 網頁改檔、不用 Agent、「先改再說」都走這道門，不靠 skill。  
4. **寫完之後（本檔）：** PR 上的 Open Code Review（阿里雲），可選 Codex Security  

第 4 層只抓規格漏掉的缺陷。不要把它當成新人訓練，也不要當成唯一安全流程，更不要把它當成合併門檻。不要為此重造審查 MCP。

## Open Code Review（已接 CI）

工作流程：[`.github/workflows/pr-review-net.yml`](../../.github/workflows/pr-review-net.yml)

- 事件：`pull_request`（**不用** `pull_request_target`，避免 fork 工作流程拿到 secrets）
- 未設定 `OCR_LLM_AUTH_TOKEN` 時**跳過**（fork PR 也是如此）
- Action 釘選 `alibaba/open-code-review@a66240084b382ed97a47590bdec13a6a34df0743`（v1.10.0），CLI `ocr_version: 1.10.0`
- `continue-on-error: true`：LLM 失敗不阻擋合併；這是網，不是門

### 要啟用時（維護者）

Repository **Settings → Secrets and variables → Actions**：

| 名稱 | 類型 | 說明 |
|---|---|---|
| `OCR_LLM_URL` | secret | 例如 `https://api.openai.com/v1/chat/completions` |
| `OCR_LLM_AUTH_TOKEN` | secret | 模型 API token（不要寫進倉） |
| `OCR_LLM_MODEL` | variable | 可選；未設則 `gpt-4o` |
| `OCR_LLM_USE_ANTHROPIC` | variable | 可選；Anthropic 設 `true`，其餘 `false` |

權杖只存在 GitHub secrets。工作流程只引用 `${{ secrets.* }}`。

## Codex Security（未接 CI）

[Codex Security](https://help.openai.com/en/articles/20001107-codex-security) 是 ChatGPT 的倉庫掃描產品（威脅模型、沙盒驗證、建議 patch）。它掃的是**已存在的碼**，不能取代 OpenSpec。

本倉不把它嵌進 Actions：需要 ChatGPT 工作區與 GitHub 連線，無法用公開 workflow 代替。有帳號的維護者可自行在 ChatGPT 啟用此倉庫。
