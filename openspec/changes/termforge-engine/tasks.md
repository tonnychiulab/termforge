# Implementation Checklist: TermForge Engine & Asteroids Demo

## 1. Project Scaffolding & Build Foundation
- [x] 1.1 Initialize Cargo workspace with `termforge` (engine crate) and `asteroids` (demo binary)
- [x] 1.2 Configure dependencies (`termion ^3.0`, dev-dependency `criterion`) and workspace profile optimizations (`opt-level = 3`, `lto = true`)
- [x] 1.3 Setup panic hook in `termforge::terminal::guard` to restore canonical terminal state and unhide cursor on panic
- [x] 1.4 Implement `termforge::math` primitives (`Vec2`, `Rect`, `Transform`, angle/trigonometry helpers) with unit tests

## 2. Core Engine & ECS Architecture
- [x] 2.1 Implement `GenerationalArena` storage for entity lifecycle management (`EntityId`, generation counters)
- [x] 2.2 Implement component storage containers and type-safe `ComponentPool` registrations
- [x] 2.3 Implement system execution pipeline supporting sequential system dispatch with `World` and `dt` contexts
- [x] 2.4 Build fixed-timestep game loop with accumulator, delta clamp (250ms), and frame limiter (60 FPS default)
- [x] 2.5 Unit test ECS entity recycling, generation invalidation, and component query consistency

## 3. Double-Buffered Rendering Engine
- [x] 3.1 Implement `Cell`, `Color` (ANSI 16, 256, RGB truecolor), and `StyleModifier` bitflags
- [x] 3.2 Implement `Buffer2D` grid abstraction with bounds checking and index math
- [x] 3.3 Implement `DoubleBuffer` structure with `diff()` generator for dirty-cell detection
- [x] 3.4 Build ANSI SGR sequence serializer with cursor jump batching and minimal escape overhead
- [x] 3.5 Implement terminal size detection and dynamic buffer resize handler on `SIGWINCH`
- [x] 3.6 Benchmark rendering diff throughput under 10%, 50%, and 100% cell change scenarios

## 4. Input Subsystem
- [x] 4.1 Implement raw keyboard stream reader wrapping `termion::input::TermRead`
- [x] 4.2 Build non-blocking asynchronous event channel connecting terminal input thread to engine loop
- [x] 4.3 Implement `InputState` tracker with `is_pressed`, `just_pressed`, and `just_released` queries
- [x] 4.4 Implement ring-buffered input history for combo detection and replay debugging
- [x] 4.5 Unit test key press transitions across sequential game loop frames

## 5. Collision Detection & Spatial Partitioning
- [x] 5.1 Implement AABB vs AABB and Circle vs Circle narrowphase intersection tests
- [x] 5.2 Implement hybrid Circle vs AABB contact test
- [x] 5.3 Implement `Quadtree` 2D spatial index with configurable max depth and bucket capacity
- [x] 5.4 Implement collision layer and collision mask bitfield filtering
- [x] 5.5 Benchmark Quadtree broadphase query performance with 500 moving entities against brute-force checks

## 6. Animation & Particle Subsystem
- [x] 6.1 Implement `Tween<T>` with 30+ easing functions (Linear, Quad, Cubic, Sine, Exp, Elastic, Bounce)
- [x] 6.2 Implement frame-based ASCII sprite animator with `PlayMode` (Loop, Once, PingPong)
- [x] 6.3 Implement `Particle` struct and object-pooling `ParticleEmitter`
- [x] 6.4 Implement particle color gradients and lifetime alpha/character fade transitions
- [x] 6.5 Unit test tween interpolation values and particle pool recycle behavior

## 7. Audio & Sound Pipeline
- [x] 7.1 Define `AudioBackend` trait with `play_sound(sound_id)` and `set_volume()` methods
- [x] 7.2 Implement `TerminalBellBackend` producing ASCII bell sequences with rate-limiting cooldown
- [x] 7.3 Implement frame-level `SoundEventQueue` dispatched at the end of each game tick

## 8. Scene & Camera Management
- [x] 8.1 Implement stack-based `SceneManager` with push, pop, replace lifecycle triggers (`on_enter`, `on_exit`, `on_pause`, `on_resume`)
- [x] 8.2 Implement animated scene transitions (Fade, Slide, Dissolve)
- [x] 8.3 Implement `Camera2D` with viewport clipping, world-to-screen matrix, and smooth target tracking lerp

## 9. Asteroids Demo Game Implementation
- [x] 9.1 Scaffold `asteroids` demo entry point integrating `termforge::Engine`
- [x] 9.2 Implement `Ship` component, inertia-based thrust physics, and directional rotation controls
- [x] 9.3 Implement screen edge wrapping system for all in-bounds entities
- [x] 9.4 Implement bullet weapon system with fire rate cooldown and lifetime decay
- [x] 9.5 Implement asteroid spawning, floating rotation, and size splitting hierarchy (Large -> Medium -> Small)
- [x] 9.6 Implement particle explosion effects on asteroid and ship destruction
- [x] 9.7 Implement player death, respawn delay, and 3-second invulnerability blinking state
- [x] 9.8 Implement HUD overlay rendering score, wave number, and heart life indicators
- [x] 9.9 Implement Game Over state with restart prompt and progressive wave difficulty scaling

## 10. Verification, Benchmarks & Validation
- [x] 10.1 Run full test suite (`cargo test --workspace`) ensuring 0 failures
- [x] 10.2 Run criterion benchmarks on render diff, ECS query, and Quadtree broadphase
- [x] 10.3 Verify stable 60 FPS performance in full asteroid stress test (50+ active asteroids, 100+ particles)
- [x] 10.4 Run `openspec validate` to confirm strict SDD artifact compliance
