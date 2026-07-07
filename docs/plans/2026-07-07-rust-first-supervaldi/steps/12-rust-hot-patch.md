# PR12 Add Rust Logic Hot Patch Support

Branch: `codex/rust-supervaldi-12-rust-hot-patch`

Purpose: Add the declared Rust logic edit class for live development.

Depends on: PR11

Code boundary:
- Rust owns dev-only action-body hot patching.
- Release builds exclude hot patch machinery.

Changes:
- Add action-body edit detector.
- Add debug-only dynamic action implementation loader.
- Add exact rebuild-required classification for signature, type, module, state shape, dependency, macro, crate graph, and platform-boundary edits.
- Add dev server messaging for hot patch success and rebuild-required failures.

Proof:
- Supported action-body edit patches live.
- Every unsupported edit class returns exact reason.
- Release target excludes dynamic loader symbols.

Plan update:
- Mark PR12 complete.
- Add support matrix.
