use valdi_rust_codec::binary::{self, BinaryInvalidManifestEnvelope};
use valdi_rust_codec::{
    json_debug, InvalidFixtureManifest, CURRENT_BINARY_WIRE_VERSION, CURRENT_SCHEMA_VERSION,
};
use valdi_rust_fixtures::corpus::SERIALIZED_FIXTURES;
use valdi_rust_fixtures::invalid::INVALID_FIXTURE_MANIFEST_JSON;

#[test]
fn current_schema_and_wire_versions_are_accepted() {
    let fixture = first_fixture();
    assert_eq!(fixture.fixture_schema_version, CURRENT_SCHEMA_VERSION);

    let encoded = binary::encode_fixture(&fixture).expect("current binary encode succeeds");
    let decoded = binary::decode_fixture(&encoded).expect("current binary decode succeeds");
    assert_eq!(decoded, fixture);
}

#[test]
fn unsupported_json_schema_version_returns_exact_diagnostic() {
    let mut fixture = first_fixture();
    fixture.fixture_schema_version = CURRENT_SCHEMA_VERSION + 1;

    let diagnostic = json_debug::encode_fixture(&fixture)
        .expect_err("unsupported JSON debug schema version must fail");
    assert_eq!(diagnostic.code, "IR_SCHEMA_VERSION_UNSUPPORTED");
    assert_eq!(diagnostic.path, "$.fixture_schema_version");
}

#[test]
fn unsupported_binary_schema_and_wire_versions_return_exact_diagnostics() {
    let fixture = first_fixture();

    let unsupported_schema =
        binary::encode_fixture_with_versions(&fixture, CURRENT_SCHEMA_VERSION + 1, CURRENT_BINARY_WIRE_VERSION)
            .expect("test can encode unsupported schema envelope");
    let schema_diagnostic = binary::decode_fixture(&unsupported_schema)
        .expect_err("unsupported binary schema version must fail");
    assert_eq!(schema_diagnostic.code, "IR_SCHEMA_VERSION_UNSUPPORTED");
    assert_eq!(schema_diagnostic.path, "$.binary.schema_version");

    let unsupported_wire =
        binary::encode_fixture_with_versions(&fixture, CURRENT_SCHEMA_VERSION, CURRENT_BINARY_WIRE_VERSION + 1)
            .expect("test can encode unsupported wire envelope");
    let wire_diagnostic = binary::decode_fixture(&unsupported_wire)
        .expect_err("unsupported binary wire version must fail");
    assert_eq!(wire_diagnostic.code, "IR_BINARY_WIRE_VERSION_UNSUPPORTED");
    assert_eq!(wire_diagnostic.path, "$.binary.wire_version");
}

#[test]
fn corrupted_binary_payload_returns_exact_diagnostic() {
    let diagnostic = binary::decode_fixture(&[0xff, 0x00, 0x13, 0x37])
        .expect_err("corrupted binary payload must fail");
    assert_eq!(diagnostic.code, "IR_BINARY_PAYLOAD_CORRUPT");
    assert_eq!(diagnostic.path, "$.binary");
}

#[test]
fn binary_invalid_manifest_payload_shape_returns_exact_diagnostics() {
    let mut unsupported_schema = invalid_manifest();
    unsupported_schema.schema_version = CURRENT_SCHEMA_VERSION + 1;
    let unsupported_schema_bytes = encode_invalid_manifest_payload(unsupported_schema);
    let unsupported_schema_diagnostic = binary::decode_invalid_manifest(&unsupported_schema_bytes)
        .expect_err("unsupported invalid manifest payload schema must fail");
    assert_eq!(unsupported_schema_diagnostic.code, "IR_SCHEMA_VERSION_UNSUPPORTED");
    assert_eq!(unsupported_schema_diagnostic.path, "$.schema_version");

    let mut missing_owner = invalid_manifest();
    missing_owner.owner_pr.clear();
    let missing_owner_encode_diagnostic = binary::encode_invalid_manifest(&missing_owner)
        .expect_err("invalid manifest binary encode must validate required fields");
    assert_eq!(missing_owner_encode_diagnostic.code, "IR_FIELD_REQUIRED");
    assert_eq!(missing_owner_encode_diagnostic.path, "$.owner_pr");

    let missing_owner_bytes = encode_invalid_manifest_payload(missing_owner);
    let missing_owner_decode_diagnostic = binary::decode_invalid_manifest(&missing_owner_bytes)
        .expect_err("invalid manifest binary decode must validate required fields");
    assert_eq!(missing_owner_decode_diagnostic.code, "IR_FIELD_REQUIRED");
    assert_eq!(missing_owner_decode_diagnostic.path, "$.owner_pr");
}

fn first_fixture() -> valdi_rust_codec::FixtureEnvelope {
    json_debug::decode_fixture(SERIALIZED_FIXTURES[0].contents).expect("fixture parses")
}

fn invalid_manifest() -> InvalidFixtureManifest {
    json_debug::decode_invalid_manifest(INVALID_FIXTURE_MANIFEST_JSON).expect("invalid manifest parses")
}

fn encode_invalid_manifest_payload(payload: InvalidFixtureManifest) -> Vec<u8> {
    let envelope = BinaryInvalidManifestEnvelope {
        schema_version: CURRENT_SCHEMA_VERSION,
        wire_version: CURRENT_BINARY_WIRE_VERSION,
        payload,
    };
    postcard::to_allocvec(&envelope).expect("test invalid manifest envelope encodes")
}
