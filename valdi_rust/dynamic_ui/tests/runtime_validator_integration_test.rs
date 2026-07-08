use valdi_rust_dynamic_ui::{
    apply_to_mock_backend,
    fixtures::{contract_document, dynamic_ui_fixture},
    load_in_memory, DynamicUiSourceMetadata, DynamicUiTrustMetadata, VALIDATED_IR_SCHEMA_PATH,
};

const CONTRACT_JSON: &str = include_str!("../../../docs/rust_migration/replacement_contract.yaml");

#[test]
fn contract_validator_diagnostics_are_preserved_for_dynamic_ir() {
    let contract = contract_document(CONTRACT_JSON).expect("contract parses");
    let mut fixture = dynamic_ui_fixture().expect("fixture parses");
    fixture.ir_debug.coverage_tokens.pop();

    let diagnostic = load_in_memory(
        fixture,
        &contract,
        DynamicUiSourceMetadata::in_memory("contract_drift", VALIDATED_IR_SCHEMA_PATH),
        DynamicUiTrustMetadata::first_party("PR13"),
    )
    .expect_err("contract drift must fail");

    assert_eq!(diagnostic.code, "IR_CONTRACT_DRIFT");
    assert_eq!(diagnostic.path, "$.ir_debug.coverage_tokens");
    assert_eq!(diagnostic.severity.as_str(), "error");
    assert_eq!(diagnostic.source.source_id(), "contract_drift");
    assert_eq!(diagnostic.trust.owner_pr, "PR13");
    assert_eq!(diagnostic.node_id.as_deref(), Some("node.dynamic_ui.root"));
}

#[test]
fn runtime_validator_and_mock_backend_accept_valid_dynamic_document() {
    let contract = contract_document(CONTRACT_JSON).expect("contract parses");
    let fixture = dynamic_ui_fixture().expect("fixture parses");
    let document = load_in_memory(
        fixture,
        &contract,
        DynamicUiSourceMetadata::in_memory("runtime_acceptance", VALIDATED_IR_SCHEMA_PATH),
        DynamicUiTrustMetadata::first_party("PR13"),
    )
    .expect("dynamic document loads");

    assert!(document
        .runtime_document
        .contains_node(document.runtime_document.root.node_id));
    let receipt = apply_to_mock_backend(&document.runtime_document).expect("mock backend accepts");
    assert!(receipt
        .snapshot
        .contains("family=create capability=view_tree"));
    assert!(receipt
        .snapshot
        .contains("family=root capability=view_tree"));
}
