# Implementation Checklist: use-trusted-applicability-workflow

## 1. Workflow
- [x] 1.1 `pull_request` 與 `pull_request_target` 並存
- [x] 1.2 checker 來自 `base.sha`；PR 樹放 `pr/` 當資料
- [x] 1.3 `permissions: contents: read`、`persist-credentials: false`、無 secrets

## 2. 安全場景
- [x] 2.1 信任邊界寫明不執行 PR 程式
- [x] 2.2 機密：無 token
- [x] 2.3 timeout 5 分鐘

## 3. 不適用款
- [x] 3.1 託管／CDN／平台標頭已標不適用
- [x] 3.2 未改 `dist/`、遊戲、deploy.yml
