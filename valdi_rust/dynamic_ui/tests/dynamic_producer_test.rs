use valdi_rust_backend::operations::BackendOperationFamily;
use valdi_rust_dynamic_ui::{
    apply_to_mock_backend,
    capabilities::DynamicUiProducerCapabilitySet,
    fixtures::{contract_document, dynamic_ui_fixture},
    load_from_producer, DynamicProducerDescriptor, DynamicUiSourceMetadata, DynamicUiTrustMetadata,
    InMemoryDynamicProducer, VALIDATED_IR_SCHEMA_PATH,
};

const CONTRACT_JSON: &str = include_str!("../../../docs/rust_migration/replacement_contract.yaml");

#[test]
fn in_memory_dynamic_producer_feeds_validator_runtime_and_mock_backend() {
    let contract = contract_document(CONTRACT_JSON).expect("contract parses");
    let fixture = dynamic_ui_fixture().expect("fixture parses");
    let descriptor = DynamicProducerDescriptor {
        name: "dynamic_ui.memory_producer",
        owner_pr: "PR13",
        source: DynamicUiSourceMetadata::in_memory(
            "dynamic_ui.memory_producer",
            VALIDATED_IR_SCHEMA_PATH,
        ),
        trust: DynamicUiTrustMetadata::first_party("PR13"),
        capabilities: DynamicUiProducerCapabilitySet::all(),
    };
    let producer = InMemoryDynamicProducer::new(descriptor, fixture);

    let document = load_from_producer(&producer, &contract).expect("producer loads");
    assert_eq!(document.fixture.fixture_id, "contract.dynamic_ui.v1");
    assert_eq!(
        document.runtime_document.root.node_id.as_str(),
        "node.dynamic_ui.root"
    );

    let receipt = apply_to_mock_backend(&document.runtime_document).expect("mock backend accepts");
    let families = receipt
        .operations
        .iter()
        .map(|operation| operation.family())
        .collect::<Vec<_>>();
    assert_eq!(
        families,
        vec![
            BackendOperationFamily::TransactionGroup,
            BackendOperationFamily::Create,
            BackendOperationFamily::Root,
            BackendOperationFamily::Destroy,
            BackendOperationFamily::TransactionGroup,
        ]
    );
    assert_eq!(
        receipt.mock_backend_operation_count,
        receipt.operations.len()
    );
}
