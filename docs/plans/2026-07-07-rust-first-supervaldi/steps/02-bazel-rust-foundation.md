# PR02 Add Rust Framework Bazel Foundation

Branch: `codex/rust-supervaldi-02-bazel-rust-foundation`

Status: Complete

Purpose: Add the Rust workspace and Bazel foundation used by all Rust framework work.

Depends on: PR01

Code boundary:
- Rust crates and Bazel targets are introduced.
- No C or C++ app bridge surface is exposed.

Changes:
- Add `valdi_rust/` crate layout for IR, runtime, backend traits, codegen support, CLI tools, and tests.
- Add Bazel macros for Rust framework crates, platform host crates, generated glue, and fixture tests.
- Add crate naming and visibility rules.
- Add empty iOS, Android, web, and PNG test labels.
- Add forbidden public API lint that rejects app-facing C and C++ symbols.

Proof:
- `bazelisk test` passes for the Rust foundation labels.
- Generated crate graph lists every crate owner and visibility boundary.
- Public API lint passes.
- `bazelisk build` passes for the Rust foundation crate labels.

Plan update:
- Mark PR02 complete.
- Add Bazel command receipts.
