# Delta Spec: Subsystems — Animation, Collision, Input, Audio, Scene

## ADDED Requirements

---

## Animation System

### Requirement: Tween Animation Engine
The animation system SHALL provide a tweening engine that interpolates numeric values over time using configurable easing functions.

#### Scenario: Linear position tween
- **GIVEN** a tween from `x=0.0` to `x=100.0` with duration 2.0 seconds and `EaseFunction::Linear`
- **WHEN** 1.0 second has elapsed
- **THEN** the interpolated value SHALL be `50.0`
- **AND** the tween SHALL report progress as `0.5`

#### Scenario: Ease-in-out cubic tween
- **GIVEN** a tween from `alpha=0.0` to `alpha=1.0` with duration 1.0 second and `EaseFunction::CubicInOut`
- **WHEN** 0.5 seconds have elapsed (midpoint)
- **THEN** the interpolated value SHALL be `0.5`
- **AND** the rate of change SHALL be maximal at the midpoint

#### Scenario: Tween completion callback
- **GIVEN** a tween with an `on_complete` callback registered
- **WHEN** the tween reaches its target value
- **THEN** the callback SHALL be invoked exactly once
- **AND** the tween SHALL be marked as `TweenState::Completed`

### Requirement: Sprite Animation
The animation system SHALL support frame-based sprite animations using sequences of ASCII/Unicode character frames with configurable frame duration.

#### Scenario: Looping sprite animation
- **GIVEN** a sprite animation with frames `['◐', '◓', '◑', '◒']` at 4 FPS
- **WHEN** 1.0 second of game time elapses
- **THEN** each frame SHALL display for exactly 250ms
- **AND** the animation SHALL loop back to frame 0 after frame 3

#### Scenario: One-shot animation
- **GIVEN** a sprite animation configured as `PlayMode::Once` with 3 frames
- **WHEN** all 3 frames have been displayed
- **THEN** the animation SHALL hold on the last frame
- **AND** an `AnimationFinished` event SHALL be emitted

### Requirement: Particle Emitter
The animation system SHALL provide a particle emitter that spawns short-lived visual entities with configurable position, velocity, lifetime, color, and character.

#### Scenario: Explosion particle burst
- **GIVEN** a particle emitter at position `(50, 20)` configured with `burst_count: 30`, `lifetime: 0.5..1.0s`, `speed: 5.0..15.0`, `characters: ['*', '.', '·', '✦']`
- **WHEN** the emitter fires a burst
- **THEN** exactly 30 particles SHALL be spawned
- **AND** each particle SHALL have a random velocity direction (0..2π radians)
- **AND** each particle SHALL have a random speed within `5.0..15.0`
- **AND** each particle SHALL despawn after its individual lifetime expires

#### Scenario: Continuous particle stream
- **GIVEN** a particle emitter configured with `emit_rate: 10` particles per second
- **WHEN** 1.0 second of game time elapses
- **THEN** approximately 10 particles SHALL have been spawned (±2 for frame timing)
- **AND** expired particles SHALL be recycled from a pool, not heap-allocated

#### Scenario: Particle color fade
- **GIVEN** a particle with `start_color: RGB(255, 200, 0)` and `end_color: RGB(100, 0, 0)` and lifetime 1.0s
- **WHEN** 0.5 seconds have elapsed
- **THEN** the particle's rendered color SHALL be approximately `RGB(178, 100, 0)` (linear interpolation)

---

## Collision Detection

### Requirement: AABB Collision Detection
The collision system SHALL detect overlaps between Axis-Aligned Bounding Boxes with O(1) per-pair test complexity.

#### Scenario: Two overlapping AABBs
- **GIVEN** Box A at `(10, 10)` with size `(5, 3)` and Box B at `(13, 11)` with size `(4, 2)`
- **WHEN** collision detection runs
- **THEN** a collision SHALL be reported between A and B
- **AND** the collision result SHALL include the overlap region

#### Scenario: Non-overlapping AABBs
- **GIVEN** Box A at `(0, 0)` with size `(5, 5)` and Box B at `(10, 10)` with size `(3, 3)`
- **WHEN** collision detection runs
- **THEN** no collision SHALL be reported between A and B

### Requirement: Circle Collision Detection
The collision system SHALL detect overlaps between circular colliders using distance-squared comparison (avoiding sqrt for performance).

