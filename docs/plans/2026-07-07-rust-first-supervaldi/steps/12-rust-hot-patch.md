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
- Marked PR12 complete in the main plan.
- Added support matrix covering the only live-patchable edit class plus every rebuild-required edit class.

Implemented files:
- `valdi_rust/hot_patch/src/detector.rs` classifies action-body edits and exact rebuild-required reasons for signature, type, module, state shape, dependency, macro, crate graph, and platform-boundary drift.
- `valdi_rust/hot_patch/src/loader.rs` provides a dev-only typed action implementation loader used by tests to prove the action body changes live.
- `valdi_rust/hot_patch/src/dev_server.rs` emits stable dev-server style success and rebuild-required messages.
- `valdi_rust/hot_patch/src/release_guard.rs` proves release builds exclude hot patch machinery and dev loader symbols.
- `valdi_rust/hot_patch/src/support_matrix.rs` records the PR12 support matrix and drift checks.

Support matrix:

| Edit class | PR12 behavior | Diagnostic / proof |
| --- | --- | --- |
| `action_body` | Live patch | `HOT_PATCH_ACTION_BODY_SUPPORTED`; `//valdi_rust/hot_patch:live_action_body_patch_test` |
| `signature` | Rebuild required | `HOT_PATCH_SIGNATURE_REBUILD_REQUIRED` |
| `type` | Rebuild required | `HOT_PATCH_TYPE_REBUILD_REQUIRED` |
| `module` | Rebuild required | `HOT_PATCH_MODULE_REBUILD_REQUIRED` |
| `state_shape` | Rebuild required | `HOT_PATCH_STATE_SHAPE_REBUILD_REQUIRED` |
| `dependency` | Rebuild required | `HOT_PATCH_DEPENDENCY_REBUILD_REQUIRED` |
| `macro` | Rebuild required | `HOT_PATCH_MACRO_REBUILD_REQUIRED` |
| `crate_graph` | Rebuild required | `HOT_PATCH_CRATE_GRAPH_REBUILD_REQUIRED` |
| `platform_boundary` | Rebuild required | `HOT_PATCH_PLATFORM_BOUNDARY_REBUILD_REQUIRED` |

Validation targets:
- `rustfmt --check valdi_rust/hot_patch/src/*.rs valdi_rust/hot_patch/tests/*.rs`
- `bazelisk test //valdi_rust:rust_hot_patch_tests`
- `bazelisk build //valdi_rust/hot_patch:hot_patch`
- `bazelisk test //valdi_rust/tests:crate_graph_visibility_test //valdi_rust/tests:forbidden_public_api_test //valdi_rust/tests:rust_app_no_ts_dependency_test`
- `python3 -m json.tool valdi_rust/crate_graph.json`
- `git diff --check`
