# PR04 Add Complete Ir Fixture Corpus

Branch: `codex/rust-supervaldi-04-fixture-corpus`

Status: Complete

Purpose: Add fixture coverage for every contract row and parity target.

Depends on: PR03

Code boundary:
- Fixtures are Rust and serialized IR assets.
- No platform backend implementation enters this PR.

Changes:
- Add canonical fixture manifest generated from the contract.
- Add fixtures for tree, layout, styling, text, assets, events, state, bindings, animations, modules, native views, accessibility, hot reload, diagnostics, web DOM, PNG, dynamic UI, TS compatibility, and build graph rows.
- Tag each fixture with required parity targets.
- Add invalid fixture corpus for diagnostics.
- Add Rust fixture crate metadata that statically references the canonical manifest, invalid manifest, and serialized fixture assets without adding PR05 codec behavior.

Proof:
- Fixture coverage test proves every contract row and fixture tag pair has coverage.
- Invalid fixture manifest proves every diagnostic family has at least one failing fixture.
- Fixture IDs remain stable across regeneration.
- `valdi_rust/fixtures/contract_fixture_manifest.json` is the PR04 fixture manifest.
- Serialized `.ir.json` assets are canonical debug fixtures only; production JSON and binary codec behavior remains PR05 scope.

Plan update:
- Mark PR04 complete.
- Add fixture manifest path.
