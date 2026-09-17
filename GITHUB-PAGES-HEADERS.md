# GitHub Pages and HTTP security headers

`_headers` (Netlify / Cloudflare Pages) is **not** applied by GitHub Pages. A file with that name would only be a downloadable path, not response headers.

This demo therefore:

- Does not ship `_headers` (that would claim `X-Frame-Options` / `nosniff` that never reach the browser).
- Sets a Content-Security-Policy **meta** tag in `index.html` (scripts and connect limited to `'self'`). `frame-ancestors` cannot be set from a meta tag.
- Sets referrer policy via `<meta name="referrer">`.
- Includes `framebust.js` as a best-effort against simple embedding. Cross-origin sandboxed iframes can still ignore this; GitHub Pages cannot send `X-Frame-Options`.

WASM is already served as `application/wasm` by GitHub Pages.
