# PR15 Render Rust Ir Through Retained Android Valdi Backend

Branch: `codex/rust-supervaldi-15-android-transition`

Purpose: Prove Rust IR can render through the retained Android Valdi backend while the Rust Android backend is built.

Depends on: PR14

Code boundary:
- Rust owns transition adapter boundary.
- Kotlin hosts the Android entry point.
- C++ exists only inside transition crate targets and retained Valdi renderer targets.

Changes:
- Add Android bridge for Rust runtime and transition adapter.
- Add Kotlin host hook.
- Add public API audit that rejects C++ symbols in Rust app crates.

Proof:
- Android emulator renders contract fixtures through retained backend.
- Ordered transition op snapshots match mock backend ordered op snapshots for the same fixtures.
- Retained backend tree and attr snapshots match expected fixture snapshots.
- Public Rust app crates expose no C++ symbols.

Plan update:
- Mark PR15 complete.
- Add Android artifact path.
- Keep C++ transition leak risk open until PR38.
