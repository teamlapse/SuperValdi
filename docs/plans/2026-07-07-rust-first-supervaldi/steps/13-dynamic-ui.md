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
