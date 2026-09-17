# Design: WebAssembly Architecture & Universal Static Deployment

## Context

TermForge was initially engineered for POSIX raw terminals using `termion`, which relies on `std::io::stdin()` and `std::io::stdout()`. When compiling to `wasm32-unknown-unknown`, direct OS syscalls and native TTY controls are unavailable.

To support web deployments while preserving 100% of core physics, ECS, collision, and animation logic, we introduce a dual-target architecture:
1. **Native target (`x86_64-unknown-linux-gnu` / macOS):** Uses `termion` with direct tty file descriptors.
2. **Web target (`wasm32-unknown-unknown`):** Uses an in-memory byte buffer for ANSI rendering diffs, marshaled to JavaScript and ingested by `xterm.js`.

---

## Architectural Decisions

### Decision 1: Decoupled Terminal Target via Conditional Compilation & Memory Flushes
- **Decision:** Rather than having `DoubleBuffer` write exclusively to `io::Write`, add a `flush_to_string(&mut self) -> String` or byte slice method.
- **Rationale:** WebAssembly has no native stdout. Emitting a string diff directly into Wasm linear memory allows JavaScript to execute `term.write(ansi_string)` in a single call with zero string serialization overhead.
- **Alternatives Considered:**
  - *Emscripten POSIX emulation:* Bloats binary size by megabytes and adds massive runtime overhead.
  - *Direct Canvas2D drawing:* Bypasses the terminal aesthetic and discards our double-buffer ANSI diff pipeline.

### Decision 2: Pure CSS + SVG CRT Filter Layer
- **Decision:** Implement CRT scanlines, curved glass aberration, and phosphor glow using CSS pseudo-elements (`::after`), SVG filters, and radial gradients over the canvas/DOM container.
- **Rationale:** GPU-accelerated by browser compositors, zero impact on Wasm simulation frame rate, easily toggleable via user preferences.

### Decision 3: Cloudflare Pages & GitHub Pages Dual Support
- **Decision:** Build a single, zero-dependency `dist/` folder with relative paths, accompanied by:
  - `_headers` file for Cloudflare Pages caching rules.
  - `.github/workflows/deploy.yml` for automated GitHub Pages branch pushes.
- **Rationale:** Ensures users can deploy to Cloudflare Pages (drag-and-drop or Git integration) or GitHub Pages without code modifications.

---

## Data Flow Diagram

```text
[ Browser Event Loop (requestAnimationFrame) ]
                    |
                    v
          [ JavaScript Glue ]
          /                 \
(Key Events)            (Tick + Render)
        v                     v
[ Wasm Asteroids ] ----> [ DoubleBuffer Diff ]
                              |
                     (ANSI Escape String)
                              v
                      [ xterm.js Terminal ]
                              |
                    [ CRT Shader / CSS ]
                              |
                         [ Monitor ]
```
