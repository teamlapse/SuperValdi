use valdi_rust_dynamic_ui::{
    fixtures::{contract_document, dynamic_ui_fixture},
    load_in_memory, DynamicUiSourceMetadata, DynamicUiTrustMetadata, DYNAMIC_UI_NODE_ID_DRIFT,
    DYNAMIC_UI_OWNER_PR_MISMATCH, DYNAMIC_UI_SCHEMA_PATH_INVALID, DYNAMIC_UI_SOURCE_UNTRUSTED,
    VALIDATED_IR_SCHEMA_PATH,
};

const CONTRACT_JSON: &str = include_str!("../../../docs/rust_migration/replacement_contract.yaml");

#[test]
fn untrusted_source_returns_exact_source_trust_node_and_owner_context() {
    let contract = contract_document(CONTRACT_JSON).expect("contract parses");
    let fixture = dynamic_ui_fixture().expect("fixture parses");

    let diagnostic = load_in_memory(
        fixture,
        &contract,
        DynamicUiSourceMetadata::json_debug("dynamic_ui.json", VALIDATED_IR_SCHEMA_PATH),
        DynamicUiTrustMetadata::untrusted("PR13"),
    )
    .expect_err("untrusted dynamic source must fail");

    assert_eq!(diagnostic.code, DYNAMIC_UI_SOURCE_UNTRUSTED);
    assert_eq!(diagnostic.path, "$.trust.level");
    assert_eq!(diagnostic.source.source_id(), "dynamic_ui.json");
    assert_eq!(diagnostic.trust.level.as_str(), "untrusted");
    assert_eq!(diagnostic.node_id.as_deref(), Some("node.dynamic_ui.root"));
    assert_eq!(diagnostic.owner_pr.as_deref(), Some("PR01,PR05,PR13"));
}

#[test]
fn invalid_schema_path_is_rejected_before_runtime_bridge() {
    let contract = contract_document(CONTRACT_JSON).expect("contract parses");
    let fixture = dynamic_ui_fixture().expect("fixture parses");

    let diagnostic = load_in_memory(
        fixture,
        &contract,
        DynamicUiSourceMetadata::json_debug("dynamic_ui.json", "$.wrong"),
        DynamicUiTrustMetadata::generated_fixture("PR13"),
    )
    .expect_err("wrong schema path must fail");

    assert_eq!(diagnostic.code, DYNAMIC_UI_SCHEMA_PATH_INVALID);
    assert_eq!(diagnostic.path, "$.source.schema_path");
    assert_eq!(diagnostic.severity.as_str(), "error");
    assert_eq!(diagnostic.source.schema_path, "$.wrong");
}

#[test]
fn owner_pr_mismatch_is_rejected_with_exact_metadata_path() {
    let contract = contract_document(CONTRACT_JSON).expect("contract parses");
    let fixture = dynamic_ui_fixture().expect("fixture parses");

    let diagnostic = load_in_memory(
        fixture,
        &contract,
        DynamicUiSourceMetadata::generated_fixture(
            "contract.dynamic_ui.v1",
            VALIDATED_IR_SCHEMA_PATH,
        ),
        DynamicUiTrustMetadata::generated_fixture("PR99"),
    )
    .expect_err("owner mismatch must fail");

    assert_eq!(diagnostic.code, DYNAMIC_UI_OWNER_PR_MISMATCH);
    assert_eq!(diagnostic.path, "$.metadata.owner_prs");
    assert_eq!(diagnostic.trust.owner_pr, "PR99");
    assert_eq!(diagnostic.owner_pr.as_deref(), Some("PR01,PR05,PR13"));
}

#[test]
fn node_id_drift_is_rejected_with_schema_node_and_owner_context() {
    let contract = contract_document(CONTRACT_JSON).expect("contract parses");
    let mut fixture = dynamic_ui_fixture().expect("fixture parses");
    fixture.ir_debug.root_node_id = "node.dynamic_ui.changed".to_string();

    let diagnostic = load_in_memory(
        fixture,
        &contract,
        DynamicUiSourceMetadata::in_memory("node_drift", VALIDATED_IR_SCHEMA_PATH),
        DynamicUiTrustMetadata::first_party("PR13"),
    )
    .expect_err("node drift must fail");

    assert_eq!(diagnostic.code, DYNAMIC_UI_NODE_ID_DRIFT);
    assert_eq!(diagnostic.path, "$.ir_debug.root_node_id");
    assert_eq!(
        diagnostic.node_id.as_deref(),
        Some("node.dynamic_ui.changed")
    );
    assert_eq!(diagnostic.owner_pr.as_deref(), Some("PR01,PR05,PR13"));
}
