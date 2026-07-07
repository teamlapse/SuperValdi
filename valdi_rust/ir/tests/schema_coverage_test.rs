use valdi_rust_ir::schema::{CONTRACT_ROW_COUNT, CONTRACT_ROW_COVERAGE};

#[test]
fn all_contract_rows_have_public_schema_type_coverage() {
    assert_eq!(CONTRACT_ROW_COVERAGE.len(), CONTRACT_ROW_COUNT);

    let expected = [
        "schema_versioning",
        "component_identity",
        "tree_structure",
        "element_taxonomy",
        "layout",
        "styling",
        "text",
        "assets",
        "events_gestures",
        "actions_state",
        "bindings_expressions",
        "animations",
        "native_modules",
        "native_views",
        "accessibility",
        "hot_reload",
        "diagnostics",
        "web_dom",
        "png_backend",
        "dynamic_ui",
        "ts_compatibility",
        "build_graph",
    ];

    for expected_id in expected {
        let row = CONTRACT_ROW_COVERAGE
            .iter()
            .find(|row| row.id == expected_id)
            .unwrap_or_else(|| panic!("missing IR schema coverage for {expected_id}"));
        assert!(!row.surface.is_empty(), "{expected_id} must name its contract surface");
        assert!(!row.module.is_empty(), "{expected_id} must name its Rust module");
        assert!(
            !row.public_types.is_empty(),
            "{expected_id} must name public Rust schema types"
        );
    }
}

#[test]
fn stable_identity_newtypes_cover_required_id_families() {
    let ids = [
        valdi_rust_ir::ids::NodeId::new("node").as_str(),
        valdi_rust_ir::ids::ComponentId::new("component").as_str(),
        valdi_rust_ir::ids::StateId::new("state").as_str(),
        valdi_rust_ir::ids::ActionId::new("action").as_str(),
        valdi_rust_ir::ids::BindingId::new("binding").as_str(),
        valdi_rust_ir::ids::ModuleId::new("module").as_str(),
        valdi_rust_ir::ids::NativeViewId::new("native_view").as_str(),
        valdi_rust_ir::ids::AssetId::new("asset").as_str(),
        valdi_rust_ir::ids::SourceSpanId::new("source_span").as_str(),
        valdi_rust_ir::ids::DiagnosticId::new("diagnostic").as_str(),
        valdi_rust_ir::ids::SchemaVersionId::new("schema").as_str(),
        valdi_rust_ir::ids::CapabilityId::new("capability").as_str(),
    ];

    assert_eq!(ids.len(), 12);
    assert!(ids.contains(&"node"));
    assert!(ids.contains(&"capability"));
}
