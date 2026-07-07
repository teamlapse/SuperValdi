# PR34 Port Native View Registry And Handoff

Branch: `codex/rust-supervaldi-34-rust-native-view-handoff`

Purpose: Connect Rust backend rendering to generated native view contracts and platform implementations.

Depends on: PR33

Code boundary:
- PR22 and PR23 own contract and generated stubs.
- This PR owns Rust backend registry, lifecycle dispatch, measurement handoff, reuse, accessibility mapping, platform extensions, and fallback routing.

Changes:
- Add Rust native view registry.
- Add platform native view handoff for Swift/UIKit, Kotlin/View, and JS/DOM.
- Add lifecycle, measurement, reuse, typed attr, typed event, accessibility, platform extension, and fallback routing inside Rust backend.
- Add retained-backend comparison snapshots.

Proof:
- Native view backend fixtures pass on iOS, Android, web, and PNG fallback.
- Lifecycle, measurement, reuse, accessibility, platform extension, and fallback paths match contract snapshots.
- Rust backend native view code has no dependency on retained native view renderer targets.

Plan update:
- Mark PR34 complete.
- Add native view backend receipts.
