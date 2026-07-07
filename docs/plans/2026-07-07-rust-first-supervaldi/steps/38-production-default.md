# PR38 Make Rust Backend Production Default

Branch: `codex/rust-supervaldi-38-production-default`

Purpose: Make Rust backends the production default for Rust-authored apps.

Depends on: PR37

Code boundary:
- Rust-authored apps use Rust UIKit, Rust Android View, and Rust DOM backends by default.
- C++ and TypeScript renderer paths remain only for existing TypeScript compatibility targets.

Changes:
- Make Rust UIKit backend default for Rust-authored iOS apps.
- Make Rust Android View backend default for Rust-authored Android apps.
- Make Rust DOM backend default for Rust-authored web apps.
- Remove transition adapter targets from Rust app build graphs.
- Enforce forbidden dependency graph checks in default app templates and sample targets.
- Keep TypeScript compatibility sample on retained path.

Proof:
- iOS Rust app target excludes TypeScript compiler/runtime, JS direct renderer, TSN, `RenderRequest`, `ViewNodeRenderer`, `ViewNodeTree`, C++ renderer adapter targets, and transition adapter targets.
- Android Rust app target excludes TypeScript compiler/runtime, JS direct renderer, TSN, `RenderRequest`, `ViewNodeRenderer`, `ViewNodeTree`, C++ renderer adapter targets, and transition adapter targets.
- Web Rust app bundle excludes TypeScript compiler/runtime, JS direct renderer, TSN, `RenderRequest`, `ViewNodeRenderer`, `ViewNodeTree`, C++ renderer adapter targets, and transition adapter targets.
- Existing TypeScript sample still builds and renders on retained compatibility path.
- PR37 release gates pass on the default build graph.

Plan update:
- Mark PR38 complete.
- Close C++/TS dependency risks resolved by production default.
