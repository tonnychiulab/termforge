# Proposal: In-Page Controls & Tactical Abilities Cheat Sheet (add-in-page-controls-cheat-sheet)

## Why

With the introduction of the 9-key flight cluster (`QWE / ASD / ZXC`), active braking, lateral strafing, 180° flip, hyperspace jump, and EMP smart bombs, players unfamiliar with the codebase or arcade nuances may miss these high-utility mechanics if they only use standard Arrow keys and Space.

**User Pain Points & Solution:**
1. **Discoverability:** Players opening the live demo link from social shares or messages need immediate, ambient visual guidance without navigating external documentation.
2. **Device Agnostic Guidance:** Desktop users need keyboard shortcut hints; mobile users need clarity on what each on-screen virtual pad button executes.
3. **Non-Intrusive Layout:** Displaying an ambient, retro-styled cheat sheet beneath the CRT monitor ensures zero obstruction of the active 80×24 gameplay viewport while maintaining aesthetic continuity.

## What Changes

- Add a dedicated, responsive **Controls Cheat Sheet** section directly beneath the CRT monitor frame in `dist/index.html`.
- Styled using CRT phosphor aesthetics (subtle neon borders, monospace font, keyboard-styled badge glyphs).
- Primary language: **Traditional Chinese (繁體中文)** with English secondary labels.
- Clear categorization:
  - 🚀 **飛行機動 (Flight Maneuvers):** W 前進推進、S 主動煞車、A/D 旋轉、Q/E 左右側移、X 180° 翻轉
  - ⚡ **戰術特技 (Tactical Skills):** Space 主砲射擊、B/F 全屏 EMP 炸彈、Z/H 超空間瞬移
  - ⚙️ **系統控制 (System):** P 暫停/繼續、M 音效開關、R/Enter 重新開始

## Capabilities

### Added
- `in-page-cheat-sheet`: Semantic, responsive HTML/CSS badge grid beneath the CRT monitor frame.
- `mobile-legend-alignment`: Virtual button icons and colors matching the cheat sheet legend.

## Impact

- **Affected Files**:
  - `dist/index.html`
  - `dist/crt.css`
- **Breaking Changes**: None. Purely additive presentation enhancement.
