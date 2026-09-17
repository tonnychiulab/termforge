# 🦀 TermForge — High-Performance Pure Rust Terminal Game Engine

A modern, zero-dependency, 60 FPS terminal game engine built in pure Rust, complete with an arcade-style **Asteroids** demo game.

Built following the **[OpenSpec](https://github.com/Fission-AI/openspec)** Spec-Driven Development (SDD) standard.

👉 **[🎮 點此立即線上遊玩 (Play Live Web Demo)](https://tonnychiulab.github.io/termforge/)** (免安裝，支援電腦鍵盤與手機觸控！)

---

## 🌟 Features & Architecture

- **🎬 Double-Buffered Cell Grid Renderer:** Eliminates screen flicker entirely. Diff algorithm calculates dirty cells and batches ANSI escape sequences, writing only modified characters with minimal cursor jumps.
- **🧩 Generational Arena ECS:** Type-safe Entity-Component-System with $O(1)$ allocation/deallocation and generational counter indexing to prevent stale references (ABA problem).
- **⏱️ Fixed-Timestep Game Loop:** Canonical Glenn Fiedler accumulator pattern (60 Hz physics update, decoupled render loop, delta-time clamping to prevent spiral of death).
- **📍 Quadtree Spatial Partitioning:** $O(N \log N)$ broadphase collision indexing supporting AABB and circular colliders with bitmask layer filtering.
- **✨ Tween & Particle Systems:** Object-pooled particle emitter with 30+ easing functions, ASCII sprite animations, and smooth color/character fade transitions.
- **🎮 Multi-Threaded Non-Blocking Input:** Asynchronous stdin reader tracking `is_pressed`, `just_pressed`, and `just_released` state transitions.
- **🛡️ RAII Panic-Safe Terminal Guard:** Automatically handles raw mode and alternate screen buffer. Installs a global panic hook that guarantees canonical terminal restoration even on runtime crashes.
- **🔊 Audio Abstraction:** Pluggable `AudioBackend` trait with rate-limited terminal bell (`\x07`) driver out of the box.

---

## 🚀 Quick Start

### Prerequisites
- Rust 2021 edition (`cargo`, `rustc`)

### Run the Asteroids Demo Game
```bash
cargo run --release -p asteroids
```

### Run Full Test Suite
```bash
cargo test --workspace
```

---

## 🎮 Asteroids Controls

| Key | Action |
| :--- | :--- |
| **W** / **Up Arrow** | Thrust forward (with inertia & particle exhaust) |
| **A** / **Left Arrow** | Rotate ship counter-clockwise |
| **D** / **Right Arrow** | Rotate ship clockwise |
| **Space** | Fire plasma bullets |
| **R** / **Enter** | Restart after Game Over |
| **Ctrl + Q** / **Esc** | Quit game |

---

## 📂 OpenSpec SDD Documentation

All design decisions and behavioral specifications adhere strictly to OpenSpec:

- [`openspec/config.yaml`](openspec/config.yaml): Project tech stack rules and artifact requirements.
- [`proposal.md`](openspec/changes/termforge-engine/proposal.md): Problem definition, capabilities, and impact.
- [`specs/engine-core.md`](openspec/changes/termforge-engine/specs/engine-core.md): Core loop, ECS, and double-buffer render specs.
- [`specs/subsystems.md`](openspec/changes/termforge-engine/specs/subsystems.md): Animation, Quadtree collision, input, audio, scene specs.
- [`specs/demo-game.md`](openspec/changes/termforge-engine/specs/demo-game.md): Asteroids arcade requirements and scenarios.
- [`design.md`](openspec/changes/termforge-engine/design.md): In-depth architectural decisions, data layouts, and risk mitigations.
- [`tasks.md`](openspec/changes/termforge-engine/tasks.md): Complete GFM implementation checklist.
