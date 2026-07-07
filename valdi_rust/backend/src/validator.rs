use crate::{
    capabilities::{BackendCapabilitySet, FixtureTagBackendDecision},
    diagnostics::{BackendDiagnostic, BackendResult},
    operations::{BackendOperation, REQUIRED_OPERATION_FAMILIES},
};

pub const BACKEND_CAPABILITY_UNSUPPORTED: &str = "BACKEND_CAPABILITY_UNSUPPORTED";
pub const BACKEND_OPERATION_FAMILY_MISSING: &str = "BACKEND_OPERATION_FAMILY_MISSING";
pub const BACKEND_FIXTURE_TAG_MAPPING_MISSING: &str = "BACKEND_FIXTURE_TAG_MAPPING_MISSING";
pub const BACKEND_FIXTURE_TAG_MAPPING_EXTRA: &str = "BACKEND_FIXTURE_TAG_MAPPING_EXTRA";
pub const BACKEND_FIXTURE_TAG_MAPPING_EMPTY: &str = "BACKEND_FIXTURE_TAG_MAPPING_EMPTY";
pub const BACKEND_FIXTURE_TAG_MAPPING_DUPLICATE: &str = "BACKEND_FIXTURE_TAG_MAPPING_DUPLICATE";

pub fn validate_operation_capabilities(
    operation: &BackendOperation,
    capabilities: BackendCapabilitySet,
) -> BackendResult<()> {
    let family = operation.family();
    let required = family.required_capability();
    if capabilities.supports(required) {
        return Ok(());
    }

    Err(BackendDiagnostic::error(
        BACKEND_CAPABILITY_UNSUPPORTED,
        family.diagnostic_path(),
        format!(
            "{} requires {} on {}",
            family.as_str(),
            required.as_str(),
            capabilities.target.as_str()
        ),
    ))
}

pub fn validate_required_operation_families(operations: &[BackendOperation]) -> BackendResult<()> {
    for required in REQUIRED_OPERATION_FAMILIES {
        if operations
            .iter()
            .any(|operation| operation.family() == *required)
        {
            continue;
        }
        return Err(BackendDiagnostic::error(
            BACKEND_OPERATION_FAMILY_MISSING,
            required.diagnostic_path(),
            format!("missing backend operation family {}", required.as_str()),
        ));
    }
    Ok(())
}

pub fn validate_fixture_tag_decisions(
    required_tags: &[&str],
    decisions: &[FixtureTagBackendDecision],
) -> BackendResult<()> {
    for decision in decisions {
        if decision.operation_families.is_empty() {
            return Err(BackendDiagnostic::error(
                BACKEND_FIXTURE_TAG_MAPPING_EMPTY,
                "$.fixture_tag_backend_decisions[].operation_families",
                format!(
                    "fixture tag {} has no backend operation family",
                    decision.fixture_tag
                ),
            ));
        }
    }

    for (index, decision) in decisions.iter().enumerate() {
        if decisions[index + 1..]
            .iter()
            .any(|other| other.fixture_tag == decision.fixture_tag)
        {
            return Err(BackendDiagnostic::error(
                BACKEND_FIXTURE_TAG_MAPPING_DUPLICATE,
                "$.fixture_tag_backend_decisions[].fixture_tag",
                format!(
                    "duplicate backend fixture tag decision {}",
                    decision.fixture_tag
                ),
            ));
        }
    }

    for required in required_tags {
        if decisions
            .iter()
            .any(|decision| decision.fixture_tag == *required)
        {
            continue;
        }
        return Err(BackendDiagnostic::error(
            BACKEND_FIXTURE_TAG_MAPPING_MISSING,
            "$.fixture_tag_backend_decisions[].fixture_tag",
            format!("missing backend fixture tag decision {required}"),
        ));
    }

    for decision in decisions {
        if required_tags.contains(&decision.fixture_tag) {
            continue;
        }
        return Err(BackendDiagnostic::error(
            BACKEND_FIXTURE_TAG_MAPPING_EXTRA,
            "$.fixture_tag_backend_decisions[].fixture_tag",
            format!(
                "extra backend fixture tag decision {}",
                decision.fixture_tag
            ),
        ));
    }

    Ok(())
}
