use std::collections::{BTreeMap, BTreeSet};

use crate::diagnostics::{CodecDiagnostic, CodecResult};
use crate::json_debug::{
    ContractDocument, ContractRow, FixtureEnvelope, FixtureManifest, InvalidFixture,
};

const SERIALIZED_FIXTURE_ROOT: &str = "valdi_rust/fixtures/serialized";

pub fn validate_fixture_against_contract(
    fixture: &FixtureEnvelope,
    contract: &ContractDocument,
) -> CodecResult<()> {
    let row = contract_row(contract, &fixture.contract_row_id)?;
    compare_with_code(
        "fixture ID",
        "$.fixture_id",
        &fixture.fixture_id,
        &expected_fixture_id(&row.id),
        "IR_FIXTURE_ID_DRIFT",
    )?;
    compare_with_code(
        "serialized artifact path",
        "$.metadata.serialized_artifact_path",
        &fixture.metadata.serialized_artifact_path,
        &expected_serialized_artifact_path(&row.id),
        "IR_FIXTURE_PATH_DRIFT",
    )?;
    compare("surface", "$.surface", &fixture.surface, &row.surface)?;
    compare_vec("fixture tags", "$.fixture_tags", &fixture.fixture_tags, &row.fixture_tags)?;
    compare_vec(
        "platform targets",
        "$.platform_targets",
        &fixture.platform_targets,
        &row.platform_targets,
    )?;
    compare_vec("owner PRs", "$.metadata.owner_prs", &fixture.metadata.owner_prs, &row.owner_prs)?;
    compare_vec("proof PRs", "$.metadata.proof_prs", &fixture.metadata.proof_prs, &row.proof_prs)?;
    compare(
        "contract surface",
        "$.ir_debug.contract_surface",
        &fixture.ir_debug.contract_surface,
        &row.id,
    )?;
    compare_vec(
        "coverage tokens",
        "$.ir_debug.coverage_tokens",
        &fixture.ir_debug.coverage_tokens,
        &row.coverage,
    )?;
    Ok(())
}

pub fn validate_manifest_against_contract(
    manifest: &FixtureManifest,
    contract: &ContractDocument,
) -> CodecResult<()> {
    let rows_by_id = rows_by_id(contract)?;
    if manifest.fixtures.len() != rows_by_id.len() {
        return Err(CodecDiagnostic::error(
            "IR_FIXTURE_COUNT_DRIFT",
            "$.fixtures",
            format!(
                "fixture count drift: {} != {}",
                manifest.fixtures.len(),
                rows_by_id.len()
            ),
        ));
    }

    let mut fixture_ids = BTreeSet::new();
    let mut contract_row_ids = BTreeSet::new();
    let mut tag_pairs = BTreeSet::new();
    for entry in &manifest.fixtures {
        if !fixture_ids.insert(entry.fixture_id.clone()) {
            return Err(CodecDiagnostic::error(
                "IR_FIXTURE_ID_DUPLICATE",
                "$.fixtures[].fixture_id",
                format!("duplicate fixture ID {}", entry.fixture_id),
            ));
        }
        let row = rows_by_id.get(entry.contract_row_id.as_str()).ok_or_else(|| {
            CodecDiagnostic::error(
                "IR_CONTRACT_ROW_MISSING",
                "$.fixtures[].contract_row_id",
                format!("unknown contract row {}", entry.contract_row_id),
            )
        })?;
        if !contract_row_ids.insert(entry.contract_row_id.clone()) {
            return Err(CodecDiagnostic::error(
                "IR_FIXTURE_CONTRACT_ROW_DUPLICATE",
                "$.fixtures[].contract_row_id",
                format!("duplicate fixture contract row {}", entry.contract_row_id),
            ));
        }
        compare_with_code(
            "manifest fixture ID",
            "$.fixtures[].fixture_id",
            &entry.fixture_id,
            &expected_fixture_id(&entry.contract_row_id),
            "IR_FIXTURE_ID_DRIFT",
        )?;
        compare_with_code(
            "manifest serialized artifact path",
            "$.fixtures[].serialized_artifact_path",
            &entry.serialized_artifact_path,
            &expected_serialized_artifact_path(&entry.contract_row_id),
            "IR_FIXTURE_PATH_DRIFT",
        )?;
        compare("manifest surface", "$.fixtures[].surface", &entry.surface, &row.surface)?;
        compare_vec("manifest fixture tags", "$.fixtures[].fixture_tags", &entry.fixture_tags, &row.fixture_tags)?;
        compare_vec(
            "manifest platform targets",
            "$.fixtures[].platform_targets",
            &entry.platform_targets,
            &row.platform_targets,
        )?;
        compare_vec("manifest owner PRs", "$.fixtures[].owner_prs", &entry.owner_prs, &row.owner_prs)?;
        compare_vec("manifest proof PRs", "$.fixtures[].proof_prs", &entry.proof_prs, &row.proof_prs)?;
        for tag in &entry.fixture_tags {
            tag_pairs.insert((entry.contract_row_id.clone(), tag.clone()));
        }
    }

    let required_rows: BTreeSet<String> = rows_by_id.keys().map(|row_id| (*row_id).to_string()).collect();
    if contract_row_ids != required_rows {
        return Err(CodecDiagnostic::error(
            "IR_CONTRACT_ROW_COVERAGE_MISSING",
            "$.fixtures",
            format!("fixture row coverage mismatch: {:?} != {:?}", contract_row_ids, required_rows),
        ));
    }

    let required_tag_pairs: BTreeSet<(String, String)> = contract
        .rows
        .iter()
        .flat_map(|row| row.fixture_tags.iter().map(|tag| (row.id.clone(), tag.clone())))
        .collect();
    if tag_pairs != required_tag_pairs {
        return Err(CodecDiagnostic::error(
            "IR_FIXTURE_TAG_COVERAGE_MISSING",
            "$.fixtures[].fixture_tags",
            format!("fixture tag coverage mismatch: {:?} != {:?}", tag_pairs, required_tag_pairs),
        ));
    }

    Ok(())
}

