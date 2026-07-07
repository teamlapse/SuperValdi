# PR36 Add Full Platform Parity Suite

Branch: `codex/rust-supervaldi-36-platform-parity`

Purpose: Prove retained and Rust backends match across every contract fixture before production release gates.

Depends on: PR35

Code boundary:
- Rust backend and retained compatibility path both run in parity tests.
- C++ retained renderer remains compatibility and transition-only.

Changes:
- Run contract fixtures through retained iOS and Android backends.
- Run contract fixtures through Rust iOS, Android, web DOM, PNG, and TS compatibility targets.
- Add normalized tree, attr, layout, event, animation, accessibility, native view, module, DOM, PNG, and diagnostic reports.
- Add parity dashboard artifact.

Proof:
- Every contract fixture passes declared parity targets.
- Every failure includes contract row ID, fixture ID, backend, platform, node ID, source span, and owner PR.
- Rust app graph remains free of TypeScript and C++ renderer dependencies during parity run.

Plan update:
- Mark PR36 complete.
- Add parity report.
