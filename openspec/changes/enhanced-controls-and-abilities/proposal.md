# Proposal: Enhanced Controls and Combat Abilities (enhanced-controls-and-abilities)

## Why

In the baseline Asteroids implementation, ship control is limited to classical rotate (A/D) and forward thrust (W), with Space for firing. While faithful to the 1979 arcade edition, this rigid movement model creates high friction on modern keyboards and limits tactical maneuvering during high-density asteroid swarms.

**User Pain Points & Needs:**
1. **Lack of Deceleration:** Players frequently crash due to uncontrollable inertia without a dedicated braking mechanism.
2. **Limited Evasive Maneuvers:** No strafing or quick turnaround forces cumbersome 180° manual panning when fleeing or targeting split asteroid fragments.
3. **Emergency Recourse:** No emergency panic-button ability (such as a smart bomb or hyperspace jump) when boxed in by multiple large asteroids.
4. **Game Ergonomics:** Missing pause (<kbd>P</kbd>) and mute (<kbd>M</kbd>) toggles degrades player convenience in browser/terminal environments.

## What Changes

We introduce the "Modern Space Fighter" layout utilizing the 9-key cluster (`QWE / ASD / ZXC`), arrow keys, and dedicated ability triggers:
- **Active Braking (S / Down):** Active reverse deceleration for precise flight control.
- **Lateral Strafing (Q / E):** Perpendicular port/starboard thrusters enabling circular dogfighting.
- **Instant 180° Flip (X):** Instant turnaround to counter rearward threats.
- **Hyperspace Teleport (Z / H):** Instant jump to a random safe position on screen.
- **Smart Bomb / EMP Blast (B / F):** Limited-use shockwave clearing close-range threats with visual particle rings.
- **Ergonomic Controls (P / M):** Full pause state machine and audio muting.

## Capabilities

### Added
- `ship-maneuver-suite`: Braking, lateral strafing vectors, and 180° yaw flip.
- `special-abilities`: Hyperspace randomized safe-spawn & EMP blast radius destruction.
- `game-state-controls`: Pause overlay modal & audio mute toggle in both native terminal and WASM web player.

## Impact

- **Affected Modules**:
  - `crates/termforge/src/input.rs`
  - `games/asteroids/src/main.rs`
  - `crates/termforge_web/src/lib.rs`
  - `dist/index.html` (Add new buttons to mobile virtual pad)
- **Breaking Changes**: None. Previous forward thrust and rotation controls remain fully functional.
