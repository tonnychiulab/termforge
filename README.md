<div align="center">

# 🦀 TermForge

### 純 Rust 實作的終端機遊戲引擎與 Asteroids 示範專案
**A lightweight terminal game engine & Asteroids demo written in pure Rust**

<p align="center">
  <a href="https://tonnychiulab.github.io/termforge/">
    <img src="https://img.shields.io/badge/🎮_點擊免安裝線上試玩-PLAY_ONLINE-33ff66?style=for-the-badge&logo=google-chrome&logoColor=black" alt="Play Online" height="40">
  </a>
</p>

[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg?style=flat-square)](#)
[![Rust](https://img.shields.io/badge/rust-2021_edition-orange.svg?style=flat-square&logo=rust)](#)
[![Wasm Size](https://img.shields.io/badge/Wasm_Size-76_KB-brightgreen.svg?style=flat-square)](#)
[![OpenSpec SDD](https://img.shields.io/badge/Spec--Driven-OpenSpec_Compliant-purple.svg?style=flat-square)](https://github.com/Fission-AI/openspec)

[繁體中文](#-繁體中文說明) · [English](#-english-documentation) · [OpenSpec 規範](#-openspec-sdd-規格驅動開發文件)

---

### 🕹️ 網頁試玩傳送門
👉 **[https://tonnychiulab.github.io/termforge/](https://tonnychiulab.github.io/termforge/)** 👈  
*(免安裝，支援電腦鍵盤與手機觸控，採用 8-bit 音效合成與復古 CRT 螢幕風格)*

---

</div>

<br>

# 🇹🇼 繁體中文說明

## 🌟 專案簡介與特色

**TermForge** 是一個以純 Rust 開發的終端機遊戲引擎實驗專案，開發流程遵循 [OpenSpec](https://github.com/Fission-AI/openspec) 規格驅動開發 (SDD) 實踐。

- 🎬 **雙緩衝渲染機制 (Double-Buffered Grid Renderer)**：
  - 透過前後雙緩衝區比對差異，只更新該幀變更的字元（Dirty cells），降低終端輸出負擔並減少畫面閃爍。
- 🧩 **基礎 ECS 架構 (Generational Arena ECS)**：
  - 採用 Generational Index 設計的實體與元件儲存池，避免陳舊參照問題。
- ⏱️ **固定時間步階遊戲迴圈 (Fixed-Timestep Game Loop)**：
  - 60Hz 物理更新邏輯，模擬與渲染分離，並設有時間步階保護上限。
- 📍 **四元樹空間分割 (Quadtree Spatial Partitioning)**：
  - 實作簡易 Quadtree 空間索引，輔助多實體碰撞時的廣相過濾。
- ✨ **補間與粒子效果 (Tween & Particle Systems)**：
  - 支援基礎 Easing 函數、ASCII 影格動畫與物件池微粒效果。
- 🛡️ **終端狀態還原 (Panic-Safe Terminal Guard)**：
  - 透過 RAII Guard 與 Panic Hook，嘗試在異常離開時盡可能恢復終端原始設置。
- 🌐 **WebAssembly 網頁虛擬終端**：
  - 透過 WASM 與 xterm.js 將終端體驗移植至瀏覽器，搭配 CSS 模擬復古 CRT 效果。

---

## 🎮 操作指南 (Controls)

| 按鍵 (Key) | 動作說明 (Action) | 特效與機制 (Feedback & Mechanics) |
| :--- | :--- | :--- |
| <kbd>W</kbd> 或 <kbd>↑</kbd> | **推進加速 (Thrust)** | 順向向量加速度，船尾噴射微粒火花動畫 |
| <kbd>S</kbd> 或 <kbd>↓</kbd> | **主動煞車 (Brake)** | 主動逆向減速，急速降低慣性速度避免撞毀 |
| <kbd>A</kbd> 或 <kbd>←</kbd> | **逆時針旋轉 (Rotate Left)** | 8 方向平滑旋轉與船身 ASCII 符號即時切換 |
| <kbd>D</kbd> 或 <kbd>→</kbd> | **順時針旋轉 (Rotate Right)** | 8 方向平滑旋轉與船身 ASCII 符號即時切換 |
| <kbd>Q</kbd> | **左側平移 (Strafe Left)** | 垂直朝左側橫向噴射，戰術繞圈狗鬥 |
| <kbd>E</kbd> | **右側平移 (Strafe Right)** | 垂直朝右側橫向噴射，戰術繞圈狗鬥 |
| <kbd>X</kbd> | **180° 瞬間回頭 (Flip)** | 一鍵瞬間反轉 180 度反咬追擊的小行星 |
| <kbd>Z</kbd> 或 <kbd>H</kbd> | **超空間瞬移 (Hyperspace)** | 突發狀況隨機傳送至安全區域，帶 1.5 秒護盾 |
| <kbd>Space</kbd> | **發射電漿砲 (Fire)** | 冷卻連發，內建 8-bit Web Audio 激光音效 |
| <kbd>B</kbd> 或 <kbd>F</kbd> | **全屏 EMP 震撼彈 (Bomb)** | 每條命可用 1 次，引爆全場近身威脅並觸發圓環衝擊波 |
| <kbd>P</kbd> | **暫停 / 繼續 (Pause)** | 隨時切換暫停狀態 |
| <kbd>M</kbd> | **靜音切換 (Mute)** | 切換音效輸出開關 |
| <kbd>R</kbd> 或 <kbd>Enter</kbd> | **重新開始 (Restart)** | Game Over 後快速重啟全新戰局 |
| <kbd>Ctrl</kbd> + <kbd>Q</kbd> / <kbd>Esc</kbd> | **離開遊戲 (Quit)** | 安全退出遊戲並還原終端機 (原生版) |

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
- **變更 3：九宮格戰術機動與特殊技能 (`enhanced-controls-and-abilities`)**
  - [`proposal.md`](openspec/changes/enhanced-controls-and-abilities/proposal.md) — 戰術操控與技能系統規劃
  - [`specs/controls-and-abilities.md`](openspec/changes/enhanced-controls-and-abilities/specs/controls-and-abilities.md) — 煞車、側移、180° 翻轉、瞬移與 EMP 炸彈規範
  - [`design.md`](openspec/changes/enhanced-controls-and-abilities/design.md) — 運動力學向量與能力冷卻狀態機
  - [`tasks.md`](openspec/changes/enhanced-controls-and-abilities/tasks.md) — 實作檢核表 (100% 完成)

---

## 👥 共同作者與開發紀錄 (Authors & Credits)

本專案為人類工程師與 AI 助理共同協作的實驗性專案：

- **👨‍💻 Tonny Chiu** ([@tonnychiulab](https://github.com/tonnychiulab))
  - 專案發起、需求與架構決策、功能審查。
- **🤖 Antigravity (AGY)**
  - AI 程式設計助理，負責輔助規格撰寫、代碼實現、WASM 網頁調試與自動化流程設置。

### 🧠 協同開發模型紀錄 (AI Models Used)

在開發與迭代過程中，嘗試了多階段模型協同：

1. **Claude Opus (Thinking)**：
   - 用於前期語義理解、架構推導與 OpenSpec SDD 規範文件規劃（`termforge-engine` 與 `add-web-wasm-support`）。
2. **Gemini Pro & Flash**：
   - 作為子代理 (Subagents) 協助研究 OpenSpec 規範與參考案例。
3. **Gemini Flash (Medium)**：
   - 負責核心引擎代碼編寫、WASM 移植、CRT 網頁端整合與部署自動化。

*本專案為持續學習與改進的實驗作品，若有任何建議或發現問題，非常歡迎提交 Issue 或 PR 交流指教！*

---

## 📜 開源許可證 (License)

本專案採用 [MIT OR Apache-2.0](LICENSE) 雙重許可證授權。


