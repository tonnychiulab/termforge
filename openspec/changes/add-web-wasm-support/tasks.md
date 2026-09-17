# Implementation Checklist: WebAssembly & Universal Web Deployment

## 1. Core Engine Web Compatibility
- [x] 1.1 Add `flush_to_string()` method in `termforge::render::DoubleBuffer` to output dirty ANSI deltas into memory
- [x] 1.2 Enable target-specific conditional compilation for `termion` in `crates/termforge`
- [x] 1.3 Add `wasm32-unknown-unknown` compilation target to Rust toolchain

## 2. WebAssembly Bridge Crate (`crates/termforge_web`)
- [x] 2.1 Scaffold `termforge_web` crate with `wasm-bindgen` dependency
- [x] 2.2 Expose `WasmAsteroidsGame` struct with `new(width, height)`, `tick(dt)`, `key_down(key)`, `key_up(key)`, and `render()`
- [x] 2.3 Compile optimized `.wasm` bundle using `wasm-pack` or `cargo build --target wasm32-unknown-unknown`

## 3. Retro CRT Web Frontend
- [x] 3.1 Create `web/index.html` featuring xterm.js CDN loader and retro layout
- [x] 3.2 Implement CSS3 CRT screen curvature, scanline flicker overlay, and phosphor glow effects
- [x] 3.3 Implement on-screen touch D-Pad and Fire buttons for mobile browsers
- [x] 3.4 Wire browser keyboard event listener to Wasm bridge inputs

## 4. Production Packaging & Multi-Platform Deployment
- [x] 4.1 Assemble standalone `dist/` bundle containing HTML, CSS, JS, and optimized `.wasm`
- [x] 4.2 Add `_headers` configuration for Cloudflare Pages (MIME type `application/wasm`)
- [x] 4.3 Add GitHub Actions workflow (`.github/workflows/deploy.yml`) for automated GitHub Pages publishing
- [x] 4.4 Local static server test verifying 60 FPS in browser
