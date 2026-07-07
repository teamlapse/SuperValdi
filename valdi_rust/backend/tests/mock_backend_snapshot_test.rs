use valdi_rust_backend::mock_backend_snapshot;

#[test]
fn mock_backend_snapshot_is_stable() {
    assert_eq!(
        mock_backend_snapshot(),
        include_str!("../snapshots/fixture_tag_backend_ops.snap")
    );
}
