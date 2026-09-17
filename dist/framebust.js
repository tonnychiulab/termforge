if (window.top !== window.self) {
  try {
    window.top.location.replace(window.self.location.href);
  } catch (_err) {
    // Cross-origin parent: GitHub Pages cannot set frame-ancestors.
  }
}