#### Scenario: Two overlapping circles
- **GIVEN** Circle A at center `(10.0, 10.0)` with radius `5.0` and Circle B at center `(14.0, 10.0)` with radius `3.0`
- **WHEN** collision detection runs
- **THEN** a collision SHALL be reported (distance 4.0 < sum of radii 8.0)

#### Scenario: Circle-AABB hybrid collision
- **GIVEN** a circle at `(10.0, 10.0)` with radius `3.0` and an AABB at `(12, 8)` with size `(4, 4)`
- **WHEN** collision detection runs
- **THEN** a collision SHALL be reported between the circle and the AABB
- **AND** the nearest point on the AABB to the circle center SHALL be computed for contact resolution

### Requirement: Quadtree Spatial Index
The collision system SHALL maintain a Quadtree spatial index to partition the game space and reduce broadphase collision checks from O(n²) to O(n log n).

#### Scenario: Quadtree insertion and subdivision
- **GIVEN** a Quadtree covering a 120×40 region with max capacity 8 per node
- **WHEN** 9 entities are inserted into the same quadrant
- **THEN** the quadrant SHALL subdivide into 4 child nodes
- **AND** entities SHALL be redistributed into the appropriate child nodes

#### Scenario: Broadphase query
- **GIVEN** 500 entities distributed across the game space
- **WHEN** a broadphase collision query is executed for a single entity
- **THEN** only entities in the same or neighboring Quadtree nodes SHALL be tested
- **AND** the query SHALL complete in ≤1ms

#### Scenario: Quadtree rebuild per frame
- **GIVEN** a frame where entities have moved
- **WHEN** the collision broadphase runs
- **THEN** the Quadtree SHALL be rebuilt from scratch each frame (clear + reinsert)
- **AND** the rebuild SHALL complete within the collision system's time budget

### Requirement: Collision Layers and Masks
The collision system SHALL support layer-based filtering where each collider has a `layer` (what it is) and a `mask` (what it collides with), using bitwise operations.

