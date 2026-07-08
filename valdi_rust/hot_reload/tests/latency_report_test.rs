use valdi_rust_hot_reload::{
    deterministic_latency_measurements, validate_hot_reload_latency_report, PATCH_LATENCY_BUDGET_US,
};

#[test]
fn latency_report_is_deterministic_and_under_budget() {
    let expected = include_str!("../snapshots/hot_reload_latency.snap");
    validate_hot_reload_latency_report(expected).unwrap();
    let total = deterministic_latency_measurements()
        .iter()
        .map(|measurement| measurement.elapsed_us)
        .sum::<u32>();

    assert_eq!(total, 8_900);
    assert!(total < PATCH_LATENCY_BUDGET_US);
}

#[test]
fn latency_snapshot_negative_drift_fails() {
    let diagnostic = validate_hot_reload_latency_report("wrong\n").unwrap_err();

    assert_eq!(diagnostic.code, "HOT_RELOAD_LATENCY_DRIFT");
    assert_eq!(diagnostic.path, "$.hot_reload.latency");
}
