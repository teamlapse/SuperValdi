# PR17 Add Deterministic Png Display-List Backend

Branch: `codex/rust-supervaldi-17-png-display-list`

Purpose: Add deterministic raster output for the base static render surface.

Depends on: PR13

Code boundary:
- Rust owns display-list generation and raster backend.
- No platform renderer dependency enters PNG tests.

Changes:
- Add display list model.
- Add raster backend for layout boxes, backgrounds, borders, clipping, transforms, and static images.
- Add deterministic font/image resource loading hooks used by PR25.
- Add golden test harness and deterministic output metadata.

Proof:
- Golden PNGs pass for base static-render fixtures.
- PNG backend consumes validated IR and backend ops generated from the shared runtime.
- Golden metadata records schema version, fixture ID, backend version, font set, image set, and platform scale.

Plan update:
- Mark PR17 complete.
- Add base PNG artifact path.
