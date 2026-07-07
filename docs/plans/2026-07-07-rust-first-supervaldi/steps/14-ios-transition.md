# PR14 Render Rust Ir Through Retained Ios Valdi Backend

Branch: `codex/rust-supervaldi-14-ios-transition`

Purpose: Prove Rust IR can render through the retained iOS Valdi backend while the Rust UIKit backend is built.

Depends on: PR13

Code boundary:
- Rust owns the transition adapter boundary.
- C++ exists only inside transition crate targets and retained Valdi renderer targets.
- Rust app public crates expose no C++ symbols.

Changes:
- Add Rust-to-`RenderRequest` transition adapter.
- Add C++ shim limited to transition crate.
- Add iOS host hook.
- Add public API audit that rejects C++ symbols in Rust app crates.

Proof:
- iOS simulator renders contract fixtures through retained backend.
- Ordered transition op snapshots match mock backend ordered op snapshots for the same fixtures.
- Retained backend tree and attr snapshots match expected fixture snapshots.
- Public Rust app crates expose no C++ symbols.

Plan update:
- Mark PR14 complete.
- Add iOS artifact path.
- Keep C++ transition leak risk open until PR38.
