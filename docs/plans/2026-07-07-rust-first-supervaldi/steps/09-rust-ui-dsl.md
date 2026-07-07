# PR09 Add Complete Rust Ui Dsl

Branch: `codex/rust-supervaldi-09-rust-ui-dsl`

Purpose: Add the Rust authoring surface for declaring UI.

Depends on: PR08

Code boundary:
- Rust DSL is the app-facing UI API.
- No TSX or TypeScript compiler surface is required for Rust-authored apps.

Changes:
- Add builder DSL for every IR element, attr, event, binding, module ref, native view ref, accessibility field, and platform extension.
- Add typed compile errors for invalid attr/event/element combinations.
- Add source span capture for diagnostics and hot reload.
- Add DSL golden tests against the fixture corpus.

Proof:
- DSL output equals canonical IR for every contract row.
- Invalid DSL examples produce exact diagnostics.
- Rust app sample compiles without TypeScript, TSN, or C++ renderer dependencies.

Plan update:
- Mark PR09 complete.
- Add DSL coverage notes.
