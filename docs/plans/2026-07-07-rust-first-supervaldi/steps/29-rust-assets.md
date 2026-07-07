# PR29 Port Asset Loading To Rust Backend

Branch: `codex/rust-supervaldi-29-rust-assets`

Purpose: Port production asset resolution and loaded-asset propagation.

Depends on: PR28

Code boundary:
- Rust owns asset identity, variant selection, cache identity, loading state, and diagnostics.
- Platform hooks perform platform asset fetch/decode work.

Changes:
- Add Rust asset resolver.
- Add variant selection for iOS, Android, web, SVG, data refs, and remote refs.
- Add loaded asset propagation.
- Add cache identity and loading/error state handling.
- Add missing asset diagnostics.

Proof:
- Asset fixtures pass on iOS, Android, and web.
- Missing asset failure includes module name, path, node ID, backend, and source span.
- Rust asset crate has no dependency on retained `Resources` renderer targets.

Plan update:
- Mark PR29 complete.
- Add asset receipts.
