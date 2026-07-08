# PR11 Add IR Hot Reload Pipeline

Branch: `codex/rust-supervaldi-11-ir-hot-reload`

Purpose: Deliver fast UI hot reload through IR patches.

Depends on: PR10

Code boundary:
- Rust owns watcher, IR patch generation, daemon transport, app receiver, patch validator, and compatibility checks.
- No platform module/native-view implementation is required for this PR; mock registries validate refs against the contract.

Changes:
- Added Rust UI declarative parser for hot-reloadable edit syntax.
- Added simulated file watcher, typed IR patch generator, in-memory transport, app receiver, patch validator, state compatibility checker, asset patch support, binding compatibility checker, action compatibility checker, contract-backed module ref checker, and contract-backed native view ref checker in `valdi_rust_hot_reload`.
- Added deterministic latency report split by file watch, IR generation, transport, validation, diff, mock backend apply, and host receive.
- Added exact rebuild-required diagnostics for unsupported action-body edits and module/native-view contract shape changes.
- Kept production dev server networking, platform host integration, generated module/native-view factories, Rust logic hot patching, and dynamic UI ingestion out of PR11.

Proof:
- `bazelisk test //valdi_rust:ir_hot_reload_tests` passes.
- `bazelisk build //valdi_rust/hot_reload:hot_reload` passes.
- `bazelisk test //valdi_rust/tests:crate_graph_visibility_test //valdi_rust/tests:forbidden_public_api_test` passes.
- Parser, patch generator, live sample host, rebuild diagnostics, module/native-view refs, binding/action compatibility, latency report, and sample-host snapshot tests cover the PR11 proof surface.
- UI tree, style, layout, text, asset, binding, event, accessibility, module ref, and native view ref edits patch through a persistent in-process sample host session without reinstall or relaunch semantics.
- Mock registries validate module and native-view refs without platform factory code and reject ABI/contract-shape drift with exact rebuild-required diagnostics.
- Unsupported action-body/business-logic patch returns exact rebuild-required diagnostics and leaves runtime state intact; PR12 owns supported action-body hot patching.
- Deterministic latency report records `file_watch`, `ir_generation`, `transport`, `validation`, `runtime_diff`, `mock_backend_apply`, and `host_receive`, totaling 8,900us against a 16,000us patch budget.

Plan update:
- Mark PR11 complete.
- Add latency report and quality-gate rows.
