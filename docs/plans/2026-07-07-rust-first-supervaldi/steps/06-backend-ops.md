# PR06 Add Full Backend Operation Model

Branch: `codex/rust-supervaldi-06-backend-ops`

Purpose: Create the backend operation contract shared by native, web, PNG, and transition renderers.

Depends on: PR05

Code boundary:
- Rust owns backend op enum, render backend trait, operation validator, and mock backend.
- C++ remains untouched except referenced in transition fixture names.

Changes:
- Add backend op enum for create, destroy, root, move, attr, animation, layout callback, draw callback, visibility, frame observer, asset load, text measure, native view mount, webview mount, and transaction group.
- Add render backend trait with typed capability negotiation.
- Add backend operation validator.
- Add mock backend snapshots for every fixture tag.

Proof:
- Mock backend snapshots cover every operation family.
- Operation validator rejects unsupported backend capability use with exact diagnostic.
- Backend trait compiles on host, iOS, Android, WASM, and PNG crate targets.

Plan update:
- Mark PR06 complete.
- Add snapshot path.
