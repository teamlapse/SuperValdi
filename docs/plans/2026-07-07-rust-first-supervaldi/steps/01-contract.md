# PR01 Add Production Replacement Contract

Branch: `codex/rust-supervaldi-01-contract`

Purpose: Create the authoritative contract consumed by every implementation PR.

Depends on: PR00

Code boundary:
- Contract artifacts are YAML and generated Markdown.
- No runtime implementation enters this PR.

Changes:
- Add `docs/rust_migration/replacement_contract.yaml`.
- Add generated `docs/rust_migration/replacement_contract.md`.
- Add row IDs for schema/versioning, identity, tree, taxonomy, layout, styling, text, assets, events, state, bindings, animations, modules, native views, accessibility, hot reload, diagnostics, web DOM, PNG, dynamic UI, TS compatibility, and build graph.
- Add fixture tag vocabulary: `ios`, `android`, `web`, `dom_snapshot`, `screenshot`, `static_png`, `retained_backend`, `rust_backend`, `module`, `native_view`, `hot_reload`, `dynamic_ui`, `tsx_compat`.
- Add forbidden dependency list for production Rust apps: TypeScript compiler/runtime, JS direct renderer, TSN C emitter/runtime, `RenderRequest`, `ViewNodeRenderer`, `ViewNodeTree`, C++ renderer adapter targets, and transition adapter targets.
- Add contract check for duplicate row IDs, missing owners, missing proof gates, missing fixture tags, missing platform target, and stand-in wording.

Proof:
- Contract check passes.
- Generated contract names implementation PRs and proof PRs for every row.
- Generated contract contains every row listed in the plan coverage summary.
- Plan PR files outside docs/test metadata are unchanged.

Plan update:
- Mark PR01 complete.
- Add contract artifact paths to the Quality Gate Ledger.
- Keep the contract-missing-surface risk open until PR03 and PR04 consume the contract.
