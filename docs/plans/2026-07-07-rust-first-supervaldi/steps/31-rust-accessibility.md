# PR31 Port Accessibility Mapping

Branch: `codex/rust-supervaldi-31-rust-accessibility`

Purpose: Port accessibility behavior to Rust backend.

Depends on: PR30

Code boundary:
- Rust owns accessibility model and normalized snapshots.
- Swift, Kotlin, and JS apply platform accessibility APIs.

Changes:
- Add role, label, hint, value, state, action, focus order, grouping, hidden state, and platform overrides.
- Add accessibility snapshots for controls, text input, native views, and web DOM.
- Add diagnostics for invalid accessibility combinations.

Proof:
- Accessibility snapshots pass on iOS, Android, and web.
- Invalid accessibility fixtures return exact diagnostics.
- Rust accessibility crate has no dependency on retained accessibility renderer targets.

Plan update:
- Mark PR31 complete.
- Add accessibility receipts.
