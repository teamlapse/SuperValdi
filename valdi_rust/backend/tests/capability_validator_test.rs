use valdi_rust_backend::{
    mock::sample_backend_operations, validate_operation_capabilities, BackendCapability,
    BackendCapabilitySet, BackendDiagnosticSeverity, BackendTarget,
};

#[test]
fn validator_rejects_unsupported_capability_with_exact_diagnostic() {
    let capabilities = BackendCapabilitySet::new(
        BackendTarget::RustHost,
        &[
            BackendCapability::ViewTree,
            BackendCapability::AttributesAndStyle,
        ],
    );
    let operation = sample_backend_operations()
        .into_iter()
        .find(|operation| operation.family().as_str() == "text_measure")
        .expect("sample has text measure");

    let diagnostic = validate_operation_capabilities(&operation, capabilities)
        .expect_err("text measurement capability is absent");

    assert_eq!(diagnostic.code, "BACKEND_CAPABILITY_UNSUPPORTED");
    assert_eq!(diagnostic.path, "$.operations[].text_measure");
    assert_eq!(diagnostic.severity, BackendDiagnosticSeverity::Error);
    assert_eq!(
        diagnostic.message,
        "text_measure requires text_measurement on rust_host"
    );
}
