# Proposal: TermForge — A Full-Featured Terminal Game Engine in Rust

## Why

The terminal remains one of the most universally accessible computing interfaces. Every developer machine, server, and embedded system has a terminal. Yet the ecosystem for building interactive, real-time terminal applications — particularly games — is fragmented and underpowered.

**Current pain points:**

1. **No cohesive engine exists.** Developers wanting terminal games must glue together disparate crates (`crossterm`, `tui`, custom loops) with no unified architecture. There is no "Bevy for the terminal."
2. **Performance is an afterthought.** Most terminal game implementations naively redraw the entire screen every frame, producing visible flicker and wasting CPU cycles. Professional techniques like double-buffering and dirty-rect optimization are rarely applied.
3. **Systems programming showcase gap.** The Rust ecosystem lacks a project that demonstrates ECS architecture, spatial indexing, fixed-timestep game loops, and particle systems — all within the constraint of a terminal's character grid.
4. **Educational value is enormous.** A terminal game engine strips away GPU abstraction layers, making core game engine concepts (render pipelines, collision broadphase/narrowphase, input buffering, scene graphs) directly observable and debuggable.

TermForge fills this gap: a production-quality, zero-dependency (no C linkage) Rust terminal game engine that proves systems-level engineering can be done with elegance.

## What Changes

TermForge introduces a complete game engine targeting terminal rendering, built from scratch with the following behavioral changes to the Rust terminal ecosystem:

- **Unified engine API** — A single `Engine::new().run(game)` entry point that handles terminal setup, game loop, input, rendering, and teardown automatically.
- **ECS-first architecture** — All game objects are entities with composable components, processed by parallel-safe systems. No inheritance hierarchies.
- **Flicker-free rendering** — Double-buffered cell grid with dirty-rect tracking. Only changed cells are written to the terminal, achieving 60 FPS on standard terminals.
- **Physics-grade collision** — AABB and circle collision primitives with Quadtree spatial partitioning for O(n log n) broadphase performance.
- **Rich animation** — Tweening library with 30+ easing functions, sprite sheet animation, and GPU-free particle effects.
- **Scene management** — Stack-based scene system with animated transitions (fade, slide, dissolve).
- **Audio architecture** — Trait-based audio backend with terminal bell fallback and optional system audio integration.
- **Complete demo** — A fully playable Asteroids clone showcasing every subsystem working in concert.

## Capabilities

### Added
- `termforge::engine` — Core engine with fixed-timestep game loop and configurable frame rate
- `termforge::ecs` — Generational arena ECS with component storage, entity queries, and system scheduling
- `termforge::render` — Double-buffered terminal renderer with dirty-rect optimization, 256-color and truecolor support, Unicode glyph rendering
- `termforge::animation` — Tweening engine (30+ easing curves), keyframe animation, particle emitter with configurable behaviors
- `termforge::collision` — AABB and circle collision detection, Quadtree spatial index, collision layers and masks
- `termforge::input` — Raw-mode keyboard input, key mapping, combo detection (e.g., Ctrl+Arrow), input buffering and replay
- `termforge::audio` — Audio backend trait, terminal bell integration, sound event queue
- `termforge::scene` — Scene stack manager, transition animations, camera/viewport system
- `termforge::math` — Vec2, Rect, Transform, and interpolation utilities
- `termforge_asteroids` — Complete Asteroids game demonstrating all subsystems

## Impact

- **Affected Specifications**: All specifications are new (greenfield project)
- **Dependencies**:
  - `termion` ^3.0 — Raw terminal access, ANSI control, input events
  - `criterion` (dev) — Performance benchmarking
  - No other runtime dependencies (zero-dep philosophy)
- **Performance Targets**:
  - Rendering: ≥60 FPS at 120×40 terminal size
  - Collision broadphase: ≤1ms for 500 entities
  - Input latency: ≤2ms from keypress to game state update
  - Memory: ≤10MB RSS for Asteroids demo with 200 entities
- **Breaking Changes**: None (greenfield)
