# Delta Spec: In-Page Controls & Keybinding Cheat Sheet

## ADDED Requirements

---

### Requirement: Ambient Control Legend Layout
The web presentation layer SHALL render a dedicated controls cheat sheet card container directly below the primary CRT monitor.

#### Scenario: Visual inspection on desktop viewports
- **GIVEN** a desktop viewport width $\ge 768$px
- **WHEN** the page finishes loading
- **THEN** a segmented cheat sheet card container SHALL be visible beneath the CRT monitor frame
- **AND** the container SHALL display three distinct category groupings: "🚀 飛行機動 (Flight)", "⚡ 戰術特技 (Combat)", and "⚙️ 系統控制 (System)"
- **AND** all key badges SHALL use `<kbd>` styling with high-contrast neon borders

---

### Requirement: Traditional Chinese First with English Secondary Labels
All command badges and descriptors SHALL lead with Traditional Chinese terminology followed by concise English translations.

#### Scenario: Key descriptor presentation
- **GIVEN** the "Active Braking" key badge
- **WHEN** rendered on screen
- **THEN** the badge SHALL read `<kbd>S</kbd> / <kbd>↓</kbd> 主動煞車 (Brake)`
- **AND** the "Lateral Strafe" badge SHALL read `<kbd>Q</kbd> / <kbd>E</kbd> 左右側移 (Strafe)`
- **AND** the "EMP Bomb" badge SHALL read `<kbd>B</kbd> / <kbd>F</kbd> 全屏 EMP 炸彈 (Smart Bomb)`

---

### Requirement: Responsive Layout and Mobile Virtual Pad Legend
The cheat sheet SHALL wrap cleanly on smaller mobile screens and visually correspond to the virtual touch pad buttons.

#### Scenario: Mobile viewport reflow
- **GIVEN** a mobile viewport width $< 768$px
- **WHEN** rendered on a smartphone screen
- **THEN** the cheat sheet cards SHALL stack vertically or wrap compactly
- **AND** touch button colors (e.g., Pink for EMP, Yellow for Thrust, Red for Brake) SHALL match the badge legend colors
