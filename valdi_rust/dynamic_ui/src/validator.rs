use valdi_rust_codec::{json_debug::FixtureEnvelope, validator};

use crate::{
    capabilities::{negotiate_capabilities, DynamicUiProducerCapabilitySet},
    diagnostics::{
        DynamicUiDiagnostic, DynamicUiResult, DYNAMIC_UI_CAPABILITY_UNSUPPORTED,
        DYNAMIC_UI_NODE_ID_DRIFT, DYNAMIC_UI_OWNER_PR_MISMATCH, DYNAMIC_UI_SCHEMA_PATH_INVALID,
        DYNAMIC_UI_SOURCE_UNTRUSTED,
    },
    producer::DynamicProducerDescriptor,
    source::{DynamicUiSourceMetadata, VALIDATED_IR_SCHEMA_PATH},
    trust::DynamicUiTrustMetadata,
};

pub fn validate_producer_capabilities(
    descriptor: DynamicProducerDescriptor,
) -> DynamicUiResult<()> {
    validate_capability_set(descriptor.capabilities, descriptor.source, descriptor.trust)
}

pub fn validate_capability_set(
    capabilities: DynamicUiProducerCapabilitySet,
    source: DynamicUiSourceMetadata,
    trust: DynamicUiTrustMetadata,
) -> DynamicUiResult<()> {
    for decision in negotiate_capabilities(capabilities, source) {
        if !decision.supported {
            return Err(DynamicUiDiagnostic::error(
                DYNAMIC_UI_CAPABILITY_UNSUPPORTED,
                "$.producer.capabilities[]",
                source,
                trust,
                None::<String>,
                Some(trust.owner_pr),
                format!(
                    "dynamic producer lacks {} capability",
                    decision.capability.as_str()
                ),
            ));
        }
    }
    Ok(())
}

pub fn validate_dynamic_fixture(
    fixture: &FixtureEnvelope,
    contract: &valdi_rust_codec::ContractDocument,
    source: DynamicUiSourceMetadata,
    trust: DynamicUiTrustMetadata,
) -> DynamicUiResult<()> {
    if source.schema_path != VALIDATED_IR_SCHEMA_PATH {
        return Err(DynamicUiDiagnostic::error(
            DYNAMIC_UI_SCHEMA_PATH_INVALID,
            "$.source.schema_path",
            source,
            trust,
            Some(fixture.ir_debug.root_node_id.clone()),
            Some(fixture.metadata.owner_prs.join(",")),
            format!(
                "dynamic source schema path {:?} is not {}",
                source.schema_path, VALIDATED_IR_SCHEMA_PATH
            ),
        ));
    }

    if !trust.level.is_trusted() {
        return Err(DynamicUiDiagnostic::error(
            DYNAMIC_UI_SOURCE_UNTRUSTED,
            "$.trust.level",
            source,
            trust,
            Some(fixture.ir_debug.root_node_id.clone()),
            Some(fixture.metadata.owner_prs.join(",")),
            format!("dynamic source {} is untrusted", source.source_id()),
        ));
    }

    validator::validate_fixture_against_contract(fixture, contract).map_err(|diagnostic| {
        DynamicUiDiagnostic::from_codec(
            diagnostic,
            source,
            trust,
            Some(fixture.ir_debug.root_node_id.clone()),
            Some(fixture.metadata.owner_prs.join(",")),
        )
    })?;

    if !fixture
        .metadata
        .owner_prs
        .iter()
        .any(|owner| owner == trust.owner_pr)
        && !fixture
            .metadata
            .proof_prs
            .iter()
            .any(|owner| owner == trust.owner_pr)
    {
        return Err(DynamicUiDiagnostic::error(
            DYNAMIC_UI_OWNER_PR_MISMATCH,
            "$.metadata.owner_prs",
            source,
            trust,
            Some(fixture.ir_debug.root_node_id.clone()),
            Some(fixture.metadata.owner_prs.join(",")),
            format!(
                "trust owner {} is not declared for fixture {}",
                trust.owner_pr, fixture.fixture_id
            ),
        ));
    }

    let expected_root = format!("node.{}.root", fixture.contract_row_id);
    if fixture.ir_debug.root_node_id != expected_root {
        return Err(DynamicUiDiagnostic::error(
            DYNAMIC_UI_NODE_ID_DRIFT,
            "$.ir_debug.root_node_id",
            source,
            trust,
            Some(fixture.ir_debug.root_node_id.clone()),
            Some(fixture.metadata.owner_prs.join(",")),
            format!(
                "dynamic root node drift: {:?} != {:?}",
                fixture.ir_debug.root_node_id, expected_root
            ),
        ));
    }

    Ok(())
}
