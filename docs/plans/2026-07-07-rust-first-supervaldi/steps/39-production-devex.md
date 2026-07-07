# PR39 Add Production Docs Templates And Devtools

Branch: `codex/rust-supervaldi-39-production-devex`

Purpose: Ship the production developer experience around the Rust-first framework.

Depends on: PR38

Code boundary:
- Templates generate Rust-authored apps and contracts.
- No renderer architecture change enters this PR.

Changes:
- Add production Rust app template.
- Add module template.
- Add native view template.
- Add migration docs.
- Add troubleshooting docs.
- Add devtools commands for IR inspect, fixture run, hot reload, parity run, graph check, and golden update.
- Add release checklist for iOS, Android, web, hot reload, modules, native views, DOM, PNG, TS compatibility, and default build graph.

Proof:
- New Rust app template builds and runs on iOS, Android, and web.
- Module template passes module conformance.
- Native view template passes native view conformance.
- Documentation links resolve.
- Devtools commands print exact Bazel labels and artifact paths.

Plan update:
- Mark PR39 complete.
- Add template and docs receipts.
