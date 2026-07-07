# PR16 Add Rust Dom Backend

Branch: `codex/rust-supervaldi-16-web-dom-backend`

Purpose: Add the production web rendering path for Rust-authored apps.

Depends on: PR13

Code boundary:
- Rust/WASM owns DOM operation production and state.
- A small JS host applies DOM operations and browser APIs.
- No TypeScript direct renderer is used by Rust-authored web apps.

Changes:
- Add Rust/WASM DOM backend.
- Add JS host for DOM creation, style mapping, event mapping, text measurement, class emission, and screenshot metadata.
- Add browser fixture runner.
- Add web build labels for Rust-authored apps.

Proof:
- Browser DOM snapshots pass for DOM-tagged fixtures.
- Browser screenshots pass for screenshot-tagged fixtures.
- Web Rust app bundle excludes TypeScript direct renderer dependency.

Plan update:
- Mark PR16 complete.
- Add browser artifact path.
