use valdi_rust_hot_reload::{
    HotReloadSampleHost, HOT_RELOAD_ACTION_MISSING, HOT_RELOAD_BINDING_MISSING,
};
use valdi_rust_ir::hot_reload::CompatibilityClass;

#[test]
fn binding_patch_resolves_against_retained_state() {
    let mut host = HotReloadSampleHost::new().unwrap();

    host.apply_declarative(
        "binding patch_id=patch.binding_ok binding=binding.title path=app.user.name span=hot_reload.dsl:30:1",
    )
    .unwrap();
}

#[test]
fn missing_binding_field_returns_exact_rebuild_required_diagnostic() {
    let mut host = HotReloadSampleHost::new().unwrap();
    let diagnostic = host
        .apply_declarative(
            "binding patch_id=patch.binding_bad binding=binding.missing path=app.user.missing span=hot_reload.dsl:31:1",
        )
        .unwrap_err();

    assert_eq!(diagnostic.code, HOT_RELOAD_BINDING_MISSING);
    assert_eq!(diagnostic.path, "$.patch.binding.expression");
    assert_eq!(
        diagnostic.reason,
        "binding expression cannot resolve in retained state"
    );
    assert_eq!(
        diagnostic.compatibility,
        CompatibilityClass::RequiresRebuild
    );
}

#[test]
fn missing_event_action_returns_exact_rebuild_required_diagnostic() {
    let mut host = HotReloadSampleHost::new().unwrap();
    let diagnostic = host
        .apply_declarative(
            "event patch_id=patch.event_bad node=node.button event=tap action=action.missing span=hot_reload.dsl:32:1",
        )
        .unwrap_err();

    assert_eq!(diagnostic.code, HOT_RELOAD_ACTION_MISSING);
    assert_eq!(diagnostic.path, "$.patch.event.action_id");
    assert_eq!(
        diagnostic.reason,
        "event action is not present in the retained scheduler"
    );
    assert_eq!(
        diagnostic.compatibility,
        CompatibilityClass::RequiresRebuild
    );
}
