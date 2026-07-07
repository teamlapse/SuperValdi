# PR07 Add Rust Runtime Tree Diff

Branch: `codex/rust-supervaldi-07-runtime-tree-diff`

Purpose: Add Rust document loading, identity preservation, and render-op diffing.

Depends on: PR06

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
- Static fixture declarations emit expected ordered backend ops.
- Compatible identity patches preserve state slots.
- Incompatible identity patches return rebuild-required diagnostics with source span.

Plan update:
- Mark PR07 complete.
- Add runtime diff receipts.
