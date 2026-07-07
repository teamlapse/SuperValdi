# PR30 Port Text Rich Text And Text Input

Branch: `codex/rust-supervaldi-30-rust-text-input`

Purpose: Port text rendering and input behavior to Rust backend.

Depends on: PR29

Code boundary:
- Rust owns text model, rich text spans, input state, and text diagnostics.
- Platform hooks perform native measurement, font resolution, and input integration.

Changes:
- Add text measurement hooks.
- Add plain text, attributed spans, fonts, weight, line height, alignment, wrapping, truncation, and links.
- Add text input value, selection, composition, focus, blur, keyboard submit, and typed input events.
- Add fixed-font metrics test harness.

Proof:
- Text and text input fixtures pass on iOS, Android, and web.
- Fixed-font metrics match fixture expectations.
- Rust text crate has no dependency on retained `Text` renderer targets.

Plan update:
- Mark PR30 complete.
- Add text receipts.
