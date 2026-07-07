# PR04 Add Complete Ir Fixture Corpus

Branch: `codex/rust-supervaldi-04-fixture-corpus`

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

Proof:
- Fixture coverage test proves every contract row and fixture tag pair has coverage.
- Invalid fixture manifest proves every diagnostic family has at least one failing fixture.
- Fixture IDs remain stable across regeneration.

Plan update:
- Mark PR04 complete.
- Add fixture manifest path.
