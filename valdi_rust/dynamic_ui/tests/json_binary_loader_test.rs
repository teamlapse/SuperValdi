use valdi_rust_codec::{binary, json_debug};
use valdi_rust_dynamic_ui::{
    fixtures::contract_document, load_binary_bytes, load_generated_fixture, load_json_debug,
    DynamicUiSourceMetadata, DynamicUiTrustMetadata, VALIDATED_IR_SCHEMA_PATH,
};
use valdi_rust_fixtures::corpus::SERIALIZED_FIXTURES;

const CONTRACT_JSON: &str = include_str!("../../../docs/rust_migration/replacement_contract.yaml");

#[test]
fn all_contract_fixtures_load_from_json_binary_and_generated_fixture_sources() {
    let contract = contract_document(CONTRACT_JSON).expect("contract parses");
    assert_eq!(SERIALIZED_FIXTURES.len(), 22);

    for serialized in SERIALIZED_FIXTURES {
        let json_document = load_json_debug(
            serialized.contents,
            &contract,
            DynamicUiSourceMetadata::json_debug(serialized.id.as_str(), VALIDATED_IR_SCHEMA_PATH),
            DynamicUiTrustMetadata::first_party("PR01"),
        )
        .expect(serialized.path);
        assert_eq!(json_document.fixture.fixture_id, serialized.id.as_str());
        assert_eq!(
            json_document.runtime_document.root.node_id.as_str(),
            json_document.fixture.ir_debug.root_node_id
        );

        let fixture = json_debug::decode_fixture(serialized.contents).expect(serialized.path);
        let binary_bytes = binary::encode_fixture(&fixture).expect(serialized.path);
        let binary_document = load_binary_bytes(
            &binary_bytes,
            &contract,
            DynamicUiSourceMetadata::binary_bytes(serialized.id.as_str(), VALIDATED_IR_SCHEMA_PATH),
            DynamicUiTrustMetadata::first_party("PR01"),
        )
        .expect(serialized.path);
        assert_eq!(binary_document.fixture, json_document.fixture);

        let generated_document = load_generated_fixture(
            serialized,
            &contract,
            DynamicUiSourceMetadata::generated_fixture(
                serialized.id.as_str(),
                VALIDATED_IR_SCHEMA_PATH,
            ),
            DynamicUiTrustMetadata::first_party("PR01"),
        )
        .expect(serialized.path);
        assert_eq!(generated_document.fixture, json_document.fixture);
    }
}

#[test]
fn dynamic_fixture_round_trips_through_json_and_postcard_before_runtime_load() {
    let contract = contract_document(CONTRACT_JSON).expect("contract parses");
    let serialized = SERIALIZED_FIXTURES
        .iter()
        .find(|fixture| fixture.id.as_str() == "contract.dynamic_ui.v1")
        .expect("dynamic fixture exists");
    let fixture = json_debug::decode_fixture(serialized.contents).expect(serialized.path);
    let canonical_json = json_debug::encode_fixture(&fixture).expect("json encodes");
    let json_document = load_json_debug(
        &canonical_json,
        &contract,
        DynamicUiSourceMetadata::json_debug("dynamic_ui.json", VALIDATED_IR_SCHEMA_PATH),
        DynamicUiTrustMetadata::generated_fixture("PR13"),
    )
    .expect("json loads");

    let binary_bytes = binary::encode_fixture(&json_document.fixture).expect("binary encodes");
    let binary_document = load_binary_bytes(
        &binary_bytes,
        &contract,
        DynamicUiSourceMetadata::binary_bytes("dynamic_ui.postcard", VALIDATED_IR_SCHEMA_PATH),
        DynamicUiTrustMetadata::generated_fixture("PR13"),
    )
    .expect("binary loads");

    assert_eq!(binary_document.fixture, json_document.fixture);
    assert_eq!(
        binary_document.runtime_document.root.node_id,
        json_document.runtime_document.root.node_id
    );
}
