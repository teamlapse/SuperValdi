# PR23 Generate Native View Platform Stubs

Branch: `codex/rust-supervaldi-23-native-view-stubs`

Purpose: Generate platform stubs and retained-path metadata from Rust-authored native view contracts.

Depends on: PR22

Code boundary:
- App contracts remain Rust.
- Swift, Kotlin, and JS stubs are platform implementation files.
- C++ appears only in retained compatibility metadata.

Changes:
- Generate Swift/UIKit stubs.
- Generate Kotlin/View stubs.
- Generate JS/DOM stubs.
- Generate PNG fallback metadata.
- Generate retained compatibility metadata.
- Add lifecycle, measure, reuse, attr, event, accessibility, and fallback stub tests.

Proof:
- Native view stub conformance passes for iOS, Android, web, PNG fallback metadata, and retained compatibility metadata.
- Generated stubs use the same contract IDs and attr/event IDs.
- Rust-authored app graph excludes retained C++ native view targets.

Plan update:
- Mark PR23 complete.
- Add native view stub receipts.
