use std::collections::BTreeMap;

use valdi_rust_codec::{json_debug, validator};
use valdi_rust_dsl::{
    fixtures::{
        canonical_snapshot, contract_fixture, validate_canonical_snapshot, DslFixtureInput,
        REQUIRED_CONTRACT_ROW_IDS,
    },
    DSL_CANONICAL_IR_DRIFT,
};
use valdi_rust_fixtures::corpus::{CONTRACT_FIXTURE_MANIFEST_JSON, SERIALIZED_FIXTURES};

const CONTRACT_JSON: &str = include_str!("../../../docs/rust_migration/replacement_contract.yaml");

#[test]
fn dsl_output_matches_canonical_ir_for_every_contract_row() {
    let contract = json_debug::decode_contract(CONTRACT_JSON).expect("contract parses");
    let manifest = json_debug::decode_fixture_manifest(CONTRACT_FIXTURE_MANIFEST_JSON)
        .expect("manifest parses");
    validator::validate_manifest_against_contract(&manifest, &contract)
        .expect("manifest matches contract");
    assert_eq!(contract.rows.len(), REQUIRED_CONTRACT_ROW_IDS.len());
    assert_eq!(manifest.fixtures.len(), REQUIRED_CONTRACT_ROW_IDS.len());
    let manifest_row_ids = manifest
        .fixtures
        .iter()
        .map(|fixture| fixture.contract_row_id.as_str())
        .collect::<Vec<_>>();
    assert_eq!(manifest_row_ids.as_slice(), REQUIRED_CONTRACT_ROW_IDS);

    let rows = contract_rows_by_id(&contract.rows);
    let serialized_by_fixture_id = SERIALIZED_FIXTURES
        .iter()
        .map(|fixture| (fixture.id.as_str(), fixture))
        .collect::<BTreeMap<_, _>>();

    for expected_id in REQUIRED_CONTRACT_ROW_IDS {
        assert!(
            rows.contains_key(expected_id),
            "missing contract row {expected_id}"
        );
    }

    for fixture_entry in &manifest.fixtures {
        let row = rows
            .get(fixture_entry.contract_row_id.as_str())
            .expect("manifest row exists in contract");
        let input = input_from_fixture(fixture_entry, row);
        let dsl_document = contract_fixture(input).expect("DSL fixture builds");
        let serialized = serialized_by_fixture_id
            .get(dsl_document.fixture_id.as_str())
            .expect("serialized fixture exists");
        let canonical_fixture =
            json_debug::decode_fixture(serialized.contents).expect(serialized.path);
        validator::validate_fixture_against_contract(&canonical_fixture, &contract)
            .expect("serialized fixture matches contract");

        assert_eq!(dsl_document.fixture_id, canonical_fixture.fixture_id);
        assert_eq!(
            dsl_document.contract_row_id,
            canonical_fixture.contract_row_id
        );
        assert_eq!(dsl_document.surface, canonical_fixture.surface);
        assert_eq!(dsl_document.fixture_tags, canonical_fixture.fixture_tags);
        assert_eq!(
            dsl_document.platform_targets,
            canonical_fixture.platform_targets
        );
        assert_eq!(dsl_document.owner_prs, canonical_fixture.metadata.owner_prs);
        assert_eq!(dsl_document.proof_prs, canonical_fixture.metadata.proof_prs);
        assert_eq!(
            dsl_document.serialized_artifact_path,
            canonical_fixture.metadata.serialized_artifact_path
        );

        let debug = dsl_document.canonical_debug();
        assert_eq!(
            debug.contract_surface,
            canonical_fixture.ir_debug.contract_surface
        );
        assert_eq!(debug.root_node_id, canonical_fixture.ir_debug.root_node_id);
        assert_eq!(debug.component_id, canonical_fixture.ir_debug.component_id);
        assert_eq!(debug.source_span, canonical_fixture.ir_debug.source_span);
        assert_eq!(
            debug.coverage_tokens,
            canonical_fixture.ir_debug.coverage_tokens
        );
        assert_eq!(
            dsl_document.payload.kind_token(),
            fixture_entry.contract_row_id
        );
    }

    let inputs = manifest
        .fixtures
        .iter()
        .map(|fixture| input_from_fixture(fixture, rows[fixture.contract_row_id.as_str()]))
        .collect::<Vec<_>>();
    let snapshot = canonical_snapshot(inputs).expect("snapshot renders");
    assert_eq!(snapshot, include_str!("../snapshots/dsl_canonical_ir.snap"));
}

#[test]
fn canonical_snapshot_drift_is_exactly_diagnosed() {
    let contract = json_debug::decode_contract(CONTRACT_JSON).expect("contract parses");
    let manifest = json_debug::decode_fixture_manifest(CONTRACT_FIXTURE_MANIFEST_JSON)
        .expect("manifest parses");
    let rows = contract_rows_by_id(&contract.rows);
    let inputs = manifest
        .fixtures
        .iter()
        .map(|fixture| input_from_fixture(fixture, rows[fixture.contract_row_id.as_str()]))
        .collect::<Vec<_>>();

    let expected = include_str!("../snapshots/dsl_canonical_ir.snap");
    let drifted = expected
        .lines()
        .filter(|line| !line.contains("bindings_expressions|"))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    let diagnostic =
        validate_canonical_snapshot(inputs, &drifted).expect_err("removed row must fail");

    assert_eq!(diagnostic.code, DSL_CANONICAL_IR_DRIFT);
    assert_eq!(diagnostic.path, "$.dsl_canonical_ir");
    assert_eq!(diagnostic.severity.as_str(), "error");
}

fn contract_rows_by_id<'a>(
    rows: &'a [json_debug::ContractRow],
) -> BTreeMap<&'a str, &'a json_debug::ContractRow> {
    rows.iter().map(|row| (row.id.as_str(), row)).collect()
}

fn input_from_fixture(
    fixture: &json_debug::FixtureManifestEntry,
    row: &json_debug::ContractRow,
) -> DslFixtureInput {
    DslFixtureInput {
        fixture_id: fixture.fixture_id.clone(),
        contract_row_id: fixture.contract_row_id.clone(),
        surface: fixture.surface.clone(),
        fixture_tags: fixture.fixture_tags.clone(),
        platform_targets: fixture.platform_targets.clone(),
        owner_prs: fixture.owner_prs.clone(),
        proof_prs: fixture.proof_prs.clone(),
        serialized_artifact_path: fixture.serialized_artifact_path.clone(),
        coverage_tokens: row.coverage.clone(),
    }
}
