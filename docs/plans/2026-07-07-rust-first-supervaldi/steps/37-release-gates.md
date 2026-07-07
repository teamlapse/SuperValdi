# PR37 Add Production Release Gates And Budgets

Branch: `codex/rust-supervaldi-37-release-gates`

Purpose: Install release-blocking checks before Rust backend becomes the production default.

Depends on: PR36

Code boundary:
- Release gates are CI and tooling.
- No renderer architecture changes enter this PR.

Changes:
- Add CI gate for Rust app forbidden dependency graph on iOS, Android, and web.
- Add CI gate for full platform parity suite.
- Add CI gate for module conformance.
- Add CI gate for native view conformance.
- Add CI gate for PNG golden determinism.
- Add CI gate for hot reload and hot patch support matrix.
- Add performance budgets for IR validation, diff, hot reload patch latency, backend op emission, iOS transaction apply, Android transaction apply, DOM apply, and PNG render.
- Add release artifact manifest.

Proof:
- Named CI gates pass: `rust_forbidden_deps`, `platform_parity`, `module_conformance`, `native_view_conformance`, `png_determinism`, `hot_reload_matrix`, `perf_budgets`.
- Performance report records thresholds and measured values for each budget.
- Release artifact manifest contains simulator, emulator, browser, PNG, parity, and graph-check artifacts.

Plan update:
- Mark PR37 complete.
- Add release gate receipts.
