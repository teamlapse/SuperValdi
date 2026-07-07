# PR24 Add Rust Production Sample Suite On Transition Backends

Branch: `codex/rust-supervaldi-24-samples-transition`

Purpose: Add end-to-end samples before Rust platform backend replacement begins.

Depends on: PR23

Code boundary:
- Samples are authored in Rust.
- iOS and Android samples still render through retained transition adapters.
- Web sample renders through Rust DOM backend.

Changes:
- Add iOS Rust sample.
- Add Android Rust sample.
- Add web Rust sample.
- Cover modules, native views, hot reload, dynamic UI, and PNG fixture export.
- Add sample launch scripts and CI labels.

Proof:
- Samples build and launch on iOS, Android, and web.
- Samples hot reload a UI edit.
- Samples call a module and render a native view.
- Samples export a PNG fixture.

Plan update:
- Mark PR24 complete.
- Add sample receipts.