#### Scenario: Layer filtering prevents unnecessary checks
- **GIVEN** Entity A on layer `PLAYER` (bit 0) with mask `ENEMY | ASTEROID` (bits 1,2)
- **AND** Entity B on layer `PLAYER_BULLET` (bit 3) with mask `ENEMY | ASTEROID`
- **WHEN** collision detection runs between A and B
- **THEN** no collision test SHALL be performed (A's mask does not include PLAYER_BULLET)

#### Scenario: Bidirectional layer check
- **GIVEN** Entity A on layer `PLAYER` with mask `ENEMY` and Entity B on layer `ENEMY` with mask `PLAYER`
- **WHEN** collision detection runs
- **THEN** a collision SHALL be checked (A's mask includes B's layer OR B's mask includes A's layer)

---

## Input System

### Requirement: Raw Keyboard Input Capture
The input system SHALL capture keyboard events in raw mode without waiting for Enter, with no echo to the terminal.

#### Scenario: Single key press detection
- **GIVEN** the engine is running in raw mode
- **WHEN** the user presses the `W` key
- **THEN** an `InputEvent::KeyDown(Key::Char('w'))` SHALL be available in the input buffer within 2ms

#### Scenario: Special key detection
- **GIVEN** the engine is running in raw mode
- **WHEN** the user presses the Up Arrow key
- **THEN** an `InputEvent::KeyDown(Key::Up)` SHALL be generated
- **AND** the multi-byte escape sequence SHALL be fully consumed from the input stream

#### Scenario: Modifier key combinations
- **GIVEN** the engine is running
- **WHEN** the user presses `Ctrl+C`
- **THEN** an `InputEvent::KeyDown(Key::Ctrl('c'))` SHALL be generated
- **AND** the engine SHALL NOT terminate (Ctrl+C is intercepted, not forwarded to the OS signal handler)
- **AND** the game MAY handle it as a quit request via its own logic

### Requirement: Input Buffering and Replay
The input system SHALL maintain a ring buffer of recent input events for combo detection and input replay.

#### Scenario: Input buffer stores recent events
- **GIVEN** an input buffer with capacity 64
- **WHEN** 70 key events occur within one second
- **THEN** the most recent 64 events SHALL be stored
- **AND** the oldest 6 events SHALL be discarded

#### Scenario: Combo detection
- **GIVEN** a registered combo pattern `[Key::Up, Key::Up, Key::Down, Key::Down]` with a 500ms window
- **WHEN** the user presses Up, Up, Down, Down within 400ms
- **THEN** a `ComboEvent::Triggered("konami_partial")` SHALL be emitted

### Requirement: Key State Tracking
The input system SHALL track the current state of keys (pressed/released) and provide `is_pressed(key)`, `just_pressed(key)`, and `just_released(key)` queries.

#### Scenario: Distinguishing held from just-pressed
- **GIVEN** the `Space` key was pressed in the previous frame and is still held
- **WHEN** the current frame queries input state
- **THEN** `is_pressed(Key::Char(' '))` SHALL return `true`
- **AND** `just_pressed(Key::Char(' '))` SHALL return `false`
- **AND** `just_released(Key::Char(' '))` SHALL return `false`

---

## Audio System

### Requirement: Audio Backend Trait
The audio system SHALL define an `AudioBackend` trait allowing pluggable audio implementations, with a default terminal bell backend.

#### Scenario: Terminal bell audio
- **GIVEN** the default `BellBackend` is active
- **WHEN** the game triggers `audio.play(SoundEffect::Explosion)`
- **THEN** the terminal bell character (`\x07`) SHALL be written to stdout
- **AND** a cooldown of 100ms SHALL prevent bell spam

#### Scenario: Custom audio backend
- **GIVEN** a user implements `AudioBackend` for a `RodioBackend` (external crate)
- **WHEN** the backend is registered via `engine.set_audio_backend(rodio_backend)`
- **THEN** all subsequent `audio.play()` calls SHALL be routed to the custom backend

### Requirement: Sound Event Queue
The audio system SHALL maintain a per-frame event queue, allowing game systems to enqueue sound effects that are flushed to the backend at frame end.

#### Scenario: Multiple sounds in one frame
- **GIVEN** 3 sound events enqueued in a single frame: `[Hit, Explosion, PowerUp]`
- **WHEN** the frame ends and the audio system flushes
- **THEN** each sound SHALL be dispatched to the audio backend in order
- **AND** the event queue SHALL be cleared for the next frame

---

## Scene Management

### Requirement: Scene Stack
The scene manager SHALL maintain a stack of scenes, where only the top scene receives update ticks, but multiple scenes MAY render (for overlay/pause menu patterns).

#### Scenario: Pushing a pause menu scene
- **GIVEN** a `GameplayScene` is the active scene
- **WHEN** `scene_manager.push(PauseMenuScene)` is called
- **THEN** `PauseMenuScene` SHALL become the active scene receiving updates
- **AND** `GameplayScene` SHALL stop receiving update ticks
- **AND** `GameplayScene` MAY still render (if the pause menu is transparent)

#### Scenario: Popping returns to previous scene
- **GIVEN** a scene stack of `[MainMenu, Gameplay, PauseMenu]`
- **WHEN** `scene_manager.pop()` is called
- **THEN** `PauseMenu` SHALL be removed and its `on_exit()` callback invoked
- **AND** `Gameplay` SHALL resume receiving update ticks
- **AND** `Gameplay.on_resume()` SHALL be called

### Requirement: Scene Transitions
The scene manager SHALL support animated transitions between scenes with configurable duration and effect.

#### Scenario: Fade transition between scenes
- **GIVEN** a transition from `MenuScene` to `GameScene` with `Transition::Fade(0.5)`
- **WHEN** the transition begins
- **THEN** the old scene SHALL gradually fade out over 0.25 seconds
- **AND** the new scene SHALL gradually fade in over the next 0.25 seconds
- **AND** during the transition, neither scene SHALL receive update ticks

### Requirement: Camera and Viewport
The scene system SHALL provide a camera that defines a viewport window into a larger game world, supporting position offset and optional smooth following.

#### Scenario: Camera follows player entity
- **GIVEN** a camera configured with `follow_target: player_entity` and `smoothing: 0.1`
- **WHEN** the player moves from `(50, 20)` to `(60, 20)` instantaneously
- **THEN** the camera SHALL lerp toward `(60, 20)` over multiple frames
- **AND** the camera position SHALL reach within 1 pixel of the target within 30 frames

#### Scenario: Camera clamping to world bounds
- **GIVEN** a game world of 200×100 cells and a terminal viewport of 80×24
- **WHEN** the camera attempts to follow a player at position `(5, 5)`
- **THEN** the camera SHALL clamp so that no area outside the world bounds is visible
- **AND** the minimum camera position SHALL be `(0, 0)`
