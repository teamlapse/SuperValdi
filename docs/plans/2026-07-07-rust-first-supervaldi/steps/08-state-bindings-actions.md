# PR08 Add State Bindings And Action Scheduler

Branch: `codex/rust-supervaldi-08-state-bindings-actions`

Purpose: Add Rust state, binding evaluation, and action dispatch.

Depends on: PR07

Code boundary:
- Rust owns state storage, binding resolver, expression evaluator, action scheduler, and typed errors.
- No JavaScript action runtime is used by Rust-authored apps.

Changes:
- Add field paths, literals, nullable values, boolean logic, comparisons, list iteration, computed projections, platform constants, and source spans.
- Add sync and async action dispatch with typed results, typed errors, cancellation identity, coalescing, and scheduling.
- Add state invalidation and retained state compatibility checks.
- Add event-to-action binding table.

Proof:
- State fixtures update predictably.
- Binding fixtures resolve exact values or diagnostics.
- Action cancellation and typed error fixtures pass.
- Compatible state survives runtime patches.

Plan update:
- Mark PR08 complete.
- Add action/binding receipts.
