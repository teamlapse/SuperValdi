# PR05 Add Ir Codec Validator Inspector

Branch: `codex/rust-supervaldi-05-codec-validator`

Purpose: Add production and debug encoding plus validation tooling.

Depends on: PR04

Status: Complete

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
- `//valdi_rust/codec:roundtrip_test` proves all 22 PR04 serialized fixtures round-trip through typed JSON debug and postcard binary codecs.
- `//valdi_rust/codec:validator_test` proves contract-bound fixture validation, exact invalid diagnostics for all 8 invalid fixtures, and contract row/tag drift rejection.
- `//valdi_rust/codec:version_compatibility_test` proves accepted current versions plus unsupported schema, unsupported binary wire, and corrupted binary diagnostics.
- `//valdi_rust/codec:inspect_snapshot_test` proves `ir-inspect` output is stable for schema path, node ID, source span, owner PR, fixture ID, and backend tag.

Plan update:
- Marked PR05 complete.
- Added codec, validator, diagnostics, and inspect receipts.
