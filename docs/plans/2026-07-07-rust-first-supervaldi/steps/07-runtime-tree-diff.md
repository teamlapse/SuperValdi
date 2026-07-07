# PR07 Add Rust Runtime Tree Diff

Branch: `codex/rust-supervaldi-07-runtime-tree-diff`

Purpose: Add Rust document loading, identity preservation, and render-op diffing.

Depends on: PR06

Status: Complete

Code boundary:
- Rust owns document runtime, component tree, node identity, and diff.
- No platform renderer implementation enters this PR.

Changes:
- Add runtime document loader from validated IR.
- Add component tree and stable identity tables.
- Add keyed move, insert, delete, root, fragment, slot, portal, context, and destruction diff logic.
- Emit backend ops through the backend trait.
- Add state-preserving patch foundations used by hot reload.

Proof:
- `//valdi_rust:runtime_tree_diff_tests` covers typed runtime document loading, tree diff operation ordering, identity patch preservation, exact rebuild diagnostics, and static fixture backend-op snapshot stability.
- `//valdi_rust/runtime:document_loader_test` proves typed static fixture declarations load into runtime documents using IR schema types.
- `//valdi_rust/runtime:tree_diff_test` proves create/root/move/destroy ordering, keyed identity, destruction policy, and direct consumption by the PR06 mock backend.
- `//valdi_rust/runtime:identity_patch_test` proves compatible patches preserve state slots and missing expected state mappings fail.
- `//valdi_rust/runtime:rebuild_diagnostic_test` proves incompatible identity patches return exact rebuild-required diagnostics with source spans.
- `//valdi_rust/runtime:static_fixture_backend_ops_snapshot_test` checks the ordered backend operation snapshot and its missing-operation negative case.

Plan update:
- Marked PR07 complete.
- Added runtime diff receipts.
