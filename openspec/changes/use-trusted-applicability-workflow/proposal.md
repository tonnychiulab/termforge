# Proposal: Run the merge gate from the PR base (use-trusted-applicability-workflow)

## Why

GitHub ruleset 已鎖 `main`（必須 PR、`applicability` 必過）。但 `pull_request` 工作流程用的是 **PR 裡的 YAML**，只要同一 PR 把 job 改成 `exit 0` 並填一份適用表，門就沒了。要把「腳本與 YAML 都從 base 來」做實。

## What Changes

`applicability.yml` 增加 `pull_request_target`，檢查器與 workflow 以 `github.event.pull_request.base.sha` 為準；PR 樹只當資料、不執行。暫留 `pull_request` 以免這次 PR 自己沒有檢查可跑。不改遊戲、`dist/`、deploy、不新增 secrets。

## 信任邊界（本變更）

- **誰輸入：** PR head 的檔名與 markdown。
- **誰執行：** runner 執行 **base** 上的 Python 檢查器；不執行 PR 內腳本。
- **資料在哪：** 公開 PR。`pull_request_target` 預設權限較高，本 workflow `permissions: contents: read` 且 `persist-credentials: false`。
- **後端／帳號／機密：** 無 secrets。
- **不提供的能力：** 不是 Open Code Review（阿里雲）。不驗證適用表是否說謊。有權改 Settings 的人仍可關掉 ruleset。

## Capabilities

### Modified
- `applicability-merge-gate`: 閘門程式來自 PR base，不是 PR head。

## Impact

- **Affected Modules:** `.github/workflows/applicability.yml`
- **Performance targets:** timeout 仍 5 分鐘。
- **Breaking Changes:** 無。fork PR 仍只讀、無 secrets。
