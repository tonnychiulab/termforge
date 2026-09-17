# Delta Spec: Tactical Maneuvers & Tactical Abilities

## ADDED Requirements

---

### Requirement: Active Braking Mechanism
The ship physics model SHALL support active braking when the player presses `S` or `Down Arrow`, rapidly decelerating current linear velocity.

#### Scenario: Ship decelerates under active brake
- **GIVEN** a moving ship with linear speed of 15.0 units/sec
- **WHEN** the player holds `S` or `Down Arrow`
- **THEN** the ship's linear velocity magnitude SHALL decrease by at least 60% within 0.5 seconds
- **AND** reverse thruster exhaust particles SHALL emit from the ship's nose

---

### Requirement: Lateral Strafing Thrusters
The ship SHALL support lateral strafing without altering ship facing angle when the player presses `Q` (port/left strafe) or `E` (starboard/right strafe).

#### Scenario: Left lateral strafe
- **GIVEN** a ship facing upward (angle $-90^\circ$)
- **WHEN** the player presses `Q`
- **THEN** an acceleration vector directed westward ($-1.0, 0.0$) SHALL be applied
- **AND** the ship's orientation angle SHALL remain unchanged at $-90^\circ$

#### Scenario: Right lateral strafe
- **GIVEN** a ship facing upward (angle $-90^\circ$)
- **WHEN** the player presses `E`
- **THEN** an acceleration vector directed eastward ($1.0, 0.0$) SHALL be applied

---

### Requirement: Instant 180° Flip
The ship SHALL rotate its facing direction by exactly $180^\circ$ ($\pi$ radians) instantaneously when `X` is pressed.

#### Scenario: Instant turnaround
- **GIVEN** a ship facing right ($0^\circ$)
- **WHEN** the player presses `X`
- **THEN** the ship's angle SHALL immediately change to $180^\circ$ ($\pi$ radians)
- **AND** linear velocity direction SHALL NOT be changed

---

### Requirement: Hyperspace Jump
The ship SHALL teleport to a random screen coordinate when `Z` or `H` is triggered.

#### Scenario: Hyperspace teleport
- **GIVEN** an active player ship surrounded by asteroids
- **WHEN** the player triggers `Z` or `H`
- **THEN** the ship's position SHALL change to a new random coordinate within screen bounds
- **AND** the ship's velocity SHALL be zeroed
- **AND** an instantaneous 1.5-second invulnerability shield SHALL be activated

---

### Requirement: EMP Shockwave Smart Bomb
The game SHALL grant 1 EMP bomb per life, triggered by `B` or `F`, destroying all asteroids within a blast radius and dealing splash damage.

#### Scenario: EMP detonation
- **GIVEN** the player has 1 EMP bomb remaining
- **WHEN** the player presses `B` or `F`
- **THEN** an expanding shockwave particle ring SHALL trigger
- **AND** all asteroids within a 20-cell radius SHALL be destroyed or split
- **AND** the bomb counter SHALL decrement to 0
- **AND** sound event `emp` SHALL be emitted

---

### Requirement: Pause and Mute State Controls
The engine SHALL allow pausing game simulation with `P` and toggling sound with `M`.

#### Scenario: Game pause
- **GIVEN** an active game session
- **WHEN** the player presses `P`
- **THEN** game physics and entity movements SHALL freeze
- **AND** a centered "=== PAUSED ===" dialog box SHALL overlay the screen
- **AND** pressing `P` again SHALL resume simulation smoothly
