# PR00 Rust-First Production Migration

Branch: `codex/rust-supervaldi-00-plan`

Purpose: Add the production migration plan and separate step detail files only.

Depends on: `main`

Code boundary:
- Markdown docs only.
- No Rust, C, C++, Swift, Kotlin, JavaScript, TypeScript, or Bazel implementation enters this PR.

Changes:
- Add `docs/plans/2026-07-07-rust-first-supervaldi.md`.
- Add this `steps/` directory with one detail file for every stack PR.
- Record repo facts, production definition, no-gaps rules, coverage summary, stack order, risks, roadmap, validation, and quality ledger.

Proof:
- Plan PR contains docs only.
- Ambiguity wording scan passes.
- Missing-surface scan passes.
- User accepts this plan before implementation branches resume.

Plan update:
- Mark PR00 complete after merge.
- Set PR01 as the first implementation branch.
