# Delta Spec: Engine Core — Game Loop, ECS, and Rendering Pipeline

## ADDED Requirements

---

### Requirement: Engine Lifecycle Management
The engine SHALL manage the complete terminal lifecycle including entering raw mode, enabling alternate screen, hiding the cursor on startup, and restoring all terminal state on shutdown — even after a panic.

#### Scenario: Clean engine startup and shutdown
- **GIVEN** a standard terminal with default settings
- **WHEN** the engine is initialized via `Engine::new(config)`
- **THEN** the terminal SHALL enter raw mode with no echo
- **AND** the alternate screen buffer SHALL be activated
- **AND** the cursor SHALL be hidden

#### Scenario: Graceful shutdown after panic
- **GIVEN** a running engine instance
- **WHEN** the game logic panics with an unrecoverable error
- **THEN** the engine SHALL catch the panic via a panic hook
- **AND** the terminal SHALL be restored to its original state before the panic propagates
- **AND** the panic message SHALL be printed to the restored terminal

---

### Requirement: Fixed-Timestep Game Loop
The engine SHALL execute a fixed-timestep game loop where physics/logic updates occur at a constant rate (default 60 Hz) independent of rendering frame rate.

#### Scenario: Consistent updates under varying render performance
- **GIVEN** an engine configured with a 60 Hz update rate
- **WHEN** a single render frame takes 32ms (slower than the 16.67ms budget)
- **THEN** the engine SHALL execute 2 update ticks to compensate
- **AND** the accumulated time remainder SHALL be carried to the next frame
- **AND** the render interpolation factor SHALL be passed to the render systems

#### Scenario: Frame rate limiting prevents CPU spinning
- **GIVEN** an engine configured with a 60 FPS target
- **WHEN** update and render complete in 5ms (under budget)
- **THEN** the engine SHALL sleep for the remaining ~11.67ms
- **AND** the actual sleep duration SHALL be measured and compensated in the next frame

#### Scenario: Update spiral-of-death protection
- **GIVEN** an engine running at 60 Hz update rate
- **WHEN** a frame takes longer than 250ms (e.g., due to OS scheduling)
- **THEN** the engine SHALL clamp the accumulated delta to a maximum of 250ms
- **AND** at most 15 update ticks SHALL be executed in a single frame

---

### Requirement: Entity-Component-System Architecture
The ECS SHALL use generational arena allocation for entities, providing O(1) entity creation, destruction, and component access with dangling-reference safety.

#### Scenario: Entity creation and component attachment
- **GIVEN** an empty ECS world
- **WHEN** a new entity is spawned with components `Position(10.0, 20.0)` and `Velocity(1.0, 0.0)`
- **THEN** the entity SHALL receive a unique `EntityId` with generation counter 0
- **AND** both components SHALL be retrievable by the entity's ID

#### Scenario: Entity destruction with generational safety
- **GIVEN** an entity with `EntityId { index: 5, generation: 0 }`
- **WHEN** the entity is destroyed
- **AND** a new entity is spawned that reuses slot index 5
- **THEN** the new entity SHALL have `EntityId { index: 5, generation: 1 }`
- **AND** any attempt to access components using the old ID (generation 0) SHALL return `None`

#### Scenario: Component query iteration
- **GIVEN** a world containing 100 entities, 60 of which have both `Position` and `Velocity` components
- **WHEN** a system queries for entities with `(Position, Velocity)`
- **THEN** the query SHALL iterate over exactly 60 entities
- **AND** the iteration SHALL provide mutable references to the queried components

#### Scenario: System registration and execution order
- **GIVEN** three systems registered: `InputSystem`, `PhysicsSystem`, `RenderSystem`
- **WHEN** a single game loop tick executes
- **THEN** systems SHALL execute in their registered order
- **AND** each system SHALL receive a shared reference to the world and a delta-time value

---

### Requirement: Double-Buffered Terminal Renderer
The renderer SHALL maintain two cell buffers (front and back) and compute a minimal diff to write only changed cells to the terminal output.

#### Scenario: Initial full-screen render
- **GIVEN** an empty front buffer and a back buffer with game content
- **WHEN** the first render frame is flushed
- **THEN** every cell in the back buffer SHALL be written to the terminal
- **AND** the front buffer SHALL be swapped to become the current state

#### Scenario: Partial update with dirty-rect optimization
- **GIVEN** a 120×40 terminal where only 5 cells have changed between frames
- **WHEN** the frame is flushed
- **THEN** only the 5 changed cells SHALL be written to terminal output
- **AND** the cursor SHALL be repositioned only when cells are non-adjacent
- **AND** total bytes written SHALL be proportional to changed cells, not screen size

#### Scenario: Color and style support
- **GIVEN** a cell with foreground color `RGB(255, 100, 0)`, background color `RGB(0, 0, 50)`, and bold+underline style
- **WHEN** the cell is rendered to the terminal
- **THEN** the output SHALL contain the appropriate ANSI SGR escape sequences
- **AND** style changes SHALL be batched to minimize escape sequence overhead

#### Scenario: Terminal resize handling
- **GIVEN** a running engine with a 120×40 terminal
- **WHEN** the user resizes the terminal to 80×24
- **THEN** the engine SHALL detect the SIGWINCH signal (or equivalent)
- **AND** both buffers SHALL be reallocated to the new dimensions
- **AND** a full redraw SHALL be triggered on the next frame

---

### Requirement: Cell and Glyph Representation
Each cell in the render buffer SHALL store a character (Unicode scalar value), foreground color, background color, and style attributes as a compact struct.

#### Scenario: Unicode wide-character rendering
- **GIVEN** a cell containing the character `'龍'` (CJK ideograph, display width 2)
- **WHEN** the cell is placed at column 10
- **THEN** the cell SHALL occupy columns 10 and 11
- **AND** column 11 SHALL be marked as a continuation cell (not independently addressable)

#### Scenario: Default cell state
- **GIVEN** a newly allocated render buffer
- **WHEN** no content has been drawn
- **THEN** every cell SHALL contain a space character `' '`
- **AND** foreground and background colors SHALL be `Color::Reset` (terminal default)
- **AND** no style attributes SHALL be active

---

### Requirement: Coordinate System
The engine SHALL use a Cartesian coordinate system with origin (0, 0) at the top-left corner of the terminal, X increasing rightward, and Y increasing downward.

#### Scenario: World-to-screen coordinate mapping
- **GIVEN** a game entity at world position `(50.5, 20.3)`
- **WHEN** the entity is rendered with no camera offset
- **THEN** the entity SHALL be drawn at terminal cell `(50, 20)` (truncated to integer)

#### Scenario: Sub-cell precision in physics
- **GIVEN** an entity with position `(10.7, 5.3)` and velocity `(0.3, 0.0)` per tick
- **WHEN** one physics tick of 1/60s executes
- **THEN** the position SHALL update to `(11.0, 5.3)` with floating-point precision
- **AND** rendering SHALL display the entity at cell `(11, 5)`
