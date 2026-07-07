use std::collections::BTreeMap;

use valdi_rust_codec::{binary, json_debug, validator};
use valdi_rust_fixtures::corpus::{CONTRACT_FIXTURE_MANIFEST_JSON, SERIALIZED_FIXTURES};

const CONTRACT_JSON: &str = include_str!("../../../docs/rust_migration/replacement_contract.yaml");

#[test]
fn all_pr04_fixtures_round_trip_through_json_and_postcard() {
    let contract = json_debug::decode_contract(CONTRACT_JSON).expect("contract parses");
    let manifest =
        json_debug::decode_fixture_manifest(CONTRACT_FIXTURE_MANIFEST_JSON).expect("manifest parses");
    validator::validate_manifest_against_contract(&manifest, &contract).expect("manifest matches contract");
    let manifest_by_fixture_id: BTreeMap<&str, &valdi_rust_codec::json_debug::FixtureManifestEntry> =
        manifest.fixtures.iter().map(|entry| (entry.fixture_id.as_str(), entry)).collect();

    assert_eq!(SERIALIZED_FIXTURES.len(), 22);
    for serialized in SERIALIZED_FIXTURES {
        let fixture = json_debug::decode_fixture(serialized.contents).expect(serialized.path);
        validator::validate_fixture_against_contract(&fixture, &contract).expect(serialized.path);
        assert_eq!(fixture.fixture_id, serialized.id.as_str(), "fixture ID drift for {}", serialized.path);
        assert_eq!(
            fixture.metadata.serialized_artifact_path,
            serialized.path,
            "fixture payload path drift for {}",
            serialized.path
        );
        let manifest_entry = manifest_by_fixture_id
            .get(fixture.fixture_id.as_str())
            .expect("fixture manifest entry exists");
        assert_eq!(
            manifest_entry.serialized_artifact_path, serialized.path,
            "fixture manifest path drift for {}",
            serialized.path
        );

        let canonical_json = json_debug::encode_fixture(&fixture).expect(serialized.path);
        assert_eq!(canonical_json, serialized.contents, "canonical JSON drift for {}", serialized.path);

        let json_round_trip = json_debug::decode_fixture(&canonical_json).expect(serialized.path);
        assert_eq!(json_round_trip, fixture, "JSON round-trip drift for {}", serialized.path);

        let binary = binary::encode_fixture(&fixture).expect(serialized.path);
        let binary_round_trip = binary::decode_fixture(&binary).expect(serialized.path);
        assert_eq!(binary_round_trip, fixture, "postcard round-trip drift for {}", serialized.path);
    }
}
