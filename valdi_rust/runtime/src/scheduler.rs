use valdi_rust_ir::{
    actions::{
        ActionError, ActionKind, ActionResult, CancellationPolicy, CoalescingKey, SchedulingPolicy,
    },
    ids::ActionId,
};

use crate::{
    actions::{
        ActionDispatchRecord, ActionDispatchStatus, RuntimeActionBehavior, RuntimeActionDefinition,
    },
    diagnostics::{RuntimeDiagnostic, RuntimeResult},
    state::{StateInvalidation, StateStore},
};

pub const RUNTIME_ACTION_DUPLICATE: &str = "RUNTIME_ACTION_DUPLICATE";
pub const RUNTIME_ACTION_MISSING: &str = "RUNTIME_ACTION_MISSING";
pub const RUNTIME_ACTION_PENDING_MISSING: &str = "RUNTIME_ACTION_PENDING_MISSING";
pub const RUNTIME_ACTION_CANCELLED_REUSE: &str = "RUNTIME_ACTION_CANCELLED_REUSE";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PendingAction {
    pub action_id: ActionId,
    pub coalescing_key: Option<CoalescingKey>,
    pub cancellation_identity: Option<ActionId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionScheduler {
    actions: Vec<RuntimeActionDefinition>,
    pending: Vec<PendingAction>,
    cancelled: Vec<ActionId>,
}

impl ActionScheduler {
    pub fn new(actions: Vec<RuntimeActionDefinition>) -> RuntimeResult<Self> {
        for (index, action) in actions.iter().enumerate() {
            if actions[index + 1..]
                .iter()
                .any(|other| other.definition.id == action.definition.id)
            {
                return Err(RuntimeDiagnostic::error(
                    RUNTIME_ACTION_DUPLICATE,
                    "$.actions.action_id",
                    format!("duplicate action {}", action.definition.id.as_str()),
                    action.source_span_id,
                ));
            }
        }
        Ok(Self {
            actions,
            pending: Vec::new(),
            cancelled: Vec::new(),
        })
    }

    pub fn has_action(&self, action_id: ActionId) -> bool {
        self.actions
            .iter()
            .any(|action| action.definition.id == action_id)
    }

    pub fn dispatch(
        &mut self,
        action_id: ActionId,
        state: &mut StateStore,
    ) -> RuntimeResult<ActionDispatchRecord> {
        let action = self.action(action_id)?.clone();
        if self.is_deferred(&action) {
            if let Some(coalescing_key) = action.coalescing_key {
                if self
                    .pending
                    .iter()
                    .any(|pending| pending.coalescing_key == Some(coalescing_key))
                {
                    return Ok(record(
                        &action,
                        ActionDispatchStatus::Coalesced,
                        None,
                        Vec::new(),
                    ));
                }
            }
            self.pending.push(PendingAction {
                action_id,
                coalescing_key: action.coalescing_key,
                cancellation_identity: cancellation_identity(&action),
            });
            return Ok(record(
                &action,
                ActionDispatchStatus::Scheduled,
                None,
                Vec::new(),
            ));
        }
        self.execute(&action, state)
    }

    pub fn cancel(&mut self, action_id: ActionId) -> RuntimeResult<ActionDispatchRecord> {
        let action = self.action(action_id)?.clone();
        if let Some(index) = self
            .pending
            .iter()
            .position(|pending| pending.action_id == action_id)
        {
            self.pending.remove(index);
            self.cancelled.push(action_id);
            return Ok(record(
                &action,
                ActionDispatchStatus::Cancelled,
                Some(ActionResult::Cancelled),
                Vec::new(),
            ));
        }
        Err(RuntimeDiagnostic::error(
            RUNTIME_ACTION_PENDING_MISSING,
            "$.actions.pending",
            format!("pending action {} is missing", action_id.as_str()),
            action.source_span_id,
        ))
    }

    pub fn complete_async(
        &mut self,
        action_id: ActionId,
        state: &mut StateStore,
    ) -> RuntimeResult<ActionDispatchRecord> {
        let action = self.action(action_id)?.clone();
        if self.cancelled.contains(&action_id) {
            return Err(RuntimeDiagnostic::error(
                RUNTIME_ACTION_CANCELLED_REUSE,
                "$.actions.cancellation_identity",
                format!(
                    "cancelled action {} cannot be completed",
                    action_id.as_str()
                ),
                action.source_span_id,
            ));
        }
        let Some(index) = self
            .pending
            .iter()
            .position(|pending| pending.action_id == action_id)
        else {
            return Err(RuntimeDiagnostic::error(
                RUNTIME_ACTION_PENDING_MISSING,
                "$.actions.pending",
                format!("pending action {} is missing", action_id.as_str()),
                action.source_span_id,
            ));
        };
        self.pending.remove(index);
        self.execute(&action, state)
    }

    fn action(&self, action_id: ActionId) -> RuntimeResult<&RuntimeActionDefinition> {
        self.actions
            .iter()
            .find(|action| action.definition.id == action_id)
            .ok_or_else(|| {
                RuntimeDiagnostic::error(
                    RUNTIME_ACTION_MISSING,
                    "$.actions.action_id",
                    format!("action {} is missing", action_id.as_str()),
                    None,
                )
            })
    }

    fn is_deferred(&self, action: &RuntimeActionDefinition) -> bool {
        action.definition.kind == ActionKind::Async
            || action.scheduling_policy == SchedulingPolicy::Deferred
    }

    fn execute(
        &self,
        action: &RuntimeActionDefinition,
        state: &mut StateStore,
    ) -> RuntimeResult<ActionDispatchRecord> {
        match action.behavior {
            RuntimeActionBehavior::Fail(error) => Ok(record(
                action,
                ActionDispatchStatus::Failed(error),
                Some(ActionResult::Failed(error)),
                Vec::new(),
            )),
            RuntimeActionBehavior::Complete | RuntimeActionBehavior::DeterministicAsyncComplete => {
                let invalidations = invalidate_targets(action, state)?;
                Ok(record(
                    action,
                    ActionDispatchStatus::Completed,
                    Some(ActionResult::Completed),
                    invalidations,
                ))
            }
        }
    }
}

fn cancellation_identity(action: &RuntimeActionDefinition) -> Option<ActionId> {
    match action.cancellation_policy {
        CancellationPolicy::CancelByActionId => Some(action.definition.id),
        CancellationPolicy::NotCancellable => None,
    }
}

fn invalidate_targets(
    action: &RuntimeActionDefinition,
    state: &mut StateStore,
) -> RuntimeResult<Vec<StateInvalidation>> {
    action
        .invalidation_targets
        .iter()
        .map(|target| state.invalidate(target.state_id, "action"))
        .collect()
}

fn record(
    action: &RuntimeActionDefinition,
    status: ActionDispatchStatus,
    result: Option<ActionResult>,
    invalidations: Vec<StateInvalidation>,
) -> ActionDispatchRecord {
    ActionDispatchRecord {
        action: action.definition,
        status,
        result,
        scheduling_policy: action.scheduling_policy,
        cancellation_policy: action.cancellation_policy,
        coalescing_key: action.coalescing_key,
        invalidations,
    }
}

pub fn typed_action_error(code: &'static str) -> ActionError {
    ActionError { code }
}
