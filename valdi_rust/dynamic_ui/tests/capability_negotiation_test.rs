use valdi_rust_dynamic_ui::{
    capabilities::{
        negotiate_capabilities, DynamicUiProducerCapability, DynamicUiProducerCapabilitySet,
    },
    validate_capability_set, DynamicUiSourceMetadata, DynamicUiTrustMetadata,
    DYNAMIC_UI_CAPABILITY_UNSUPPORTED, VALIDATED_IR_SCHEMA_PATH,
};

const IN_MEMORY_ONLY: &[DynamicUiProducerCapability] = &[
    DynamicUiProducerCapability::InMemoryInput,
    DynamicUiProducerCapability::RuntimeValidation,
    DynamicUiProducerCapability::RuntimeBridge,
];

#[test]
fn producer_capability_negotiation_is_typed_and_source_specific() {
    let source = DynamicUiSourceMetadata::in_memory("memory", VALIDATED_IR_SCHEMA_PATH);
    let decisions = negotiate_capabilities(DynamicUiProducerCapabilitySet::all(), source);

    assert_eq!(decisions.len(), 4);
    assert!(decisions.iter().all(|decision| decision.supported));
    assert_eq!(
        decisions[0].capability,
        DynamicUiProducerCapability::InMemoryInput
    );
}

#[test]
fn missing_required_capability_has_exact_diagnostic() {
    let diagnostic = validate_capability_set(
        DynamicUiProducerCapabilitySet::new(IN_MEMORY_ONLY),
        DynamicUiSourceMetadata::in_memory("memory", VALIDATED_IR_SCHEMA_PATH),
        DynamicUiTrustMetadata::first_party("PR13"),
    )
    .expect_err("missing mock backend sink must fail");

    assert_eq!(diagnostic.code, DYNAMIC_UI_CAPABILITY_UNSUPPORTED);
    assert_eq!(diagnostic.path, "$.producer.capabilities[]");
    assert_eq!(diagnostic.severity.as_str(), "error");
    assert_eq!(diagnostic.source.source_id(), "memory");
    assert_eq!(diagnostic.trust.owner_pr, "PR13");
    assert_eq!(
        diagnostic.message,
        "dynamic producer lacks mock_backend_sink capability"
    );
}
