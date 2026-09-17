<div align="center">

# 🦀 TermForge

### 極致效能 · 純 Rust 打造的 60 FPS 雙緩衝終端機遊戲引擎與經典 Asteroids 街機遊戲
**A High-Performance Pure Rust Terminal Game Engine & Asteroids Arcade Game**

<p align="center">
  <a href="https://tonnychiulab.github.io/termforge/">
    <img src="https://img.shields.io/badge/🎮_點擊立即免安裝試玩-PLAY_ONLINE_NOW-33ff66?style=for-the-badge&logo=google-chrome&logoColor=black" alt="Play Online Now" height="42">
  </a>
</p>

[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg?style=flat-square)](#)
[![Rust](https://img.shields.io/badge/rust-2021_edition-orange.svg?style=flat-square&logo=rust)](#)
[![Wasm Size](https://img.shields.io/badge/Wasm_Size-76_KB-brightgreen.svg?style=flat-square)](#)
[![OpenSpec SDD](https://img.shields.io/badge/Spec--Driven-OpenSpec_Compliant-purple.svg?style=flat-square)](https://github.com/Fission-AI/openspec)

[繁體中文](#-繁體中文說明) · [English](#-english-documentation) · [OpenSpec 規範](#-openspec-sdd-規格驅動開發文件)

---

### 🕹️ 傳送門：直接在瀏覽器開打！
👉 **[https://tonnychiulab.github.io/termforge/](https://tonnychiulab.github.io/termforge/)** 👈  
*(免安裝、零下載！支援電腦鍵盤與手機觸控，內建 8-bit 街機合成音效與復古 CRT 掃描線光暈)*

---

</div>

<br>

# 🇹🇼 繁體中文說明

## 🌟 專案亮點

**TermForge** 是一個從零以純 Rust 打造的終端機遊戲引擎，完全遵循 [OpenSpec](https://github.com/Fission-AI/openspec) 規格驅動開發 (SDD) 規範。

- 🎬 **無閃爍雙緩衝渲染器 (Double-Buffered Grid Renderer)**：
  - 前後雙緩衝區 Diff 演算法，只輸出每幀變更的字元（Dirty-rect），游標智慧跳轉，徹底告別傳統終端應用的殘影與全螢幕閃爍。
- 🧩 **世代競技場 ECS 架構 (Generational Arena ECS)**：
  - 輕量化、極速 $O(1)$ 實體配置，世代計數器徹底杜絕 ABA 懸垂指標參照。
- ⏱️ **固定時間步階遊戲迴圈 (Fixed-Timestep Game Loop)**：
  - 經典 60Hz 累加器物理模擬，繪圖與邏輯分離，附帶 250ms 防止螺旋死亡（spiral of death）保護機制。
- 📍 **四元樹空間分割 (Quadtree Spatial Indexing)**：
  - $O(N \log N)$ 廣相碰撞篩選，支援圓形、AABB 碰撞盒與位元遮罩圖層過濾。
- ✨ **補間與物件池粒子系統 (Tween & Particle Systems)**：
  - 30+ 種 Easing 曲線、ASCII 影格精靈動畫、爆炸微粒發射器與漸層色彩衰減。
- 🛡️ **RAII 終端保護鎖 (Panic-Safe Terminal Guard)**：
  - 具備全域 Panic Hook，即使遊戲異常中斷也會自動恢復原始終端機與可見游標。
- 🌐 **WebAssembly 網頁虛擬終端**：
  - 核心邏輯編譯成 WASM 僅 **76 KB**，搭配 xterm.js 與 CSS3 硬體加速復古 CRT 螢幕特效。

---

## 🎮 操作指南

| 按鍵 | 動作說明 | 特效與回饋 |
| :--- | :--- | :--- |
| <kbd>W</kbd> 或 <kbd>↑</kbd> | **推進加速 (Thrust)** | 向量慣性加速度，船尾噴射微粒火花動畫 |
| <kbd>A</kbd> 或 <kbd>←</kbd> | **逆時針旋轉 (Rotate Left)** | 8 方向平滑旋轉與船身 ASCII 符號即時切換 |
| <kbd>D</kbd> 或 <kbd>→</kbd> | **順時針旋轉 (Rotate Right)** | 8 方向平滑旋轉與船身 ASCII 符號即時切換 |
| <kbd>Space</kbd> | **發射電漿砲 (Fire)** | 冷卻射速限制，內建 Web Audio 激光音效 |
| <kbd>R</kbd> 或 <kbd>Enter</kbd> | **重新開始 (Restart)** | Game Over 後快速重啟全新戰局 |
| <kbd>Ctrl</kbd> + <kbd>Q</kbd> / <kbd>Esc</kbd> | **離開遊戲 (Quit)** | 安全退出遊戲並還原終端機 |

---

## 💻 本地端運行

如果你已安裝 Rust 環境，可以在終端機原生運行以體驗最純粹的命令列手感：

```bash
# Clone 倉庫
git clone https://github.com/tonnychiulab/termforge.git
cd termforge

# 以 Release 極致效能模式啟動 Asteroids 遊戲
cargo run --release -p asteroids

# 執行全工作區單元測試
cargo test --workspace
```

---

<br>

# 🇺🇸 English Documentation

## Architecture & Features

- **Double-Buffered Renderer**: Differential rendering computes dirty cells each frame, minimizing ANSI escape sequence output to eliminate screen flickering at 60 FPS.
- **Generational Arena ECS**: High-performance $O(1)$ cache-friendly entity allocator with generational counters to solve the ABA problem.
- **Fixed-Timestep Loop**: Decoupled physics (60 Hz) and rendering with delta-time clamping.
- **Quadtree Collision Detection**: $O(N \log N)$ broadphase spatial acceleration for circular and AABB colliders.
- **Tween & Particle Engine**: 30+ easing curves, memory-pooled particle emitter, and ASCII sprite animators.
- **WebAssembly + Retro CRT**: Runs at 60 FPS in browsers via `xterm.js` and WebAssembly with authentic CRT phosphor glow and scanlines.

---

## 📂 OpenSpec SDD 規格驅動開發文件

本專案從功能構想、架構設計到程式碼撰寫，皆嚴謹遵循 **[OpenSpec SDD](https://github.com/Fission-AI/openspec)** 規範，完整規格目錄可供查閱：

- [`openspec/config.yaml`](openspec/config.yaml) — OpenSpec 專案規則與技術棧定義
- **變更 1：TermForge 核心引擎與 Asteroids 遊戲 (`termforge-engine`)**
  - [`proposal.md`](openspec/changes/termforge-engine/proposal.md) — 專案動機、痛點分析與功能宣告
  - [`specs/engine-core.md`](openspec/changes/termforge-engine/specs/engine-core.md) — 核心迴圈、雙緩衝繪製與 ECS 規範
  - [`specs/subsystems.md`](openspec/changes/termforge-engine/specs/subsystems.md) — 動畫、四元樹碰撞、輸入與音效子系統規範
  - [`specs/demo-game.md`](openspec/changes/termforge-engine/specs/demo-game.md) — Asteroids 街機遊戲玩法與難度演進規範
  - [`design.md`](openspec/changes/termforge-engine/design.md) — 技術決策（決策理由、替代方案評估、資料流圖）
  - [`tasks.md`](openspec/changes/termforge-engine/tasks.md) — 分階段實作檢核表 (100% 完成)
- **變更 2：WebAssembly 網頁版與復古 CRT 終端 (`add-web-wasm-support`)**
  - [`proposal.md`](openspec/changes/add-web-wasm-support/proposal.md) — 瀏覽器跨平臺遊玩背景與需求
  - [`specs/web-support.md`](openspec/changes/add-web-wasm-support/specs/web-support.md) — WASM 橋接、觸控支援與驗收情境
  - [`design.md`](openspec/changes/add-web-wasm-support/design.md) — 記憶體字串 Diff 輸出與 CSS CRT 著色架構
  - [`tasks.md`](openspec/changes/add-web-wasm-support/tasks.md) — 網頁實作與自動化部屬檢核表 (100% 完成)

---

## 👥 共同作者與開發團隊 (Authors & Credits)

本專案為 **人類工程師與 Antigravity (AGY) Agentic AI 深度結對編程 (Pair Programming)** 的極致結晶：

- **👨‍💻 Tonny Chiu** ([@tonnychiulab](https://github.com/tonnychiulab))
  - 專案發起、核心產品方向決策、技術棧選型互動與最終成果驗收。
- **🤖 Antigravity (AGY)**
  - Google DeepMind 設計的強大 Agentic AI 程式設計助理，全程自主執行專案架構設計、系統級代碼實現、WASM 橋接與 Git/Pages 部署。

### 🧠 協同開發模型架構說明 (AI Models Used)

在整個工程週期中，AGY 結合了不同專長的前沿大語言模型進行多階段協同作業：

1. **Claude Opus (Thinking)**：
   - **核心任務**：深入語義分析、全系統架構推導與 OpenSpec SDD 規範制訂。
   - **成果**：完成兩套完整的 SDD 變更規格（`termforge-engine` 與 `add-web-wasm-support`），嚴謹產出 RFC 2119 行為場景、Generational Arena ECS 設計與 Quadtree 空間分割決策。
2. **Gemini Pro & Flash**：
   - **核心任務**：作為背景子代理 (Subagents)，精準爬取並解析 OpenSpec 官方規格倉庫，提取核心 Schema 與語法規則。
3. **Gemini Flash (Medium)**：
   - **核心任務**：高吞吐量代碼生成、系統除錯與端到端工程落地。
   - **成果**：純 Rust 核心引擎代碼編寫、Rust 1.98.1 工具鏈與 wasm32 編譯排錯、WebAssembly + xterm.js 網頁版移植、CSS3 復古 CRT 掃描線著色器，以及 GitHub 遠端倉庫建立、PR 開啟與 GitHub Pages 全自動上線部署。

---

## 📜 開源許可證 (License)

本專案採用 [MIT OR Apache-2.0](LICENSE) 雙重許可證授權。

