# Implementation Checklist: add-applicability-merge-gate

## 1. Checker
- [x] 1.1 新增 `scripts/check-secure-by-construction-applicability.py`（標準庫、可 `--self-test`）
- [x] 1.2 自測：完整表通過、缺一款失敗、祖父目錄豁免、gated 無 change 失敗、docs-only 通過
- [x] 1.3 對目前非祖父 change（含本 change）跑一次，必須通過

## 2. CI 門檻
- [x] 2.1 新增 `.github/workflows/applicability.yml`（`pull_request`、無 secrets、timeout ≤ 5 分鐘）
- [x] 2.2 不用 `pull_request_target`；不把 diff 送 LLM

## 3. 新人入口
- [x] 3.1 README 繁中／英文寫 clone → Open Folder、skill 在倉內、先填適用表、Open Code Review（阿里雲）在 GitHub
- [x] 3.2 更新 skill：缺表則 CI 會紅
- [x] 3.3 `docs/security/review-net.md` 區分門與網；誠實寫 Branch protection 需維護者勾選
- [x] 3.4 `openspec/config.yaml` 註明適用表由 CI 檢查

## 4. 安全場景（適用款）
- [x] 4.1 信任邊界：文件區分門／網，不把可選審查當合併門檻
- [x] 4.2 機密：applicability workflow 無 token 本體、不需 secrets
- [x] 4.3 輸入有界：self-test 覆蓋缺表／docs-only／祖父清單

## 5. 不適用款
- [x] 5.1 託管／CDN／平台標頭已標不適用
- [x] 5.2 未修改 `dist/`、遊戲程式、`deploy.yml`
