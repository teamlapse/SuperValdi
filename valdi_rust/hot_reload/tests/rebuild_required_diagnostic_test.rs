use valdi_rust_hot_reload::{
    HotReloadSampleHost, ACTION_BODY_PATCH_INPUT, HOT_RELOAD_REBUILD_REQUIRED,
};
use valdi_rust_ir::hot_reload::CompatibilityClass;

#[test]
fn action_body_patch_returns_exact_rebuild_required_diagnostic() {
    let mut host = HotReloadSampleHost::new().unwrap();
    let before = host.state_token().unwrap();
    let diagnostic = host.apply_declarative(ACTION_BODY_PATCH_INPUT).unwrap_err();
    let after = host.state_token().unwrap();

    assert_eq!(diagnostic.code, HOT_RELOAD_REBUILD_REQUIRED);
    assert_eq!(diagnostic.path, "$.patch.action_body");
    assert_eq!(
        diagnostic.reason,
        "action body changes are owned by the Rust logic hot patch step"
    );
    assert_eq!(
        diagnostic.compatibility,
        CompatibilityClass::RequiresRebuild
    );
    assert_eq!(diagnostic.severity.as_str(), "error");
    assert_eq!(
        diagnostic.source_span_id.unwrap().as_str(),
        "hot_reload.dsl:11:1"
    );
    assert_eq!(before, after);
    assert_eq!(host.full_rebuilds(), 0);
}
