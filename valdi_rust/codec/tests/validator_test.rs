use valdi_rust_codec::{binary, json_debug, validator, DiagnosticSeverity};
use valdi_rust_fixtures::corpus::{CONTRACT_FIXTURE_MANIFEST_JSON, SERIALIZED_FIXTURES};
use valdi_rust_fixtures::invalid::INVALID_FIXTURE_MANIFEST_JSON;

const CONTRACT_JSON: &str = include_str!("../../../docs/rust_migration/replacement_contract.yaml");

#[test]
fn invalid_fixtures_produce_exact_diagnostics() {
    let invalid_manifest =
        json_debug::decode_invalid_manifest(INVALID_FIXTURE_MANIFEST_JSON).expect("invalid manifest parses");
    assert_eq!(invalid_manifest.invalid_fixtures.len(), 8);

    let canonical_json =
        json_debug::encode_invalid_manifest(&invalid_manifest).expect("invalid manifest JSON encodes");
    let json_round_trip =
        json_debug::decode_invalid_manifest(&canonical_json).expect("invalid manifest JSON decodes");
    assert_eq!(json_round_trip, invalid_manifest);

    let binary = binary::encode_invalid_manifest(&invalid_manifest).expect("invalid manifest binary encodes");
    let binary_round_trip =
        binary::decode_invalid_manifest(&binary).expect("invalid manifest binary decodes");
    assert_eq!(binary_round_trip, invalid_manifest);

    for invalid_fixture in &invalid_manifest.invalid_fixtures {
        let diagnostic = validator::validate_invalid_fixture(invalid_fixture)
            .expect_err("invalid fixture must return its expected diagnostic");
        assert_eq!(diagnostic.code, invalid_fixture.expected_diagnostic.code);
        assert_eq!(diagnostic.path, invalid_fixture.expected_diagnostic.path);
        assert_eq!(diagnostic.severity, invalid_fixture.expected_diagnostic.severity);
        assert_eq!(diagnostic.severity, DiagnosticSeverity::Error);
    }
}

#[test]
fn validator_is_bound_to_contract_rows_and_fixture_tags() {
    let contract = json_debug::decode_contract(CONTRACT_JSON).expect("contract parses");
    let manifest =
        json_debug::decode_fixture_manifest(CONTRACT_FIXTURE_MANIFEST_JSON).expect("manifest parses");
    validator::validate_manifest_against_contract(&manifest, &contract).expect("manifest matches contract");

    for serialized in SERIALIZED_FIXTURES {
        let fixture = json_debug::decode_fixture(serialized.contents).expect(serialized.path);
        validator::validate_fixture_against_contract(&fixture, &contract).expect(serialized.path);
    }
}

#[test]
fn contract_drift_missing_row_or_tag_fails() {
    let mut contract = json_debug::decode_contract(CONTRACT_JSON).expect("contract parses");
    let manifest =
        json_debug::decode_fixture_manifest(CONTRACT_FIXTURE_MANIFEST_JSON).expect("manifest parses");

    let mut missing_row_manifest = manifest.clone();
    missing_row_manifest.fixtures[0].contract_row_id = "missing_contract_row".to_string();
    let missing_row = validator::validate_manifest_against_contract(&missing_row_manifest, &contract)
        .expect_err("missing contract row must fail");
    assert_eq!(missing_row.code, "IR_CONTRACT_ROW_MISSING");
    assert_eq!(missing_row.path, "$.fixtures[].contract_row_id");

    contract.rows[0].fixture_tags.remove(0);
    let missing_tag = validator::validate_manifest_against_contract(&manifest, &contract)
        .expect_err("missing row/tag coverage must fail");
    assert_eq!(missing_tag.code, "IR_CONTRACT_DRIFT");
    assert_eq!(missing_tag.path, "$.fixtures[].fixture_tags");
}

