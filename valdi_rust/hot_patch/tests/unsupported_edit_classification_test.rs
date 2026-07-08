use valdi_rust_hot_patch::{
    detect_action_body_hot_patch, sample_action_body_before_fingerprint,
    sample_unsupported_after_fingerprint, support_decision, HotPatchDiagnosticSeverity,
    HotPatchSupportStatus, RustEditClass, REQUIRED_UNSUPPORTED_EDIT_CLASSES,
};

#[test]
fn every_required_unsupported_edit_class_has_exact_rebuild_diagnostic() {
    assert_eq!(
        REQUIRED_UNSUPPORTED_EDIT_CLASSES,
        &[
            RustEditClass::Signature,
            RustEditClass::Type,
            RustEditClass::Module,
            RustEditClass::StateShape,
            RustEditClass::Dependency,
            RustEditClass::Macro,
            RustEditClass::CrateGraph,
            RustEditClass::PlatformBoundary,
        ]
    );

    for edit_class in REQUIRED_UNSUPPORTED_EDIT_CLASSES {
        let diagnostic = detect_action_body_hot_patch(
            sample_action_body_before_fingerprint(),
            sample_unsupported_after_fingerprint(*edit_class),
        )
        .unwrap_err();
        let decision = support_decision(*edit_class);
        assert_eq!(decision.status, HotPatchSupportStatus::RebuildRequired);
        assert_eq!(diagnostic.code, decision.diagnostic_code);
        assert_eq!(diagnostic.path, edit_class.path());
        assert_eq!(diagnostic.severity, HotPatchDiagnosticSeverity::Error);
        assert_eq!(diagnostic.reason, decision.reason);
        assert!(diagnostic.message.contains(edit_class.as_str()));
    }
}

#[test]
fn unchanged_action_body_is_not_silently_applied() {
    let diagnostic = detect_action_body_hot_patch(
        sample_action_body_before_fingerprint(),
        sample_action_body_before_fingerprint(),
    )
    .unwrap_err();

    assert_eq!(diagnostic.code, "HOT_PATCH_NO_ACTION_BODY_CHANGE");
    assert_eq!(diagnostic.path, "$.rust_hot_patch.action_body");
    assert_eq!(diagnostic.reason, "action_body_unchanged");
}
