use valdi_rust_hot_patch::{
    support_matrix, validate_hot_patch_support_matrix_snapshot, validate_support_matrix,
    HotPatchSupportStatus, RustEditClass,
};

#[test]
fn support_matrix_allows_only_action_body_live_patch() {
    validate_support_matrix().unwrap();
    let live = support_matrix()
        .iter()
        .filter(|decision| decision.status == HotPatchSupportStatus::LivePatch)
        .map(|decision| decision.edit_class)
        .collect::<Vec<_>>();

    assert_eq!(live, vec![RustEditClass::ActionBody]);
}

#[test]
fn support_matrix_snapshot_is_stable_and_drift_checked() {
    let expected = include_str!("../snapshots/hot_patch_support_matrix.snap");
    validate_hot_patch_support_matrix_snapshot(&expected).unwrap();

    let drift = expected.replace(
        "signature|rebuild_required|HOT_PATCH_SIGNATURE_REBUILD_REQUIRED|action_signature_changed",
        "signature|live_patch|HOT_PATCH_ACTION_BODY_SUPPORTED|action_body_changed",
    );
    let diagnostic = validate_hot_patch_support_matrix_snapshot(&drift).unwrap_err();
    assert_eq!(diagnostic.code, "HOT_PATCH_SUPPORT_MATRIX_DRIFT");
    assert_eq!(diagnostic.path, "$.rust_hot_patch.support_matrix");
}
