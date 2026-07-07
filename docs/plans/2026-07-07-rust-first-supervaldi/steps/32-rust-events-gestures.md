# PR32 Port Events And Gestures

Branch: `codex/rust-supervaldi-32-rust-events-gestures`

Purpose: Port event dispatch and gesture payloads to Rust backend.

Depends on: PR31

Code boundary:
- Rust owns event routing, typed payloads, action dispatch, and scheduling.
- Platform hooks capture native events and deliver normalized payloads.

Changes:
- Add tap/click, press, long press, pan/drag, scroll, focus, blur, text input, keyboard submit, layout, draw, visibility, frame observer, and custom native event payloads.
- Add event-to-action dispatch through Rust scheduler.
- Add event coalescing and cancellation tests.

Proof:
- Event and gesture fixtures reach Rust actions with typed payloads on iOS, Android, and web.
- Event ordering snapshots match fixture expectations.
- Rust event crate has no dependency on retained event renderer targets.

Plan update:
- Mark PR32 complete.
- Add event receipts.
