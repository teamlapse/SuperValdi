# PR11 Add Ir Hot Reload Pipeline

Branch: `codex/rust-supervaldi-11-ir-hot-reload`

Purpose: Deliver fast UI hot reload through IR patches.

Depends on: PR10

Code boundary:
- Rust owns watcher, IR patch generation, daemon transport, app receiver, patch validator, and compatibility checks.
- No platform module/native-view implementation is required for this PR; mock registries validate refs against the contract.

Changes:
- Add Rust UI declarative parser for hot-reloadable DSL syntax.
- Add file watcher, IR patch generator, daemon transport, app receiver, patch validator, state compatibility checker, asset patch support, binding compatibility checker, action compatibility checker, contract-backed module ref checker, and contract-backed native view ref checker.
- Add latency report split by file watch, IR generation, transport, validation, diff, mock backend apply, and host receive.
- Add dev server protocol for exact rebuild-required reasons.

Proof:
- UI tree, style, layout, text, asset, binding, event, accessibility, module ref, and native view ref edits patch live on the sample host without reinstall or relaunch.
- Mock registry validates module and native-view refs without platform factory code.
- Unsupported UI patch returns exact diagnostic and leaves runtime state intact.

Plan update:
- Mark PR11 complete.
- Add latency report.
