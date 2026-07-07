# PR19 Generate Swift And Kotlin Module Factories

Branch: `codex/rust-supervaldi-19-polyglot-ios-android`

Purpose: Generate native iOS and Android factories from Rust-authored module contracts.

Depends on: PR18

Code boundary:
- App contracts remain Rust.
- Swift and Kotlin implementation stubs are app-provided platform code.
- No app-facing C++ bridge code is generated.

Changes:
- Generate Swift-facing iOS factory glue.
- Generate Kotlin-facing Android factory glue.
- Add marshalling tests for scalar, collection, struct, enum, error, callback, stream, and constants across Swift and Kotlin.
- Add build labels for generated iOS and Android module fixtures.

Proof:
- Same module conformance passes on iOS and Android.
- Swift and Kotlin stubs call into the same Rust contract IDs.
- App module implementation requires no C++.

Plan update:
- Mark PR19 complete.
- Add iOS/Android module receipts.
