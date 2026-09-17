# Delta Spec: WebAssembly Integration & Virtual Terminal Frontend

## ADDED Requirements

---

### Requirement: Wasm Engine Binding
The engine SHALL provide a WebAssembly interface exposing initialization, per-frame step execution, key event injection, and ANSI diff string retrieval.

#### Scenario: Frame execution in browser requestAnimationFrame loop
- **GIVEN** an instantiated Wasm game instance
- **WHEN** the browser triggers `requestAnimationFrame` with delta time
- **THEN** the Wasm engine SHALL advance game physics by the elapsed fixed timesteps
- **AND** the Wasm engine SHALL return a UTF-8 string containing ANSI escape codes for all dirty cells
- **AND** total execution time per tick SHALL remain $\le 4$ milliseconds

#### Scenario: Browser keyboard event mapping
- **GIVEN** the browser virtual terminal is focused
- **WHEN** the user presses physical keys ('W', 'A', 'S', 'D', 'Space', Arrow keys)
- **THEN** the JavaScript wrapper SHALL intercept the keydown/keyup events
- **AND** forward them into the Wasm engine input state buffer before the next tick

---

### Requirement: Retro CRT Virtual Terminal UI
The web frontend SHALL render terminal output inside an xterm.js terminal with customizable retro CRT visual effects.

#### Scenario: CRT scanlines and phosphor glow
- **GIVEN** the game is loaded in a modern browser
- **WHEN** the game renders on screen
- **THEN** an overlay layer SHALL generate subtle horizontal scanlines
- **AND** text glyphs SHALL display a slight phosphor glow effect
- **AND** the aspect ratio SHALL automatically fit the browser viewport without distortion

#### Scenario: Mobile and Touch responsiveness
- **GIVEN** a user opens the webpage on a mobile touch device
- **WHEN** touch controls are enabled
- **THEN** a virtual on-screen keypad (Thrust, Rotate Left/Right, Fire) SHALL be accessible
- **AND** touch events SHALL translate to corresponding game inputs

---

### Requirement: Zero-Config Cloudflare & GitHub Pages Deployment
The project SHALL generate a standalone, self-contained `dist/` bundle that operates without server-side compute.

#### Scenario: Direct static hosting
- **GIVEN** the static `dist/` directory
- **WHEN** deployed to Cloudflare Pages or GitHub Pages
- **THEN** the `index.html` SHALL load all assets (WASM binary, JS loader, CSS) via relative paths
- **AND** game execution SHALL function properly without requiring special HTTP headers (like COOP/COEP)
