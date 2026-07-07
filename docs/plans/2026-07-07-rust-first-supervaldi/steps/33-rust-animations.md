# PR33 Port Animation Dispatch

Branch: `codex/rust-supervaldi-33-rust-animations`

Purpose: Port animation behavior to Rust backend.

Depends on: PR32

Code boundary:
- Rust owns animation model, transaction grouping, and lifecycle state.
- Platform hooks execute native animation APIs.

Changes:
- Add start, end, cancel, timing, easing, delay, repeat, fill mode, property animation, layout animation, and transaction grouping.
- Add native animator bridge for iOS, Android, and web.
- Add animation cancellation and completion snapshots.

Proof:
- Animation fixtures start, complete, cancel, and match transaction snapshots on iOS, Android, and web.
- Rust animation crate has no dependency on retained native animator renderer targets.
- Animation diagnostics include node ID, animation ID, property path, platform, and owner PR.

Plan update:
- Mark PR33 complete.
- Add animation receipts.
