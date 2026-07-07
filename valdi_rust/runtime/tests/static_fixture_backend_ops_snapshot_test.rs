use valdi_rust_runtime::{
    static_fixture_backend_ops_snapshot, validate_static_fixture_backend_ops_snapshot,
};

#[test]
fn static_fixture_backend_ops_snapshot_is_stable() {
    let expected = include_str!("../snapshots/static_fixture_backend_ops.snap");
    let actual = static_fixture_backend_ops_snapshot().expect("snapshot renders");

    assert_eq!(actual, expected);
}

#[test]
fn missing_expected_backend_operation_fails_snapshot_validation() {
    let expected = include_str!("../snapshots/static_fixture_backend_ops.snap");
    let drifted = expected
        .lines()
        .filter(|line| !line.contains("child=node.item.stable"))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";

    let diagnostic = validate_static_fixture_backend_ops_snapshot(&drifted)
        .expect_err("missing backend op must fail");
    assert_eq!(diagnostic.code, "RUNTIME_BACKEND_OP_SNAPSHOT_DRIFT");
    assert_eq!(diagnostic.path, "$.runtime_tree_diff_snapshot");
    assert_eq!(diagnostic.severity.as_str(), "error");
}
