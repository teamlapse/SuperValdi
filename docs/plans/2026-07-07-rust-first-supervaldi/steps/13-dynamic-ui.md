# PR13 Add Dynamic Ir Ingestion Api

Branch: `codex/rust-supervaldi-13-dynamic-ui`

Purpose: Allow validated UI IR to be produced outside the Rust DSL and consumed by the same runtime contract.

Depends on: PR11

Code boundary:
- Rust owns dynamic producer API and validation.
- Platform backend proof remains in the backend PRs that introduce those backends.

Changes:
- Add validated dynamic IR producer API.
- Add loading from JSON, binary bytes, generated fixtures, and in-memory Rust producers.
- Add capability negotiation for dynamic producers.
- Add source and trust metadata for dynamic documents.
- Add runtime validator integration shared with DSL-produced IR.

Proof:
- Dynamic producer feeds validator, runtime, and mock backend using the same diagnostics as DSL-produced IR.
- Invalid dynamic IR returns exact schema path, source, trust, node, and owner PR diagnostics.
- Dynamic producer output round-trips through JSON and binary codecs.

Plan update:
- Mark PR13 complete.
- Add dynamic UI receipts.

Implementation receipts:
- Added `valdi_rust_dynamic_ui` as a PR11-sibling crate with dependencies only on `valdi_rust_backend`, `valdi_rust_codec`, `valdi_rust_fixtures`, `valdi_rust_ir`, and `valdi_rust_runtime`.
- Dynamic ingestion supports JSON debug, postcard binary bytes, generated PR04 fixtures, and in-memory Rust producers through typed source, trust, and capability metadata.
- Runtime integration validates PR05 fixture envelopes against the PR01/PR04 contract, builds typed PR07 runtime documents, and feeds PR06 mock backend operations.
- Unsupported or invalid dynamic documents return exact diagnostics carrying schema path, source ID, trust level, node ID, and owner PR context for capability mismatch, untrusted source, invalid schema path, owner mismatch, node drift, and contract drift.
- PR13 remains parented to PR11 and uses only the Rust backend, codec, fixture, IR, and runtime crates available from that parent stack.

Proof results:
- `bazelisk build //valdi_rust/dynamic_ui:dynamic_ui` passed.
- `bazelisk test //valdi_rust:dynamic_ui_tests` passed, 9/9 tests.
- `python3 -m json.tool valdi_rust/crate_graph.json` passed.
- `python3 valdi_rust/tests/crate_graph_visibility_test.py valdi_rust/crate_graph.json` passed.
- `rustfmt --check valdi_rust/dynamic_ui/src/*.rs valdi_rust/dynamic_ui/tests/*.rs` passed.
- Snapshot proof covers stable dynamic UI runtime/mock-backend trace and invalid diagnostic receipts.
