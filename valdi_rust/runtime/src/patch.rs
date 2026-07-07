use valdi_rust_ir::ids::StateId;

use crate::{
    diagnostics::{RuntimeDiagnostic, RuntimeResult},
    document::RuntimeDocument,
    identity::{IdentityTables, StateSlot},
};

pub const RUNTIME_REBUILD_REQUIRED: &str = "RUNTIME_REBUILD_REQUIRED";
pub const RUNTIME_STATE_SLOT_DRIFT: &str = "RUNTIME_STATE_SLOT_DRIFT";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityPatchResult {
    pub preserved_state_slots: Vec<StateSlot>,
}

pub fn apply_identity_patch(
    before: &RuntimeDocument,
    after: &RuntimeDocument,
) -> RuntimeResult<IdentityPatchResult> {
    if before.component_id != after.component_id {
        return Err(RuntimeDiagnostic::error(
            RUNTIME_REBUILD_REQUIRED,
            "$.document.identity.component_id",
            format!(
                "component identity changed: {} != {}",
                before.component_id.as_str(),
                after.component_id.as_str()
            ),
            after.source_span_id,
        ));
    }
    if before.root.node_id != after.root.node_id {
        return Err(RuntimeDiagnostic::error(
            RUNTIME_REBUILD_REQUIRED,
            "$.document.identity.root_node_id",
            format!(
                "root identity changed: {} != {}",
                before.root.node_id.as_str(),
                after.root.node_id.as_str()
            ),
            after.source_span_id,
        ));
    }

    let before_tables = IdentityTables::from_document(before);
    let after_tables = IdentityTables::from_document(after);
    let preserved_state_slots = before_tables
        .state_slots
        .iter()
        .copied()
        .filter(|before_slot| {
            after_tables.state_slots.iter().any(|after_slot| {
                after_slot.state_id == before_slot.state_id
                    && after_slot.node_id == before_slot.node_id
                    && after_slot.component_id == before_slot.component_id
            })
        })
        .collect();

    Ok(IdentityPatchResult {
        preserved_state_slots,
    })
}

pub fn validate_preserved_state_slots(
    result: &IdentityPatchResult,
    expected_state_ids: &[StateId],
) -> RuntimeResult<()> {
    for expected in expected_state_ids {
        if result
            .preserved_state_slots
            .iter()
            .any(|slot| slot.state_id == *expected)
        {
            continue;
        }
        return Err(RuntimeDiagnostic::error(
            RUNTIME_STATE_SLOT_DRIFT,
            "$.identity_patch.preserved_state_slots",
            format!("missing preserved state slot {}", expected.as_str()),
            None,
        ));
    }

    if result.preserved_state_slots.len() != expected_state_ids.len() {
        return Err(RuntimeDiagnostic::error(
            RUNTIME_STATE_SLOT_DRIFT,
            "$.identity_patch.preserved_state_slots",
            format!(
                "preserved state slot count drift: {} != {}",
                result.preserved_state_slots.len(),
                expected_state_ids.len()
            ),
            None,
        ));
    }

    Ok(())
}
