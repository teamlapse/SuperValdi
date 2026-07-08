use valdi_rust_hot_patch::{
    debug_build_report, release_build_report, sample_detected_action_body_patch,
    validate_release_exclusion, DevActionImplementationLoader, HotPatchBuildProfile,
};

#[test]
fn release_build_report_excludes_hot_patch_machinery_and_loader_symbols() {
    let report = release_build_report();
    assert_eq!(report.profile, HotPatchBuildProfile::Release);
    assert!(!report.hot_patch_machinery_present);
    assert!(report.dev_loader_symbols.is_empty());
    validate_release_exclusion(report).unwrap();

    let debug = debug_build_report();
    assert_eq!(debug.profile, HotPatchBuildProfile::Debug);
    assert!(debug.hot_patch_machinery_present);
    assert_eq!(debug.dev_loader_symbols.len(), 2);
    assert_eq!(
        validate_release_exclusion(debug).unwrap_err().code,
        "HOT_PATCH_RELEASE_EXCLUDED"
    );
}

#[test]
fn release_loader_rejects_action_body_patch() {
    let patch = sample_detected_action_body_patch().unwrap();
    let mut loader = DevActionImplementationLoader::new_release();
    let diagnostic = loader.apply_action_body_patch(patch).unwrap_err();

    assert_eq!(diagnostic.code, "HOT_PATCH_RELEASE_EXCLUDED");
    assert_eq!(diagnostic.path, "$.rust_hot_patch.release");
    assert_eq!(diagnostic.reason, "release_excludes_hot_patch_machinery");
}
