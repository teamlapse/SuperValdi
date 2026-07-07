use std::collections::BTreeSet;

use valdi_rust_backend::{
    sample_backend_operations, validate_fixture_tag_decisions,
    validate_required_operation_families, FIXTURE_TAG_BACKEND_DECISIONS,
};
use valdi_rust_codec::json_debug;
use valdi_rust_fixtures::corpus::CONTRACT_FIXTURE_MANIFEST_JSON;

#[test]
fn backend_operation_families_cover_the_required_contract() {
    let operations = sample_backend_operations();
    validate_required_operation_families(&operations)
        .expect("sample operations cover all families");
}

#[test]
fn fixture_tag_backend_decisions_match_the_pr04_manifest() {
    let manifest = json_debug::decode_fixture_manifest(CONTRACT_FIXTURE_MANIFEST_JSON)
        .expect("manifest parses");
    let tag_set: BTreeSet<String> = manifest
        .fixtures
        .iter()
        .flat_map(|fixture| fixture.fixture_tags.iter().cloned())
        .collect();
    let required_tags = tag_set.iter().map(String::as_str).collect::<Vec<_>>();

    validate_fixture_tag_decisions(&required_tags, FIXTURE_TAG_BACKEND_DECISIONS)
        .expect("backend fixture tag decisions match manifest tags");
}

#[test]
fn operation_family_and_fixture_tag_drift_fail() {
    let operations = sample_backend_operations();
    let missing_create = validate_required_operation_families(&operations[1..])
        .expect_err("removing an operation family must fail");
    assert_eq!(missing_create.code, "BACKEND_OPERATION_FAMILY_MISSING");
    assert_eq!(missing_create.path, "$.operations[].create");
    assert_eq!(missing_create.severity.as_str(), "error");

    let manifest = json_debug::decode_fixture_manifest(CONTRACT_FIXTURE_MANIFEST_JSON)
        .expect("manifest parses");
    let tag_set: BTreeSet<String> = manifest
        .fixtures
        .iter()
        .flat_map(|fixture| fixture.fixture_tags.iter().cloned())
        .collect();
    let required_tags = tag_set.iter().map(String::as_str).collect::<Vec<_>>();
    let missing_tag =
        validate_fixture_tag_decisions(&required_tags, &FIXTURE_TAG_BACKEND_DECISIONS[1..])
            .expect_err("removing a fixture tag decision must fail");
    assert_eq!(missing_tag.code, "BACKEND_FIXTURE_TAG_MAPPING_MISSING");
    assert_eq!(
        missing_tag.path,
        "$.fixture_tag_backend_decisions[].fixture_tag"
    );
    assert_eq!(missing_tag.severity.as_str(), "error");
}
