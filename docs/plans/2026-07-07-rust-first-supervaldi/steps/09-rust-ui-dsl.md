# PR09 Add Complete Rust UI DSL

Branch: `codex/rust-supervaldi-09-rust-ui-dsl`

Purpose: Add the Rust authoring surface for declaring UI.

Depends on: PR08

Code boundary:
- Rust DSL is the app-facing UI API.
- No TSX or TypeScript compiler surface is required for Rust-authored apps.

Changes:
- Added `valdi_rust_dsl` as the Rust app-facing UI authoring crate.
- Added builders and typed payload construction for every PR01/PR03 contract row: schema/versioning, component identity, tree structure, element taxonomy, layout, styling, text, assets, events/gestures, actions/state, bindings/expressions, animations, native modules, native views, accessibility, hot reload identity refs, diagnostics/source spans, web DOM mappings, PNG metadata, dynamic UI refs, TS compatibility metadata, and build graph metadata.
- Added exact invalid DSL diagnostics for unsupported contract rows, missing fixture input, missing source spans, invalid attr/element pairs, and invalid event/element pairs.
- Added source span capture on app-facing element builders.
- Added DSL golden tests that consume the PR01 contract and PR04 fixture manifest/serialized fixture corpus.

Proof:
- `bazelisk test //valdi_rust:rust_ui_dsl_tests` passed. This covers DSL golden proof, invalid diagnostics, Rust app sample compile, no-forbidden-dependency checks, crate graph visibility, and public API lint.
- `bazelisk build //valdi_rust/dsl:dsl` passed.
- `bazelisk test //valdi_rust/tests:crate_graph_visibility_test //valdi_rust/tests:forbidden_public_api_test` passed.
- `python3 -m json.tool valdi_rust/crate_graph.json` passed.
- Golden proof covers all 22 PR01 contract rows by reading the contract YAML, PR04 fixture manifest, and PR04 serialized fixture artifacts, then comparing DSL canonical debug output and snapshot lines for every row.
- Invalid DSL examples produce exact diagnostic code, path, severity, and source span when applicable.
- Rust app sample compiles without TypeScript, TSN, JS direct renderer, or app-facing C/C++ renderer dependency.
- Known Bazel output: existing Bzlmod dependency-version warnings and test-size warnings may appear; no PR09-specific toolchain blocker was introduced.

Plan update:
- Marked PR09 complete.
- Added DSL coverage and quality gate notes to the main plan.
