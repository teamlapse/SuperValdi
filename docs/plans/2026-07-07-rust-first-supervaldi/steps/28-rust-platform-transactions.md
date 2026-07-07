# PR28 Add Rust Platform Transactions

Branch: `codex/rust-supervaldi-28-rust-platform-transactions`

Purpose: Replace platform mutation submission for Rust backends.

Depends on: PR27

Code boundary:
- Rust owns transaction planning and ordering.
- Swift, Kotlin, and JS perform platform API calls from Rust-owned transactions.
- C++ transaction interfaces remain retained-path only.

Changes:
- Add Rust transaction planner.
- Add Swift/UIKit transaction entry points.
- Add Kotlin/View transaction entry points.
- Add JS/DOM transaction entry points.
- Add transaction-thread execution and flush ordering tests.

Proof:
- iOS, Android, and web mutate views from Rust backend transactions.
- Transaction order snapshots match fixture expectations.
- Rust platform transaction crates have no dependency on `IViewTransaction`.

Plan update:
- Mark PR28 complete.
- Add transaction receipts.
