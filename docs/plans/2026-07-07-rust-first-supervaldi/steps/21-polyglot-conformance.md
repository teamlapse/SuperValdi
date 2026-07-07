# PR21 Add Complete Module Conformance Suite

Branch: `codex/rust-supervaldi-21-polyglot-conformance`

Purpose: Lock module behavior across every target before native view contracts land.

Depends on: PR20

Code boundary:
- Conformance code exercises generated factories.
- C++ remains transition-only and compatibility-only.

Changes:
- Add cross-target module conformance runner.
- Add fixture modules for every contract type family.
- Add dispatch, callback, stream, cancellation, constants, and typed error tests.
- Add failure diagnostics snapshots.

Proof:
- Complete module conformance passes on iOS, Android, web, Rust host, C++ transition, and TS compatibility.
- Every failing module fixture returns exact contract ID, method ID, arg path, platform, and owner PR.
- Rust-authored app graph excludes C++ transition module targets.

Plan update:
- Mark PR21 complete.
- Update module risk.
