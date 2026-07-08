use valdi_rust_runtime::{
    state_binding_action_trace_snapshot, validate_state_binding_action_trace_snapshot,
    RUNTIME_STATE_ACTION_TRACE_DRIFT,
};

#[test]
fn state_binding_action_trace_snapshot_is_stable() {
    let expected = include_str!("../snapshots/state_binding_action_trace.snap");
    let actual = state_binding_action_trace_snapshot().expect("trace renders");

    assert_eq!(actual, expected);
}

#[test]
fn removed_invalidation_fails_trace_snapshot_validation() {
    let expected = include_str!("../snapshots/state_binding_action_trace.snap");
    let drifted = expected
        .lines()
        .filter(|line| !line.contains("action save status=completed"))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";

    let diagnostic = validate_state_binding_action_trace_snapshot(&drifted)
        .expect_err("removed invalidation line must fail");
    assert_eq!(diagnostic.code, RUNTIME_STATE_ACTION_TRACE_DRIFT);
    assert_eq!(diagnostic.path, "$.state_binding_action_trace");
    assert_eq!(diagnostic.severity.as_str(), "error");
}
