# PR05 Add Ir Codec Validator Inspector

Branch: `codex/rust-supervaldi-05-codec-validator`

Purpose: Add production and debug encoding plus validation tooling.

Depends on: PR04

Code boundary:
- Rust owns JSON debug codec, binary codec, validator, and inspect CLI.
- No JS runtime or C++ renderer dependency enters the codec crate.

Changes:
- Add JSON debug codec for readable development artifacts.
- Add `postcard` binary codec for production and hot reload payloads.
- Add validator generated from `replacement_contract.yaml`.
- Add `ir-inspect` CLI that prints schema path, node ID, source span, owner PR, fixture ID, and backend tag.
- Add codec compatibility tests for schema and binary version handling.

Proof:
- Every fixture round-trips through JSON and binary codecs.
- Invalid fixtures return exact diagnostic codes.
- `ir-inspect` output is stable in snapshot tests.

Plan update:
- Mark PR05 complete.
- Add codec and diagnostics receipts.
