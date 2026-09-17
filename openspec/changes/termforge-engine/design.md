# Design: TermForge Architecture & System Engineering

## Context

Terminal user interfaces (TUIs) have evolved substantially with modern libraries like `ratatui` and `crossterm`. However, TUI libraries are fundamentally optimized for document-like, event-driven layouts (widgets, scrolling lists, key-press forms), not for real-time, tick-synchronized 60 FPS interactive graphical simulations.

Game development in terminal environments requires solving distinct systems challenges:
- **I/O Bandwidth Bottlenecks:** Writing raw ANSI escape sequences over pseudo-terminals (pty/tty) consumes non-trivial CPU and terminal emulator rendering time. Naive whole-screen flushes cause severe tearing and frame drops.
- **Tick Non-Determinism:** Terminal input arrives irregularly; coupling game physics directly to render cycles creates erratic motion and physics tunneling.
- **Cache Locality in High-Entity Simulations:** Classic object-oriented hierarchies scatter state across heap pointers, destroying cache lines in simulations with hundreds of particles and colliders.

TermForge addresses these challenges from the ground up in Rust without C runtime dependencies.

---

## Goals / Non-Goals

### Goals
- Deliver a complete, modular, standalone terminal game engine framework in pure Rust (2021 edition).
- Achieve consistent 60 FPS update and render performance at 120×40 resolution with zero terminal tearing.
- Implement an explicit Entity-Component-System (ECS) pattern using generational indexing for memory safety without garbage collection overhead.
- Provide production-ready sub-systems: fixed-timestep game loop, double-buffered cell grid renderer, Quadtree-accelerated collision detection, tween/particle animation system, raw keyboard input buffer, scene state machine, and audio abstractions.
- Ship a fully playable, polished Asteroids demo showcasing all subsystems acting in concert.

### Non-Goals
- Supporting legacy Windows console (`cmd.exe` without ANSI emulation). TermForge targets ANSI-compliant VT100/Xterm environments via `termion`.
- Software rasterization of 3D polygonal geometry (focus is 2D character-grid spatial gaming).
- Full hardware-level sound synthesis within the core engine crate (audio backend is trait-based, providing terminal bell out of the box with hookable drivers).

---

## Architecture Overview & Module Dependencies

```text
+-------------------------------------------------------------+
|                     termforge_asteroids                     |
|                   (Game Logic & Assets)                     |
+-------------------------------------------------------------+
                              |
                              v
+-------------------------------------------------------------+
|                      termforge::engine                      |
|           (Main Loop, Timestep, System Dispatcher)          |
+-------------------------------------------------------------+
   |          |           |            |           |        |
   v          v           v            v           v        v
+------+  +-------+  +---------+  +---------+  +-------+ +------+
| ecs  |  | render|  |collision|  |animation|  | input | |scene |
+------+  +-------+  +---------+  +---------+  +-------+ +------+
   |          |           |            |           |        |
   +----------+-----------+------------+-----------+--------+
                              |
                              v
                      +---------------+
                      |termforge::math|
                      +---------------+
```

---

## Architectural Decisions

### Decision 1: Generational Arena Allocator for ECS
- **Decision:** Build a lightweight, type-safe Generational Arena ECS instead of importing heavy frameworks like `bevy_ecs`.
- **Rationale:** Keeps compilation times ultra-fast, zero foreign C-bindings, zero complex proc-macro bloat, while maintaining $O(1)$ allocation/deallocation and avoiding the ABA reference invalidation problem.
- **Alternatives Considered:**
  - *Sparse-set ECS (EnTT/Shipyard style):* Higher implementation complexity for terminal game scales; generational arena provides identical amortized efficiency for $\le 2000$ active entities.
  - *Classic OO Trait Objects (`Vec<Box<dyn Entity>>`):* Discarded due to pointer chasing, cache misses, and lack of component composability.

