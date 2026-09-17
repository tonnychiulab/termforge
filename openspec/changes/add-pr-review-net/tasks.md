# Implementation Checklist: add-pr-review-net

## 1. Workflow
- [x] 1.1 新增 `.github/workflows/pr-review-net.yml`（`pull_request`、無 token 跳過、SHA 釘選、continue-on-error）
- [x] 1.2 文件 `docs/security/review-net.md` 說明 OCR 啟用方式與 Codex Security 不接 CI

## 2. 安全場景（適用款）
- [x] 2.1 信任邊界寫明 LLM 外送
- [x] 2.2 workflow 無 token 字面值
- [x] 2.3 timeout 30 分鐘且無 token 則 skip

## 3. 不適用款
- [x] 3.1 託管／CDN／平台標頭已標不適用
- [x] 3.2 未修改 `dist/`、遊戲程式、deploy.yml
