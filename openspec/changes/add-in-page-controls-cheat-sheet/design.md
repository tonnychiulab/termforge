# Design: Controls Cheat Sheet Layout & Component Design

## Context

The CRT display occupies up to 650px height with dark phosphor aesthetics. Placing a keybinding legend directly into the terminal canvas would consume precious character cells from the 80×24 game grid. Housing the legend in the DOM wrapper maintains clean separation between the WASM simulation and explanatory UI.

---

## Architectural Decisions

### Decision 1: Flexbox Grid with Category Grouping
- **Structure:**
  - Container `.controls-cheat-sheet` styled as a brushed dark metal cockpit panel.
  - Three flex column categories:
    1. **飛行機動 (Flight):** Forward, Brake, Turn, Strafe, 180° Flip.
    2. **戰術特技 (Combat):** Fire, EMP Shockwave, Hyperspace Jump.
    3. **系統功能 (System):** Pause, Mute, Restart.
- **Rationale:** Prevents horizontal information overload; logical grouping allows players to digest mechanics hierarchically.

### Decision 2: Retro Terminal Micro-Typography
- **Styling:**
  - `<kbd>` elements styled with dark slate background `#161b22`, 1px borders in phosphor colors, and subtle outer glow `box-shadow`.
  - Color-coding by function:
    - Gold / Amber for Propulsion & Turn (`--crt-amber`)
    - Cyan / Neon Green for Combat (`--crt-cyan`, `--crt-green`)
    - Magenta / Crimson for Defense & Emergency (`#df80ff`, `#ff6666`)

---

## Layout Wireframe

```text
+-------------------------------------------------------------+
|                     [ CRT MONITOR FRAME ]                   |
|                        (80x24 Screen)                       |
+-------------------------------------------------------------+
                               |
                               v
+-------------------------------------------------------------+
|               🕹️ 戰術操作指南 (TACTICAL FLIGHT MANUAL)       |
+-------------------------------------------------------------+
| [🚀 飛行機動]          | [⚡ 戰術特技]         | [⚙️ 系統控制]     |
| [W/↑] 推進加速 (Thrust)| [Space] 電漿砲 (Fire) | [P] 暫停 (Pause)  |
| [S/↓] 主動煞車 (Brake) | [B/F]   全屏EMP炸彈   | [M] 靜音 (Mute)   |
| [A/D] 左右旋轉 (Turn)  | [Z/H]   超空間瞬移    | [R] 重生 (Restart)|
| [Q/E] 左右側移 (Strafe)|                      |                   |
| [X]   180°回頭 (Flip)  |                      |                   |
+------------------------+----------------------+-------------------+
```
