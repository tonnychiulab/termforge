# Design: Maneuver Kinematics & Ability Mechanics

## Context

The classical 2D Asteroids physics model operates with single-axis forward thrust $F = \hat{u} \cdot a$ along unit vector $\hat{u} = (\cos \theta, \sin \theta)$. Extending this model to support lateral strafing and braking transforms the flight dynamics into a semi-Newtonian 6-DOF (Degree of Freedom) 2D spacecraft simulator.

---

## Architectural Decisions

### Decision 1: Perpendicular Force Vectors for Strafing
- **Decision:** Calculate strafe direction using normal vectors:
  $$\hat{u}_{\text{left}} = \left(-\sin \theta, \cos \theta\right), \quad \hat{u}_{\text{right}} = \left(\sin \theta, -\cos \theta\right)$$
- **Rationale:** Keeps strafing relative to current ship yaw angle rather than world cardinal directions, maintaining dogfight immersion.

### Decision 2: Active Braking Exponential Damping
- **Decision:** When `S` or `Down Arrow` is active, apply an aggressive deceleration factor:
  $$v_{t+dt} = v_t \times (0.90)^{60 \cdot dt}$$
- **Rationale:** Far more satisfying than simple linear clamping, producing smooth deceleration feel like retro thrusters firing in reverse.

### Decision 3: EMP Smart Bomb Shockwave Expansion
- **Decision:** The EMP bomb triggers an immediate distance query against all active asteroids:
  $$\text{dist}(P_{\text{ship}}, P_{\text{asteroid}}) \le R_{\text{blast}}$$
  Asteroids within $R_{\text{blast}} = 22.0$ are immediately split/destroyed with corresponding score awards, accompanied by a 60-particle circular shockwave ring.

### Decision 4: Global Game State Enum
- **Decision:** Introduce `enum GameState { Running, Paused, GameOver }` in both native and WASM loops.
- **Rationale:** Decouples input handling, logic ticking, and HUD overlay states cleanly without messy boolean flags.
