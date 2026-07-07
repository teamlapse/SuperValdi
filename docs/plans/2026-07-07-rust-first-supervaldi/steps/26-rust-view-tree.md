# PR26 Add Rust Native View Tree

Branch: `codex/rust-supervaldi-26-rust-view-tree`

Purpose: Start replacing retained native rendering with Rust-owned tree management.

Depends on: PR24

Code boundary:
- Rust owns view tree identity and lifecycle for Rust backend.
- C++ renderer tree remains only for retained compatibility and transition paths.

Changes:
- Port root, parent/child order, keyed movement, lifecycle, pooling identity, fragments, slots, portals, and managed contexts.
- Add Rust tree snapshots.
- Add retained-backend tree comparison runner.

Proof:
- Rust tree snapshots match retained backend snapshots for tree fixtures.
- Tree diff emits the same ordered op snapshots as mock backend.
- Rust tree crate has no dependency on `ViewNodeTree`.

Plan update:
- Mark PR26 complete.
- Add tree parity receipts.
