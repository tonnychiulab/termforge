# Delta Spec: Asteroids Demo Game

## ADDED Requirements

---

### Requirement: Game Initialization
The Asteroids demo SHALL initialize with a player ship centered on screen, 4 large asteroids at random positions (outside a safe radius from the ship), a score of 0, and 3 lives.

#### Scenario: Game start state
- **GIVEN** the Asteroids demo is launched
- **WHEN** the first frame renders
- **THEN** the player ship SHALL be at the center of the terminal viewport
- **AND** exactly 4 large asteroids SHALL be spawned at random positions
- **AND** no asteroid SHALL spawn within 15 cells of the player ship
- **AND** the HUD SHALL display `Score: 0` and `Lives: ♥♥♥`

---

### Requirement: Player Ship Controls
The player ship SHALL rotate left/right with A/D (or Left/Right arrows), thrust forward with W (or Up arrow), and fire bullets with Space.

#### Scenario: Ship rotation
- **GIVEN** the ship is facing upward (angle 0°)
- **WHEN** the player presses the Right arrow key
- **THEN** the ship SHALL rotate clockwise by 5° per update tick
- **AND** the ship's visual representation SHALL update to reflect the new angle

#### Scenario: Ship thrust with inertia
- **GIVEN** the ship is stationary and facing right (angle 90°)
- **WHEN** the player holds the Up arrow for 30 ticks (0.5s)
- **THEN** the ship SHALL accelerate in the facing direction
- **AND** the ship's velocity SHALL be capped at a maximum speed of 8.0 units/tick
- **AND** a thrust particle effect SHALL be emitted behind the ship

#### Scenario: Ship drift without thrust
- **GIVEN** the ship has velocity `(3.0, 2.0)` and no thrust is applied
- **WHEN** 60 ticks elapse
- **THEN** the ship SHALL gradually decelerate via drag coefficient 0.99 per tick
- **AND** the ship SHALL NOT stop abruptly

---

### Requirement: Screen Wrapping
All game entities (ship, asteroids, bullets) SHALL wrap around screen edges seamlessly.

#### Scenario: Ship wraps horizontally
- **GIVEN** a terminal viewport of 120×40
- **WHEN** the ship moves past column 120 (right edge)
- **THEN** the ship SHALL reappear at column 0 (left edge)
- **AND** the transition SHALL be seamless with no visual pop

#### Scenario: Asteroid wraps vertically
- **GIVEN** an asteroid moving downward at row 40
- **WHEN** it moves past the bottom edge
- **THEN** it SHALL reappear at row 0

---

### Requirement: Bullet Firing
The player SHALL fire bullets in the ship's facing direction. Bullets SHALL have a limited lifetime and travel at a fixed speed.

#### Scenario: Bullet spawn
- **GIVEN** the ship is at position `(60, 20)` facing angle 45°
- **WHEN** the player presses Space
- **THEN** a bullet SHALL spawn at the ship's nose position
- **AND** the bullet's velocity SHALL be in the ship's facing direction at speed 15.0

#### Scenario: Bullet lifetime expiry
- **GIVEN** a bullet with lifetime 1.5 seconds
- **WHEN** 1.5 seconds have elapsed since spawn
- **THEN** the bullet entity SHALL be destroyed
- **AND** no explosion effect SHALL be triggered

#### Scenario: Fire rate limiting
- **GIVEN** the last bullet was fired 100ms ago
- **AND** the fire cooldown is 200ms
- **WHEN** the player presses Space
- **THEN** no bullet SHALL be fired
- **AND** the remaining cooldown SHALL be 100ms

---

### Requirement: Asteroid Behavior
Asteroids SHALL exist in three sizes (Large, Medium, Small) and split into smaller pieces when hit by a bullet.

#### Scenario: Large asteroid destruction
- **GIVEN** a Large asteroid with position `(40, 15)` and velocity `(1.0, 0.5)`
- **WHEN** a bullet collides with it
- **THEN** the Large asteroid SHALL be destroyed
- **AND** 2 Medium asteroids SHALL spawn at its position with randomized velocities (faster than the original)
- **AND** a particle explosion effect SHALL trigger at the collision point
- **AND** the player's score SHALL increase by 20 points