#[test]
fn manifest_rejects_exact_corpus_drift() {
    let contract = json_debug::decode_contract(CONTRACT_JSON).expect("contract parses");
    let manifest =
        json_debug::decode_fixture_manifest(CONTRACT_FIXTURE_MANIFEST_JSON).expect("manifest parses");

    let mut extra_entry_manifest = manifest.clone();
    let mut extra_entry = extra_entry_manifest.fixtures[0].clone();
    extra_entry.fixture_id = "contract.extra_fixture.v1".to_string();
    extra_entry_manifest.fixtures.push(extra_entry);
    let extra_entry_diagnostic =
        validator::validate_manifest_against_contract(&extra_entry_manifest, &contract)
            .expect_err("extra fixture entry must fail");
    assert_eq!(extra_entry_diagnostic.code, "IR_FIXTURE_COUNT_DRIFT");
    assert_eq!(extra_entry_diagnostic.path, "$.fixtures");

    let mut duplicate_row_manifest = manifest.clone();
    duplicate_row_manifest.fixtures[1].contract_row_id =
        duplicate_row_manifest.fixtures[0].contract_row_id.clone();
    let duplicate_row_diagnostic =
        validator::validate_manifest_against_contract(&duplicate_row_manifest, &contract)
            .expect_err("duplicate contract row entry must fail");
    assert_eq!(duplicate_row_diagnostic.code, "IR_FIXTURE_CONTRACT_ROW_DUPLICATE");
    assert_eq!(duplicate_row_diagnostic.path, "$.fixtures[].contract_row_id");

    let mut fixture_id_drift_manifest = manifest.clone();
    fixture_id_drift_manifest.fixtures[0].fixture_id = "contract.schema_versioning.v2".to_string();
    let fixture_id_drift =
        validator::validate_manifest_against_contract(&fixture_id_drift_manifest, &contract)
            .expect_err("fixture ID drift must fail");
    assert_eq!(fixture_id_drift.code, "IR_FIXTURE_ID_DRIFT");
    assert_eq!(fixture_id_drift.path, "$.fixtures[].fixture_id");

    let mut path_drift_manifest = manifest.clone();
    path_drift_manifest.fixtures[0].serialized_artifact_path =
        "valdi_rust/fixtures/serialized/schema_versioning_drift.ir.json".to_string();
    let path_drift = validator::validate_manifest_against_contract(&path_drift_manifest, &contract)
        .expect_err("serialized path drift must fail");
    assert_eq!(path_drift.code, "IR_FIXTURE_PATH_DRIFT");
    assert_eq!(path_drift.path, "$.fixtures[].serialized_artifact_path");
}

#[test]
fn fixture_payload_rejects_identity_and_path_drift() {
    let contract = json_debug::decode_contract(CONTRACT_JSON).expect("contract parses");
    let fixture = json_debug::decode_fixture(SERIALIZED_FIXTURES[0].contents).expect("fixture parses");

    let mut fixture_id_drift = fixture.clone();
    fixture_id_drift.fixture_id = "contract.schema_versioning.v2".to_string();
    let fixture_id_diagnostic = validator::validate_fixture_against_contract(&fixture_id_drift, &contract)
        .expect_err("fixture ID drift must fail");
    assert_eq!(fixture_id_diagnostic.code, "IR_FIXTURE_ID_DRIFT");
    assert_eq!(fixture_id_diagnostic.path, "$.fixture_id");

    let mut path_drift = fixture;
    path_drift.metadata.serialized_artifact_path =
        "valdi_rust/fixtures/serialized/schema_versioning_drift.ir.json".to_string();
    let path_diagnostic = validator::validate_fixture_against_contract(&path_drift, &contract)
        .expect_err("serialized path drift must fail");
    assert_eq!(path_diagnostic.code, "IR_FIXTURE_PATH_DRIFT");
    assert_eq!(path_diagnostic.path, "$.metadata.serialized_artifact_path");
}
