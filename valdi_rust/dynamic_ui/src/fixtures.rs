use valdi_rust_codec::{json_debug, ContractDocument, FixtureEnvelope};
use valdi_rust_fixtures::corpus::{CONTRACT_FIXTURE_MANIFEST_JSON, SERIALIZED_FIXTURES};

use crate::{
    diagnostics::{DynamicUiDiagnostic, DynamicUiResult, DYNAMIC_UI_INVALID_DIAGNOSTIC_DRIFT},
    loader::{load_generated_fixture, DynamicUiDocument},
    runtime_bridge::{apply_to_mock_backend, validate_dynamic_ui_trace_snapshot},
    source::{DynamicUiSourceMetadata, VALIDATED_IR_SCHEMA_PATH},
    trust::DynamicUiTrustMetadata,
};

pub const DYNAMIC_UI_FIXTURE_ID: &str = "contract.dynamic_ui.v1";

pub fn contract_document(contract_json: &str) -> DynamicUiResult<ContractDocument> {
    json_debug::decode_contract(contract_json).map_err(|diagnostic| {
        DynamicUiDiagnostic::from_codec(
            diagnostic,
            DynamicUiSourceMetadata::generated_fixture("contract", VALIDATED_IR_SCHEMA_PATH),
            DynamicUiTrustMetadata::first_party("PR13"),
            None,
            Some("PR13".into()),
        )
    })
}

pub fn fixture_manifest() -> DynamicUiResult<valdi_rust_codec::FixtureManifest> {
    json_debug::decode_fixture_manifest(CONTRACT_FIXTURE_MANIFEST_JSON).map_err(|diagnostic| {
        DynamicUiDiagnostic::from_codec(
            diagnostic,
            DynamicUiSourceMetadata::generated_fixture(
                "contract_fixture_manifest",
                VALIDATED_IR_SCHEMA_PATH,
            ),
            DynamicUiTrustMetadata::first_party("PR13"),
            None,
            Some("PR13".into()),
        )
    })
}

pub fn dynamic_ui_fixture() -> DynamicUiResult<FixtureEnvelope> {
    let serialized = SERIALIZED_FIXTURES
        .iter()
        .find(|fixture| fixture.id.as_str() == DYNAMIC_UI_FIXTURE_ID)
        .expect("dynamic UI fixture is part of PR04 corpus");
    json_debug::decode_fixture(serialized.contents).map_err(|diagnostic| {
        DynamicUiDiagnostic::from_codec(
            diagnostic,
            DynamicUiSourceMetadata::generated_fixture(
                DYNAMIC_UI_FIXTURE_ID,
                VALIDATED_IR_SCHEMA_PATH,
            ),
            DynamicUiTrustMetadata::first_party("PR13"),
            None,
            Some("PR13".into()),
        )
    })
}

pub fn load_dynamic_ui_fixture(contract_json: &str) -> DynamicUiResult<DynamicUiDocument> {
    let contract = contract_document(contract_json)?;
    let serialized = SERIALIZED_FIXTURES
        .iter()
        .find(|fixture| fixture.id.as_str() == DYNAMIC_UI_FIXTURE_ID)
        .expect("dynamic UI fixture is part of PR04 corpus");
    load_generated_fixture(
        serialized,
        &contract,
        DynamicUiSourceMetadata::generated_fixture(DYNAMIC_UI_FIXTURE_ID, VALIDATED_IR_SCHEMA_PATH),
        DynamicUiTrustMetadata::generated_fixture("PR13"),
    )
}

pub fn dynamic_ui_trace_snapshot(contract_json: &str) -> DynamicUiResult<String> {
    let document = load_dynamic_ui_fixture(contract_json)?;
    Ok(apply_to_mock_backend(&document.runtime_document)?.snapshot)
}

pub fn validate_trace_snapshot(contract_json: &str, expected: &str) -> DynamicUiResult<()> {
    let document = load_dynamic_ui_fixture(contract_json)?;
    validate_dynamic_ui_trace_snapshot(&document.runtime_document, expected)
}

pub fn invalid_diagnostics_snapshot(diagnostics: &[DynamicUiDiagnostic]) -> String {
    let mut lines = vec!["dynamic_ui_invalid_diagnostics_v1".to_string()];
    for diagnostic in diagnostics {
        lines.push(format!(
            "- code={} path={} severity={} source={} trust={} owner={} node={}",
            diagnostic.code,
            diagnostic.path,
            diagnostic.severity.as_str(),
            diagnostic.source.source_id(),
            diagnostic.trust.level.as_str(),
            diagnostic.owner_pr.as_deref().unwrap_or("<none>"),
            diagnostic.node_id.as_deref().unwrap_or("<none>")
        ));
    }
    format!("{}\n", lines.join("\n"))
}

pub fn validate_invalid_diagnostics_snapshot(
    diagnostics: &[DynamicUiDiagnostic],
    expected: &str,
) -> DynamicUiResult<()> {
    let actual = invalid_diagnostics_snapshot(diagnostics);
    if actual == expected {
        return Ok(());
    }
    Err(DynamicUiDiagnostic::error(
        DYNAMIC_UI_INVALID_DIAGNOSTIC_DRIFT,
        "$.dynamic_ui.invalid_diagnostics",
        DynamicUiSourceMetadata::in_memory("invalid_diagnostics", VALIDATED_IR_SCHEMA_PATH),
        DynamicUiTrustMetadata::first_party("PR13"),
        None::<String>,
        Some("PR13"),
        "dynamic UI invalid diagnostic snapshot drift",
    ))
}
