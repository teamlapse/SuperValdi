# PR10 Add Tsx-To-Ir Compatibility Frontend

Branch: `codex/rust-supervaldi-10-tsx-to-ir-compat`

Purpose: Preserve existing TypeScript apps and surface schema gaps before backend replacement begins.

Depends on: PR09

Code boundary:
- TypeScript remains compatibility infrastructure only.
- Rust-authored apps keep no TypeScript dependency.

Changes:
- Add TSX compiler path that emits UI IR.
- Keep existing direct renderer compatibility path.
- Add TSX feature coverage manifest tied to `replacement_contract.yaml`.
- Add fixture equivalence tests between TSX-emitted IR and Rust DSL IR.
- Add Rust graph check proving Rust app crates have no TS dependency.

Proof:
- TSX fixtures produce normalized IR equal to Rust DSL fixtures.
- Existing TS sample still builds and renders through retained compatibility path.
- Rust app crates have no TS dependency.

Plan update:
- Mark PR10 complete.
- Update TS dependency risk.
