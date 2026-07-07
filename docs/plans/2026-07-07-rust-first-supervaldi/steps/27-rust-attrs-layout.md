# PR27 Port Attrs Layout And Yoga Orchestration

Branch: `codex/rust-supervaldi-27-rust-attrs-layout`

Purpose: Port layout and attribute handling to the Rust backend.

Depends on: PR26

Code boundary:
- Rust owns typed attr registry and layout orchestration.
- Yoga remains the layout engine through existing or Rust-bound bindings.

Changes:
- Port typed attr registry.
- Port Yoga orchestration, invalidation, measurement hooks, scroll sizing, safe area, RTL, z order, clipping, transforms, border, radius, shadow/elevation, visibility, and display.
- Add retained-backend attr/layout comparison runner.

Proof:
- Layout and attr metrics match retained backend for all layout fixtures.
- Unsupported backend capability returns exact diagnostic.
- Rust attrs/layout crates have no dependency on `AttributesManager` or `ViewNodeRenderer`.

Plan update:
- Mark PR27 complete.
- Add layout parity receipts.
