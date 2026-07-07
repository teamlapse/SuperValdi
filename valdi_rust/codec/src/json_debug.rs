use serde::{Deserialize, Serialize};

use crate::diagnostics::{CodecDiagnostic, CodecResult, DiagnosticSeverity};
use crate::CURRENT_SCHEMA_VERSION;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SchemaVersion {
    pub id: String,
    pub major: u16,
    pub minor: u16,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FixtureMetadata {
    pub owner_prs: Vec<String>,
    pub proof_prs: Vec<String>,
    pub serialized_artifact_path: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct IrDebugEnvelope {
    pub schema_version: SchemaVersion,
    pub contract_surface: String,
    pub root_node_id: String,
    pub component_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_span: Option<String>,
    pub coverage_tokens: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FixtureEnvelope {
    pub fixture_schema_version: u16,
    pub fixture_id: String,
    pub contract_row_id: String,
    pub surface: String,
    pub fixture_tags: Vec<String>,
    pub platform_targets: Vec<String>,
    pub metadata: FixtureMetadata,
    pub ir_debug: IrDebugEnvelope,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FixtureManifestEntry {
    pub fixture_id: String,
    pub contract_row_id: String,
    pub surface: String,
    pub fixture_tags: Vec<String>,
    pub platform_targets: Vec<String>,
    pub owner_prs: Vec<String>,
    pub proof_prs: Vec<String>,
    pub serialized_artifact_path: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FixtureManifest {
    pub schema_version: u16,
    pub owner_pr: String,
    pub source_contract: String,
    pub fixture_id_format: String,
    pub serialized_artifact_root: String,
    pub fixtures: Vec<FixtureManifestEntry>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ContractDocument {
    pub schema_version: u16,
    pub fixture_tags: Vec<String>,
    pub platform_targets: Vec<String>,
    pub rows: Vec<ContractRow>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ContractRow {
    pub id: String,
    pub surface: String,
    pub coverage: Vec<String>,
    pub owner_prs: Vec<String>,
    pub proof_prs: Vec<String>,
    pub fixture_tags: Vec<String>,
    pub platform_targets: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct InvalidDiagnostic {
    pub code: String,
    pub path: String,
    pub severity: DiagnosticSeverity,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct InvalidFixture {
    pub fixture_id: String,
    pub diagnostic_family: String,
    pub expected_diagnostic: InvalidDiagnostic,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct InvalidFixtureManifest {
    pub schema_version: u16,
    pub owner_pr: String,
    pub source_ir_module: String,
    pub invalid_fixture_id_format: String,
    pub invalid_fixtures: Vec<InvalidFixture>,
}

pub fn decode_fixture(input: &str) -> CodecResult<FixtureEnvelope> {
    let fixture = serde_json::from_str::<FixtureEnvelope>(input).map_err(json_error)?;
    validate_fixture_shape(&fixture)?;
    Ok(fixture)
}

pub fn encode_fixture(fixture: &FixtureEnvelope) -> CodecResult<String> {
    validate_fixture_shape(fixture)?;
    stable_json(fixture)
}

pub fn decode_contract(input: &str) -> CodecResult<ContractDocument> {
    let contract = serde_json::from_str::<ContractDocument>(input).map_err(json_error)?;
    require_version(contract.schema_version, "$.schema_version")?;
    require_non_empty_rows("contract rows", "$.rows", contract.rows.len())?;
    Ok(contract)
}

pub fn decode_fixture_manifest(input: &str) -> CodecResult<FixtureManifest> {
    let manifest = serde_json::from_str::<FixtureManifest>(input).map_err(json_error)?;
    require_version(manifest.schema_version, "$.schema_version")?;
    require_non_empty("owner_pr", "$.owner_pr", &manifest.owner_pr)?;
    require_non_empty_rows("fixtures", "$.fixtures", manifest.fixtures.len())?;
    Ok(manifest)
}

pub fn decode_invalid_manifest(input: &str) -> CodecResult<InvalidFixtureManifest> {
    let manifest = serde_json::from_str::<InvalidFixtureManifest>(input).map_err(json_error)?;
    validate_invalid_manifest_shape(&manifest)?;
    Ok(manifest)
}

pub fn encode_invalid_manifest(manifest: &InvalidFixtureManifest) -> CodecResult<String> {
    validate_invalid_manifest_shape(manifest)?;
    stable_json(manifest)
}

pub(crate) fn validate_fixture_shape(fixture: &FixtureEnvelope) -> CodecResult<()> {
    require_version(fixture.fixture_schema_version, "$.fixture_schema_version")?;
    require_non_empty("fixture_id", "$.fixture_id", &fixture.fixture_id)?;
    require_non_empty("contract_row_id", "$.contract_row_id", &fixture.contract_row_id)?;
    require_non_empty("surface", "$.surface", &fixture.surface)?;
    require_non_empty_rows("fixture tags", "$.fixture_tags", fixture.fixture_tags.len())?;
    require_non_empty_rows("platform targets", "$.platform_targets", fixture.platform_targets.len())?;
    require_non_empty_rows("owner PRs", "$.metadata.owner_prs", fixture.metadata.owner_prs.len())?;
    require_non_empty_rows("proof PRs", "$.metadata.proof_prs", fixture.metadata.proof_prs.len())?;
    require_non_empty(
        "serialized_artifact_path",
        "$.metadata.serialized_artifact_path",
        &fixture.metadata.serialized_artifact_path,
    )?;
    require_non_empty(
        "contract_surface",
        "$.ir_debug.contract_surface",
        &fixture.ir_debug.contract_surface,
    )?;
    require_non_empty("root_node_id", "$.ir_debug.root_node_id", &fixture.ir_debug.root_node_id)?;
    require_non_empty("component_id", "$.ir_debug.component_id", &fixture.ir_debug.component_id)?;
    require_non_empty_rows("coverage tokens", "$.ir_debug.coverage_tokens", fixture.ir_debug.coverage_tokens.len())?;
    Ok(())
}

pub(crate) fn require_version(version: u16, path: &str) -> CodecResult<()> {
    if version == CURRENT_SCHEMA_VERSION {
        return Ok(());
    }
    Err(CodecDiagnostic::error(
        "IR_SCHEMA_VERSION_UNSUPPORTED",
        path,
        format!("unsupported schema version {version}"),
    ))
}

pub(crate) fn validate_invalid_manifest_shape(manifest: &InvalidFixtureManifest) -> CodecResult<()> {
    require_version(manifest.schema_version, "$.schema_version")?;
    require_non_empty("owner_pr", "$.owner_pr", &manifest.owner_pr)?;
    require_non_empty("source_ir_module", "$.source_ir_module", &manifest.source_ir_module)?;
    require_non_empty(
        "invalid_fixture_id_format",
        "$.invalid_fixture_id_format",
        &manifest.invalid_fixture_id_format,
    )?;
    require_non_empty_rows("invalid fixtures", "$.invalid_fixtures", manifest.invalid_fixtures.len())?;
    for fixture in &manifest.invalid_fixtures {
        require_non_empty("fixture_id", "$.invalid_fixtures[].fixture_id", &fixture.fixture_id)?;
        require_non_empty(
            "diagnostic_family",
            "$.invalid_fixtures[].diagnostic_family",
            &fixture.diagnostic_family,
        )?;
        require_non_empty(
            "expected diagnostic code",
            "$.invalid_fixtures[].expected_diagnostic.code",
            &fixture.expected_diagnostic.code,
        )?;
        require_non_empty(
            "expected diagnostic path",
            "$.invalid_fixtures[].expected_diagnostic.path",
            &fixture.expected_diagnostic.path,
        )?;
    }
    Ok(())
}

fn require_non_empty(name: &str, path: &str, value: &str) -> CodecResult<()> {
    if !value.trim().is_empty() {
        return Ok(());
    }
    Err(CodecDiagnostic::error("IR_FIELD_REQUIRED", path, format!("{name} is required")))
}

fn require_non_empty_rows(name: &str, path: &str, len: usize) -> CodecResult<()> {
    if len > 0 {
        return Ok(());
    }
    Err(CodecDiagnostic::error("IR_FIELD_REQUIRED", path, format!("{name} are required")))
}

fn stable_json<T: Serialize>(value: &T) -> CodecResult<String> {
    serde_json::to_string_pretty(value)
        .map(|json| format!("{json}\n"))
        .map_err(|err| CodecDiagnostic::error("IR_JSON_ENCODE_ERROR", "$", err.to_string()))
}

fn json_error(err: serde_json::Error) -> CodecDiagnostic {
    CodecDiagnostic::error("IR_JSON_PARSE_ERROR", "$", err.to_string())
}
