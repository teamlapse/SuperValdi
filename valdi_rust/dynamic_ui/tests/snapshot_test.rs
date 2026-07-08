use valdi_rust_dynamic_ui::{
    capabilities::{DynamicUiProducerCapability, DynamicUiProducerCapabilitySet},
    fixtures::{
        contract_document, dynamic_ui_fixture, dynamic_ui_trace_snapshot,
        invalid_diagnostics_snapshot, validate_invalid_diagnostics_snapshot,
        validate_trace_snapshot,
    },
    load_in_memory, validate_capability_set, DynamicUiSourceMetadata, DynamicUiTrustMetadata,
    DYNAMIC_UI_INVALID_DIAGNOSTIC_DRIFT, DYNAMIC_UI_TRACE_DRIFT, VALIDATED_IR_SCHEMA_PATH,
};

const CONTRACT_JSON: &str = include_str!("../../../docs/rust_migration/replacement_contract.yaml");
const IN_MEMORY_ONLY: &[DynamicUiProducerCapability] = &[
    DynamicUiProducerCapability::InMemoryInput,
    DynamicUiProducerCapability::RuntimeValidation,
    DynamicUiProducerCapability::RuntimeBridge,
];

#[test]
fn dynamic_ui_trace_snapshot_is_stable() {
    let expected = include_str!("../snapshots/dynamic_ui_trace.snap");
    let actual = dynamic_ui_trace_snapshot(CONTRACT_JSON).expect("snapshot renders");
    assert_eq!(actual, expected);

    let drifted = expected
        .lines()
        .filter(|line| !line.contains("family=root"))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    let diagnostic =
        validate_trace_snapshot(CONTRACT_JSON, &drifted).expect_err("removed op must fail");
    assert_eq!(diagnostic.code, DYNAMIC_UI_TRACE_DRIFT);
    assert_eq!(diagnostic.path, "$.dynamic_ui.trace");
}

#[test]
fn invalid_diagnostics_snapshot_is_stable() {
    let expected = include_str!("../snapshots/dynamic_ui_invalid_diagnostics.snap");
    let diagnostics = invalid_diagnostics();
    let actual = invalid_diagnostics_snapshot(&diagnostics);
    assert_eq!(actual, expected);

    let drifted = expected
        .lines()
        .filter(|line| !line.contains("DYNAMIC_UI_OWNER_PR_MISMATCH"))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    let diagnostic = validate_invalid_diagnostics_snapshot(&diagnostics, &drifted)
        .expect_err("removed diagnostic must fail");
    assert_eq!(diagnostic.code, DYNAMIC_UI_INVALID_DIAGNOSTIC_DRIFT);
    assert_eq!(diagnostic.path, "$.dynamic_ui.invalid_diagnostics");
}

fn invalid_diagnostics() -> Vec<valdi_rust_dynamic_ui::DynamicUiDiagnostic> {
    let contract = contract_document(CONTRACT_JSON).expect("contract parses");
    let fixture = dynamic_ui_fixture().expect("fixture parses");

    let capability = validate_capability_set(
        DynamicUiProducerCapabilitySet::new(IN_MEMORY_ONLY),
        DynamicUiSourceMetadata::in_memory("memory", VALIDATED_IR_SCHEMA_PATH),
        DynamicUiTrustMetadata::first_party("PR13"),
    )
    .expect_err("capability mismatch must fail");

    let untrusted = load_in_memory(
        fixture.clone(),
        &contract,
        DynamicUiSourceMetadata::json_debug("dynamic_ui.json", VALIDATED_IR_SCHEMA_PATH),
        DynamicUiTrustMetadata::untrusted("PR13"),
    )
    .expect_err("untrusted source must fail");

    let schema_path = load_in_memory(
        fixture.clone(),
        &contract,
        DynamicUiSourceMetadata::json_debug("dynamic_ui.json", "$.wrong"),
        DynamicUiTrustMetadata::generated_fixture("PR13"),
    )
    .expect_err("schema path must fail");

    let owner = load_in_memory(
        fixture.clone(),
        &contract,
        DynamicUiSourceMetadata::generated_fixture(
            "contract.dynamic_ui.v1",
            VALIDATED_IR_SCHEMA_PATH,
        ),
        DynamicUiTrustMetadata::generated_fixture("PR99"),
    )
    .expect_err("owner mismatch must fail");

    let mut drifted_node = fixture;
    drifted_node.ir_debug.root_node_id = "node.dynamic_ui.changed".to_string();
    let node = load_in_memory(
        drifted_node,
        &contract,
        DynamicUiSourceMetadata::in_memory("node_drift", VALIDATED_IR_SCHEMA_PATH),
        DynamicUiTrustMetadata::first_party("PR13"),
    )
    .expect_err("node drift must fail");

    vec![capability, untrusted, schema_path, owner, node]
}
