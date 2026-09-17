# Implementation Checklist: Enhanced Controls and Combat Abilities

## 1. Ship Maneuver Kinematics
- [x] 1.1 Implement active braking (`S` / `Down Arrow`) with reverse deceleration damping in native & WASM
- [x] 1.2 Implement perpendicular lateral strafing (`Q` / `E`) with side-thruster particle emissions
- [x] 1.3 Implement instant 180° turnaround (`X`) with yaw inversion
- [x] 1.4 Implement Hyperspace teleport (`Z` / `H`) with random safe-coordinate relocation & temporary shield

## 2. Special Abilities & Bomb System
- [x] 2.1 Implement EMP Smart Bomb stock (1 per life) triggered by `B` or `F`
- [x] 2.2 Implement 360° circular shockwave particle blast animation on EMP trigger
- [x] 2.3 Implement area-of-effect destruction of nearby asteroids within blast radius

## 3. Game Ergonomics & State Machine
- [x] 3.1 Implement Game Pause toggle (`P`) with centered overlay modal
- [x] 3.2 Implement Audio Mute toggle (`M`) in native bell and web synth
- [x] 3.3 Update HUD to display remaining EMP bombs count and sound status

## 4. Web Virtual Pad & Documentation Updates
- [x] 4.1 Add on-screen touch buttons (Brake, Strafe, Bomb) to mobile CRT frontend in `dist/index.html`
- [x] 4.2 Recompile WebAssembly bundle (`dist/pkg/`) with new features
- [x] 4.3 Update `README.md` keybinding reference tables and deploy to GitHub Pages
