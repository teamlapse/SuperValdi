use valdi_rust_hot_reload::{hot_reload_trace_snapshot, validate_hot_reload_trace_snapshot};

#[test]
fn sample_host_trace_snapshot_is_stable() {
    let expected = include_str!("../snapshots/hot_reload_trace.snap");
    validate_hot_reload_trace_snapshot(expected).unwrap();
    assert!(hot_reload_trace_snapshot()
        .unwrap()
        .contains("family=native_view_ref"));
}

#[test]
fn sample_host_trace_negative_drift_fails() {
    let diagnostic = validate_hot_reload_trace_snapshot("wrong\n").unwrap_err();

    assert_eq!(diagnostic.code, "HOT_RELOAD_TRACE_DRIFT");
    assert_eq!(diagnostic.path, "$.hot_reload.trace");
}
