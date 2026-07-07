# PR03 Add Full Rust Ui Ir Schema

Branch: `codex/rust-supervaldi-03-ir-schema`

Purpose: Implement Rust UI IR types for every production surface in the contract.

Depends on: PR02

Code boundary:
- Rust owns schema types.
- No TypeScript compiler path or C++ renderer path is used by the Rust schema crate.

Changes:
- Add versioned Rust structs/enums for all contract rows.
- Add typed platform capability IDs and typed platform extension structs.
- Add stable node, component, state, action, binding, module, native view, asset, source span, and diagnostic identifiers.
- Add Rust type coverage test generated from `replacement_contract.yaml`.
- Add Rustdoc examples for every public IR type family.

Proof:
- Schema coverage test proves no contract row lacks a Rust type.
- Typed extension test rejects opaque platform bags.
- Rustdoc examples compile.

Plan update:
- Mark PR03 complete.
- Add schema coverage notes.
