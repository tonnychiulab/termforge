# Proposal: WebAssembly & Virtual Terminal Support (add-web-wasm-support)

## Why

Currently, TermForge and the Asteroids demo run exclusively inside native Unix terminals (Linux/macOS) via `termion` interacting with raw OS file descriptors (`/dev/tty`). 

**Pain Points & Opportunities:**
1. **Barrier to Entry:** Prospective users, recruiters, or gamers must have a local Rust environment or a compatible Unix shell with appropriate terminal permissions to test the engine.
2. **Web Showcase:** WebAssembly (Wasm) allows native Rust code to execute at near-native speeds in web browsers. Combining Rust Wasm with a modern browser virtual terminal (`xterm.js`) enables zero-install, cross-device instant playability while preserving the authentic 60 FPS ANSI terminal experience.
3. **Multi-Platform Deployment:** Providing a static, self-contained distribution allows one-click hosting on edge CDN platforms like Cloudflare Pages or GitHub Pages with zero server maintenance costs.

## What Changes

We introduce WebAssembly compilation targets and a virtual browser terminal runtime for TermForge:
- **Wasm Target Compatibility:** Abstract the raw terminal backend so that TermForge can drive either native OS terminals via `termion` or a browser virtual terminal via JavaScript/Wasm callbacks.
- **CRT Vintage Aesthetics:** An immersive vintage terminal UI complete with phosphor glow, scanline shader effects, and nostalgic retro styling.
- **Universal Static Bundle:** A production-ready `dist/` package containing the compiled `.wasm`, JavaScript glue, WebGL/CSS CRT shaders, and HTML5 assets ready for immediate deployment on Cloudflare Pages and GitHub Pages.

## Capabilities

### Added
- `termforge-wasm`: WebAssembly bridge exposing the engine tick and rendering diff stream to JavaScript.
- `web-runtime`: HTML5 / CSS3 / xterm.js frontend with retro CRT scanline filters and mobile touch virtual DPAD support.
- `cloudflare-github-deploy`: CI/CD workflows and configuration for zero-config deployments to Cloudflare Pages and GitHub Pages.

## Impact

- **Affected Modules**:
  - `crates/termforge/src/render.rs` (Export buffer diff as UTF-8 string/Uint8Array)
  - `games/asteroids` (Add Wasm-compatible entry point)
- **New Directory**:
  - `web/` (Frontend assets, xterm.js integration, CSS CRT effects, build scripts)
- **Dependencies**:
  - `wasm-bindgen` ^0.2
  - `web-sys` (console logging, requestAnimationFrame)
  - `xterm` ^5.3 (browser terminal emulator)
  - `xterm-addon-fit`
- **Breaking Changes**: None. Native terminal builds remain 100% functional.
