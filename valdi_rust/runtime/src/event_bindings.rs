use valdi_rust_ir::{
    events::{EventBinding, EventKind},
    ids::{ActionId, NodeId},
};

use crate::{
    diagnostics::{RuntimeDiagnostic, RuntimeResult},
    scheduler::ActionScheduler,
};

pub const RUNTIME_EVENT_BINDING_MISSING: &str = "RUNTIME_EVENT_BINDING_MISSING";
pub const RUNTIME_EVENT_ACTION_MISSING: &str = "RUNTIME_EVENT_ACTION_MISSING";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventActionBindings {
    bindings: Vec<EventBinding>,
}

impl EventActionBindings {
    pub fn new(bindings: Vec<EventBinding>) -> Self {
        Self { bindings }
    }

    pub fn resolve(
        &self,
        node_id: NodeId,
        kind: EventKind,
        scheduler: &ActionScheduler,
    ) -> RuntimeResult<ActionId> {
        let binding = self
            .bindings
            .iter()
            .find(|binding| binding.node_id == node_id && binding.kind == kind)
            .ok_or_else(|| {
                RuntimeDiagnostic::error(
                    RUNTIME_EVENT_BINDING_MISSING,
                    "$.event_bindings",
                    format!("event mapping is missing for node {}", node_id.as_str()),
                    None,
                )
            })?;
        if scheduler.has_action(binding.action_id) {
            return Ok(binding.action_id);
        }
        Err(RuntimeDiagnostic::error(
            RUNTIME_EVENT_ACTION_MISSING,
            "$.event_bindings.action_id",
            format!(
                "event action {} is missing from scheduler",
                binding.action_id.as_str()
            ),
            None,
        ))
    }
}