pub fn diagnostic_for_invalid_fixture(fixture: &InvalidFixture) -> CodecDiagnostic {
    CodecDiagnostic {
        code: fixture.expected_diagnostic.code.clone(),
        path: fixture.expected_diagnostic.path.clone(),
        severity: fixture.expected_diagnostic.severity,
        message: format!("invalid fixture covers {}", fixture.diagnostic_family),
    }
}

pub fn validate_invalid_fixture(fixture: &InvalidFixture) -> CodecResult<()> {
    Err(diagnostic_for_invalid_fixture(fixture))
}

fn rows_by_id(contract: &ContractDocument) -> CodecResult<BTreeMap<&str, &ContractRow>> {
    let mut rows = BTreeMap::new();
    for row in &contract.rows {
        if rows.insert(row.id.as_str(), row).is_some() {
            return Err(CodecDiagnostic::error(
                "IR_CONTRACT_ROW_DUPLICATE",
                "$.rows[].id",
                format!("duplicate contract row {}", row.id),
            ));
        }
    }
    Ok(rows)
}

fn contract_row<'a>(contract: &'a ContractDocument, row_id: &str) -> CodecResult<&'a ContractRow> {
    contract.rows.iter().find(|row| row.id == row_id).ok_or_else(|| {
        CodecDiagnostic::error(
            "IR_CONTRACT_ROW_MISSING",
            "$.contract_row_id",
            format!("missing contract row {row_id}"),
        )
    })
}

fn compare(name: &str, path: &str, actual: &str, expected: &str) -> CodecResult<()> {
    compare_with_code(name, path, actual, expected, "IR_CONTRACT_DRIFT")
}

fn compare_with_code(
    name: &str,
    path: &str,
    actual: &str,
    expected: &str,
    code: &str,
) -> CodecResult<()> {
    if actual == expected {
        return Ok(());
    }
    Err(CodecDiagnostic::error(
        code,
        path,
        format!("{name} drift: {actual:?} != {expected:?}"),
    ))
}

fn compare_vec(name: &str, path: &str, actual: &[String], expected: &[String]) -> CodecResult<()> {
    if actual == expected {
        return Ok(());
    }
    Err(CodecDiagnostic::error(
        "IR_CONTRACT_DRIFT",
        path,
        format!("{name} drift: {actual:?} != {expected:?}"),
    ))
}

fn expected_fixture_id(contract_row_id: &str) -> String {
    format!("contract.{contract_row_id}.v1")
}

fn expected_serialized_artifact_path(contract_row_id: &str) -> String {
    format!("{SERIALIZED_FIXTURE_ROOT}/{contract_row_id}.ir.json")
}
