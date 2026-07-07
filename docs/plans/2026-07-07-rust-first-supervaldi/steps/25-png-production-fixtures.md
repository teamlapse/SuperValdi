# PR25 Complete Png Production Fixture Coverage

Branch: `codex/rust-supervaldi-25-png-production-fixtures`

Purpose: Complete PNG coverage for all static-render contract surfaces.

Depends on: PR24

Code boundary:
- Rust owns PNG rendering.
- No platform renderer dependency enters PNG goldens.

Changes:
- Complete PNG golden suite for every `static_png` fixture.
- Add deterministic font assets.
- Add deterministic image assets.
- Add text and rich text goldens.
- Add native-view fallback and webview fallback goldens.
- Add accessibility debug metadata goldens.
- Add deterministic golden update command.

Proof:
- PNG golden suite passes on CI for every static-render fixture.
- Golden update command produces identical output across two runs.
- Failing golden output reports fixture ID, node ID, asset ID, font ID, and backend version.

Plan update:
- Mark PR25 complete.
- Add golden summary.