### Decision 2: Double-Buffered Cell Grid with Dirty-Rect & Skip Optimization
- **Decision:** Maintain two flat 1D arrays representing the terminal character matrix (`front_buffer` and `back_buffer`), computing diffs on flush and batching cursor positioning.
- **Rationale:** Terminal I/O is the primary bottleneck. If only 3% of the cells changed between ticks, sending only updated cells with ANSI cursor jumps reduces stdout writes by up to 90%, eliminating flicker completely.
- **Alternatives Considered:**
  - *Full redraw via `\x1b[2J` (Clear Screen):* Creates unacceptable strobe-like flickering at 60 FPS.
  - *Differential string diffing (Rope / Myers):* Overly complex for a fixed 2D grid; direct cell-to-cell comparison is trivially $O(W \times H)$.

### Decision 3: Decoupled Fixed Timestep Loop with Accumulator
- **Decision:** Adopt the canonical Glenn Fiedler ("Fix Your Timestep!") architecture:
  ```rust
  let mut accumulator = Duration::ZERO;
  let dt = Duration::from_micros(16_667); // 60Hz physics
  ```
- **Rationale:** Physics simulation (velocities, damping, collision resolution) remains 100% deterministic regardless of terminal output latency or OS thread scheduling spikes.
- **Alternatives Considered:**
  - *Variable delta-time ($dt$ per frame):* Causes tunneling in collision detection when frame drops occur, leading to non-reproducible glitches.

### Decision 4: Quadtree Spatial Indexing for Collision Broadphase
- **Decision:** Partition the bounded 2D game universe with a dynamic Quadtree reconstructed or refitted every physics tick.
- **Rationale:** Reduces collision pair tests from $O(N^2)$ to $O(N \log N)$. With 100 asteroids, 50 bullets, and 200 particles, $N^2$ requires up to 60,000 checks per frame; Quadtree limits checks to localized clusters ($< 800$ pairs).
- **Alternatives Considered:**
  - *Uniform Spatial Hash Grid:* Fast, but requires manual tuning of cell dimensions when entity sizes vary substantially (e.g., Large Asteroid vs Bullet).
  - *Brute force:* Acceptable for $N < 30$, totally unusable for bullet-hell or particle scenarios.

### Decision 5: Trait-Based Audio Backend
- **Decision:** Define an `AudioBackend` trait in engine core, implementing `TerminalBellBackend` as zero-dependency default, while offering an extensible event pipeline for external crates (such as `rodio`).
- **Rationale:** Maintains the zero-external-C-dependency ethos for core TermForge while still providing tactile sound feedback via ASCII Bell `\x07` or terminal escape sequence bells.

---

## Memory Management & Data Layout Strategy

1. **Cell Representation:**
   ```rust
   #[derive(Clone, Copy, PartialEq, Eq)]
   pub struct Cell {
       pub ch: char,
       pub fg: Color,
       pub bg: Color,
       pub modifier: Modifier,
   }
   ```
   Packed into a contiguous `Vec<Cell>` of size `width * height`.
2. **Entity Allocation:**
   `EntityId` contains a 32-bit index and a 32-bit generation counter (`u64` total). Reclaimed slots increment generation, instantly invalidating stale handles.
3. **Particle Recycling:**
   Particle emitter utilizes a pre-allocated circular ring buffer / free-list pool to prevent dynamic heap reallocations during intense visual explosions.

---

## Risks and Mitigations

| Risk | Impact | Mitigation Strategy |
| :--- | :--- | :--- |
| **Terminal Escape Lag** | High stdout buffering can freeze terminal or cause input lag | Flush stdout using buffered writer (`BufWriter`) with manual frame flushes; limit max dirty writes. |
| **Terminal Resizing during Tick** | Out-of-bounds cell writes or buffer indexing panics | Catch `SIGWINCH` / check `termion::terminal_size()` each tick; auto-reallocate buffers and schedule full invalidate. |
| **Panics leaving Terminal in Raw State** | Terminal becomes broken/unusable on crash (no echo, invisible cursor) | Install custom `std::panic::set_hook` that restores terminal canonical mode and alternate screen before printing stack trace. |
| **Unicode Width Mismatches** | Emoji / CJK characters occupying 2 terminal columns offset subsequent chars | Track display widths via unicode width heuristics and mark second column as `ContinuationCell`. |