#### Scenario: Medium asteroid destruction
- **GIVEN** a Medium asteroid hit by a bullet
- **WHEN** the collision is processed
- **THEN** 2 Small asteroids SHALL spawn
- **AND** the score SHALL increase by 50 points

#### Scenario: Small asteroid destruction
- **GIVEN** a Small asteroid hit by a bullet
- **WHEN** the collision is processed
- **THEN** no child asteroids SHALL spawn (smallest size)
- **AND** the score SHALL increase by 100 points
- **AND** a smaller particle explosion SHALL trigger

---

### Requirement: Player Death and Respawn
The player ship SHALL be destroyed when colliding with an asteroid, losing one life and respawning with temporary invincibility.

#### Scenario: Ship-asteroid collision
- **GIVEN** the player ship overlaps with an asteroid's collision circle
- **WHEN** collision detection runs
- **THEN** the ship SHALL be destroyed with an explosion effect
- **AND** the life counter SHALL decrease by 1
- **AND** after a 2-second delay, the ship SHALL respawn at screen center

#### Scenario: Invincibility after respawn
- **GIVEN** the ship has just respawned
- **WHEN** the ship is in its 3-second invincibility window
- **THEN** the ship SHALL blink (alternate visible/hidden every 200ms)
- **AND** collisions with asteroids SHALL be ignored
- **AND** the ship SHALL still be able to fire bullets

---

### Requirement: Game Over and Restart
The game SHALL end when all lives are lost and provide a restart option.

#### Scenario: Game over screen
- **GIVEN** the player has 0 lives remaining
- **WHEN** the ship is destroyed
- **THEN** a "GAME OVER" text SHALL display centered on screen
- **AND** the final score SHALL be displayed below
- **AND** the text "Press ENTER to restart or ESC to quit" SHALL be shown

#### Scenario: Level progression
- **GIVEN** all asteroids on screen have been destroyed
- **WHEN** the next wave begins after a 2-second delay
- **THEN** the number of initial large asteroids SHALL increase by 1 (5, 6, 7...)
- **AND** the wave number SHALL be briefly displayed
- **AND** asteroid speed SHALL increase by 10% per wave

---

### Requirement: HUD Display
The game SHALL render a heads-up display showing score, lives, and wave number without interfering with gameplay.

#### Scenario: HUD layout
- **GIVEN** the game is running
- **WHEN** any frame renders
- **THEN** the top-left corner SHALL display `Score: <N>`
- **AND** the top-right corner SHALL display `Wave: <N>`
- **AND** the top-center SHALL display remaining lives as heart symbols `♥`
- **AND** HUD elements SHALL render above all game entities

---

### Requirement: Visual Representation
Game entities SHALL be rendered using ASCII/Unicode art appropriate to their type and state.

#### Scenario: Ship appearance by direction
- **GIVEN** the ship is facing upward
- **WHEN** the ship is rendered
- **THEN** the ship glyph SHALL be `▲` (or equivalent directional character)
- **AND** the ship color SHALL be bright cyan

#### Scenario: Asteroid appearance by size
- **GIVEN** a Large asteroid
- **WHEN** it is rendered
- **THEN** it SHALL be represented as a multi-cell shape (e.g., `◉` or a 3×3 ASCII art block)
- **AND** its color SHALL be a dim gray/brown

---

### Requirement: Performance Targets
The Asteroids demo SHALL maintain smooth performance under load.

#### Scenario: 60 FPS with maximum entities
- **GIVEN** Wave 10 with 13 large asteroids, 26 medium, 52 small, 50 bullets, and 200 particles
- **WHEN** the game loop runs
- **THEN** the frame time SHALL remain ≤16.67ms (60 FPS)
- **AND** rendering SHALL complete within 5ms
- **AND** collision detection SHALL complete within 2ms
