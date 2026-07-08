use valdi_rust_hot_reload::{
    HotReloadSampleHost, HOT_RELOAD_MODULE_CONTRACT_SHAPE_CHANGED,
    HOT_RELOAD_NATIVE_VIEW_CONTRACT_SHAPE_CHANGED, MODULE_SHAPE_DRIFT_INPUT,
    NATIVE_VIEW_SHAPE_DRIFT_INPUT,
};
use valdi_rust_ir::hot_reload::CompatibilityClass;

#[test]
fn module_ref_patch_accepts_registered_shape() {
    let mut host = HotReloadSampleHost::new().unwrap();

    host.apply_declarative(
        "module_ref patch_id=patch.module_ok module=module.storage shape=storage.v1 span=hot_reload.dsl:20:1",
    )
    .unwrap();
}

#[test]
fn module_ref_patch_rejects_contract_shape_change() {
    let mut host = HotReloadSampleHost::new().unwrap();
    let diagnostic = host
        .apply_declarative(MODULE_SHAPE_DRIFT_INPUT)
        .unwrap_err();

    assert_eq!(diagnostic.code, HOT_RELOAD_MODULE_CONTRACT_SHAPE_CHANGED);
    assert_eq!(diagnostic.path, "$.patch.module_ref.shape");
    assert_eq!(
        diagnostic.compatibility,
        CompatibilityClass::RequiresRebuild
    );
    assert_eq!(diagnostic.reason, "module contract shape changed");
}

#[test]
fn native_view_ref_patch_rejects_contract_shape_change() {
    let mut host = HotReloadSampleHost::new().unwrap();
    let diagnostic = host
        .apply_declarative(NATIVE_VIEW_SHAPE_DRIFT_INPUT)
        .unwrap_err();

    assert_eq!(
        diagnostic.code,
        HOT_RELOAD_NATIVE_VIEW_CONTRACT_SHAPE_CHANGED
    );
    assert_eq!(diagnostic.path, "$.patch.native_view_ref.shape");
    assert_eq!(
        diagnostic.compatibility,
        CompatibilityClass::RequiresRebuild
    );
    assert_eq!(diagnostic.reason, "native view contract shape changed");
}
