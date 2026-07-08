# PR08 Add State Bindings And Action Scheduler

Branch: `codex/rust-supervaldi-08-state-bindings-actions`

Purpose: Add Rust state, binding evaluation, and action dispatch.

Depends on: PR07

Status: Complete

Code boundary:
- Rust owns state storage, binding resolver, expression evaluator, action scheduler, and typed errors.
- No JavaScript action runtime is used by Rust-authored apps.

Changes:
- Add field paths, literals, nullable values, boolean logic, comparisons, list iteration, computed projections, platform constants, and source spans.
- Add sync and async action dispatch with typed results, typed errors, cancellation identity, coalescing, and scheduling.
- Add state invalidation and retained state compatibility checks.
- Add event-to-action binding table.

Proof:
- `//valdi_rust:state_bindings_actions_tests` covers typed state storage, binding evaluation, action scheduling, event-to-action resolution, retained state compatibility, and trace snapshot stability.
- `//valdi_rust/runtime:state_store_test` proves typed get/set/update/invalidation and exact missing or wrong-type diagnostics.
- `//valdi_rust/runtime:binding_evaluator_test` proves field paths, nullable values, boolean logic, comparisons, list projections, computed projections, platform constants, source span diagnostics, and missing-field rejection.
- `//valdi_rust/runtime:action_scheduler_test` proves sync result, deterministic async simulation, typed error, cancellation identity, coalescing, scheduling policy, invalidation output, and canceled-action reuse rejection.
- `//valdi_rust/runtime:event_action_binding_test` proves typed event-to-action resolution and missing action/mapping diagnostics.
- `//valdi_rust/runtime:state_patch_compatibility_test` proves compatible runtime patches retain state and incompatible state identity returns exact diagnostics.
- `//valdi_rust/runtime:state_binding_action_snapshot_test` checks the deterministic state/binding/action trace snapshot and removed-invalidation negative case.

Plan update:
- Marked PR08 complete.
- Added action/binding receipts.
