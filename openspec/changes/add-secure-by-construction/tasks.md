# Implementation Checklist: add-secure-by-construction

## 1. 規格正文
- [ ] 1.1 新增 `openspec/specs/secure-by-construction.md`（七款 Requirement）
- [ ] 1.2 新增 change delta `openspec/changes/add-secure-by-construction/specs/secure-by-construction.md`
- [ ] 1.3 確認兩份 Requirement 正文一致，且不含內部路徑或 token

## 2. 強制引用
- [ ] 2.1 更新 `openspec/config.yaml` 的 proposal／specs／design／tasks rules
- [ ] 2.2 本 change 的 design 含適用表；proposal 含信任邊界

## 3. 安全場景（本 change 適用款）
- [ ] 3.1 信任邊界已寫且未宣稱不存在的控制
- [ ] 3.2 抽檢：缺少「第三方腳本」場景的 CDN 假 change 依規則不得勾完安全 task（見工作區 fixture，勿提交該 fixture）
- [ ] 3.3 機密：`openspec/` 新增檔無 `ghp_`、私鑰、`reports/termforge-cia-initial`

## 4. 不適用款（不得當沒看見）
- [ ] 4.1 託管／CDN／平台標頭三款已在 design 標不適用並寫原因
- [ ] 4.2 確認未修改 `dist/`、`.github/workflows/`、遊戲程式
