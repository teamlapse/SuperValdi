use valdi_rust_ir::{
    actions::{
        ActionDefinition, ActionError, ActionResult, CancellationPolicy, CoalescingKey,
        InvalidationTarget, SchedulingPolicy,
    },
    ids::SourceSpanId,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeActionDefinition {
    pub definition: ActionDefinition,
    pub cancellation_policy: CancellationPolicy,
    pub scheduling_policy: SchedulingPolicy,
    pub coalescing_key: Option<CoalescingKey>,
    pub invalidation_targets: Vec<InvalidationTarget>,
    pub behavior: RuntimeActionBehavior,
    pub source_span_id: Option<SourceSpanId>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeActionBehavior {
    Complete,
    DeterministicAsyncComplete,
    Fail(ActionError),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActionDispatchStatus {
    Completed,
    Scheduled,
    Coalesced,
    Failed(ActionError),
    Cancelled,
}

impl ActionDispatchStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Completed => "completed",
            Self::Scheduled => "scheduled",
            Self::Coalesced => "coalesced",
            Self::Failed(_) => "failed",
            Self::Cancelled => "cancelled",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionDispatchRecord {
    pub action: ActionDefinition,
    pub status: ActionDispatchStatus,
    pub result: Option<ActionResult>,
    pub scheduling_policy: SchedulingPolicy,
    pub cancellation_policy: CancellationPolicy,
    pub coalescing_key: Option<CoalescingKey>,
    pub invalidations: Vec<crate::state::StateInvalidation>,
}
