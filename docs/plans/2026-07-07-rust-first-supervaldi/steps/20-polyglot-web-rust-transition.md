# PR20 Generate Web Rust Host And Transition Module Factories

Branch: `codex/rust-supervaldi-20-polyglot-web-rust-transition`

Purpose: Generate remaining module factories required by production and compatibility paths.

Depends on: PR19

Code boundary:
- App contracts remain Rust.
- JS/DOM factory serves web.
- C++ transition factory serves retained compatibility only.
- TS compatibility factory serves existing TypeScript apps.

Changes:
- Generate JS/DOM factory glue.
- Generate Rust host factory glue.
- Generate C++ transition factory glue.
- Generate TS compatibility metadata.
- Add build labels for web, Rust host, C++ transition, and TS compatibility module fixtures.

Proof:
- Same module conformance passes on web, Rust host, C++ transition, and TS compatibility targets.
- Rust-authored app targets do not depend on C++ transition factories.
- Existing TS sample can call a compatibility module.

Plan update:
- Mark PR20 complete.
- Add web/Rust/transition module receipts.
