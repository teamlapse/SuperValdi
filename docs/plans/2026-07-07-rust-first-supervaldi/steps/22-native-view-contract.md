# PR22 Add Rust-Authored Native View Contract Idl

Branch: `codex/rust-supervaldi-22-native-view-contract`

Purpose: Add the app-facing Rust contract surface for native views.

Depends on: PR21

Code boundary:
- App developers write native view contracts in Rust.
- Platform teams implement Swift/UIKit, Kotlin/View, and JS/DOM stubs.
- No app developer writes C or C++ bridge code.

Changes:
- Add Rust native view contract macro or IDL crate.
- Model typed attrs, typed events, lifecycle, measurement, reuse, platform extensions, accessibility, and fallback rendering.
- Integrate contract metadata with fixture tags and generator discovery.
- Add invalid native view contract diagnostics.

Proof:
- Native view contract matrix test passes.
- Unsupported contract shape fails generation with exact alternatives.
- Generated metadata lists Swift/UIKit, Kotlin/View, JS/DOM, PNG fallback, Rust runtime, and retained compatibility targets.

Plan update:
- Mark PR22 complete.
- Update native view risk.
