# PR06 Add Full Backend Operation Model

Branch: `codex/rust-supervaldi-06-backend-ops`

Purpose: Create the backend operation contract shared by native, web, PNG, and transition renderers.

Depends on: PR05

Status: Complete

Code boundary:
- Rust owns backend op enum, render backend trait, operation validator, and mock backend.
- C++ remains untouched except referenced in transition fixture names.

Changes:
- Add backend op enum for create, destroy, root, move, attr, animation, layout callback, draw callback, visibility, frame observer, asset load, text measure, native view mount, webview mount, and transaction group.
- Add render backend trait with typed capability negotiation.
- Add backend operation validator.
- Add mock backend snapshots for every fixture tag.

Proof:
- `//valdi_rust:backend_operation_tests` covers backend contract coverage, trait compile, exact capability diagnostics, fixture-tag coverage, and the stable mock snapshot.
- `//valdi_rust/backend:mock_backend_snapshot_test` checks `valdi_rust/backend/snapshots/fixture_tag_backend_ops.snap`.
- `//valdi_rust/backend:backend_contract_coverage_test` consumes the PR04 fixture manifest and proves every fixture tag maps to a backend operation family decision.
- `//valdi_rust/backend:capability_validator_test` rejects unsupported backend capability use with exact diagnostic code, path, and severity.
- Backend trait compile proof covers host plus configured iOS and Android Rust target triples. Rust WASM compile proof is blocked in PR06 because no Rust WASM target/toolchain label is configured in `MODULE.bazel` or `bzl/workspace_rust_init.bzl`; the only WASM condition found is not a Rust backend toolchain.

Plan update:
- Marked PR06 complete.
- Added snapshot path and Rust WASM compile-proof blocker note.
