use valdi_rust_hot_reload::{
    generate_patch, parse_declarative_patch, validate_required_patch_family_coverage,
    HotReloadEditFamily, SUPPORTED_PATCH_INPUTS,
};

#[test]
fn generator_maps_supported_patches_to_backend_operations() {
    let generated = SUPPORTED_PATCH_INPUTS
        .iter()
        .map(|input| generate_patch(&parse_declarative_patch(input).unwrap()))
        .collect::<Vec<_>>();

    validate_required_patch_family_coverage(&generated).unwrap();
    assert!(generated
        .iter()
        .any(|patch| patch.intent.family() == HotReloadEditFamily::UiTree
            && patch
                .operation_families
                .iter()
                .any(|family| family.as_str() == "create")));
    assert!(generated.iter().any(|patch| {
        patch.intent.family() == HotReloadEditFamily::NativeViewRef
            && patch
                .operation_families
                .iter()
                .any(|family| family.as_str() == "native_view_mount")
    }));
}

#[test]
fn generator_family_coverage_fails_on_missing_patch_family() {
    let generated = SUPPORTED_PATCH_INPUTS
        .iter()
        .take(9)
        .map(|input| generate_patch(&parse_declarative_patch(input).unwrap()))
        .collect::<Vec<_>>();

    assert_eq!(
        validate_required_patch_family_coverage(&generated).unwrap_err(),
        HotReloadEditFamily::NativeViewRef
    );
}
