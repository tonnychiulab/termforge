# Implementation Checklist: add-write-time-security-skill

## 1. Skill
- [x] 1.1 新增 `.cursor/skills/secure-by-construction/SKILL.md`
- [x] 1.2 `description` 含實作、網頁、前端、CI、GitHub Actions、輸入、WASM、OpenSpec change，並排除 code review／安全審計／滲透測試
- [x] 1.3 正文指向 `openspec/specs/secure-by-construction.md`，缺場景則停，不重跑 CIA

## 2. 安全場景（適用款）
- [x] 2.1 信任邊界已寫且未宣稱不存在的控制
- [x] 2.2 SKILL.md 與本 change 文件無 token、私鑰、內部報告路徑

## 3. 不適用款
- [x] 3.1 託管／CDN／平台標頭／輸入有界已在 design 標不適用
- [x] 3.2 未修改 `dist/`、`.github/workflows/`、遊戲程式
