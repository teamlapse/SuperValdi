# PR35 Run Samples On Rust Platform Backends

Branch: `codex/rust-supervaldi-35-rust-backend-samples`

Purpose: Exercise Rust platform backends end to end before parity gating.

Depends on: PR34

Code boundary:
- Samples are authored in Rust.
- Rust backend is selected by feature flag.
- Transition renderer paths remain available only for comparison.

Changes:
- Switch Rust samples to Rust backend feature flag.
- Bypass retained render submission in sample targets.
- Add runtime logs that identify selected backend.
- Add sample tests for modules, native views, hot reload, dynamic UI, DOM, and PNG export on Rust backend.

Proof:
- Samples run on Rust backend for iOS, Android, and web.
- Runtime logs show Rust backend and no transition render submission.
- Sample workflows pass for modules, native views, hot reload, dynamic UI, DOM, and PNG export.

Plan update:
- Mark PR35 complete.
- Add runtime log proof.
