# PR18 Add Rust-Authored Module Contract Idl

Branch: `codex/rust-supervaldi-18-polyglot-contract`

Purpose: Add the app-facing Rust contract surface for native modules.

Depends on: PR13

Code boundary:
- App developers write module contracts in Rust.
- Generator implementation follows existing repo infrastructure.
- No app developer writes C or C++ bridge code.

Changes:
- Add Rust module contract macro or IDL crate.
- Support bool, signed/unsigned ints, f32, f64, string, bytes, nullable value, list, map, struct, enum, typed error, callback, stream, constants, and platform dispatch targets in the contract model.
- Integrate contract metadata with existing generator discovery.
- Add invalid signature diagnostics.

Proof:
- Contract type matrix test passes.
- Unsupported Rust signatures fail generation with exact alternatives.
- Generated metadata contains target factories for Swift, Kotlin, JS/DOM, Rust host, C++ transition, and TS compatibility.

Plan update:
- Mark PR18 complete.
- Update module codegen risk.
