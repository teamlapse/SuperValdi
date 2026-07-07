use valdi_rust_runtime::{
    apply_identity_patch, incompatible_patch_pair, RuntimeDiagnosticSeverity,
};

#[test]
fn incompatible_identity_patch_returns_rebuild_required_with_source_span() {
    let (before, after) = incompatible_patch_pair().expect("incompatible patch fixtures load");
    let diagnostic = apply_identity_patch(&before, &after)
        .expect_err("component identity change requires rebuild");

    assert_eq!(diagnostic.code, "RUNTIME_REBUILD_REQUIRED");
    assert_eq!(diagnostic.path, "$.document.identity.component_id");
    assert_eq!(diagnostic.severity, RuntimeDiagnosticSeverity::Error);
    assert_eq!(
        diagnostic.source_span_id.expect("source span").as_str(),
        "incompatible_after:9:5"
    );
}
