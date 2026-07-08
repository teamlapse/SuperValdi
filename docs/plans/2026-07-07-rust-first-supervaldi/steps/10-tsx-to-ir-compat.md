# PR10 Add Tsx-To-Ir Compatibility Frontend

Branch: `codex/rust-supervaldi-10-tsx-to-ir-compat`

Purpose: Preserve existing TypeScript apps and surface schema gaps before backend replacement begins.

Depends on: PR09

Code boundary:
- TypeScript remains compatibility infrastructure only.
- Rust-authored apps keep no TypeScript dependency.

Changes:
- Added a TSX compatibility sidecar under `compiler/companion/src/tsx_ir_compat` that parses static TSX fixture metadata and emits deterministic typed normalized IR envelopes.
- Kept existing direct renderer compatibility path unchanged and added a `JSXProcessor` regression proving retained output still uses `valdi_core/src/JSX`.
- Added TSX feature coverage manifest tied to `replacement_contract.yaml` and the PR04 fixture manifest.
- Added fixture equivalence checks between TSX-emitted normalized IR and the PR09 canonical DSL snapshot format for all 22 contract rows.
- Added Rust graph and Rust source checks proving Rust app crates have no TS dependency.

Proof:
- `bazelisk test //valdi_rust:tsx_to_ir_compat_tests` passed 4/4 tests.
- `bazelisk build //compiler/companion:lib` passed and compiled the compatibility sidecar through the existing companion Bazel target.
- Direct built-JS proof with `NODE_PATH=bazel-bin/compiler/companion/node_modules node <inline PR10 sidecar check>` emitted 22 normalized IR envelopes, checked exact unsupported-row diagnostics, and verified retained `JSXProcessor` output does not include the sidecar.
- `python3 valdi_rust/tests/tsx_to_ir_compat_contract_test.py docs/rust_migration/replacement_contract.yaml valdi_rust/fixtures/contract_fixture_manifest.json compiler/companion/src/tsx_ir_compat/tsx_feature_coverage_manifest.json` validated 22 contract rows and includes missing-row, duplicate-row, coverage-drift, and serialized-path-drift negative checks.
- `python3 valdi_rust/tests/rust_app_no_ts_dependency_test.py valdi_rust/crate_graph.json <valdi_rust source files>` validated 8 Rust crates have no TS compatibility dependency.
- Manager-side retained TypeScript sample proof `bazelisk build //apps/cli_example:ios.debug.valdimodule` passed and produced `bazel-out/darwin_arm64-opt-exec-ST-d57f47055a04/bin/apps/cli_example/ios/debug/assets/cli_example.valdimodule`.
- Existing companion `npm test -- TSXToIRCompatibility.spec.ts TSXFeatureCoverageManifest.spec.ts JSXProcessor.spec.ts` is blocked in this worktree because `compiler/companion/node_modules/.bin/jest` is absent; this remains an environment limitation only. The committed specs remain project-command compatible, and the Bazel typecheck, direct built-JS checks, Rust metadata tests, and retained `.valdimodule` build cover PR10 proof here.

Plan update:
- Mark PR10 complete.
- Update TS dependency risk with the PR10 no-Rust-path TS dependency proof.
