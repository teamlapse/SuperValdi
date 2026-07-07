//! Actions and state schema.
//!
//! ```
//! use valdi_rust_ir::{actions::{ActionDefinition, ActionKind}, ids::{ActionId, StateId}};
//!
//! let action = ActionDefinition { id: ActionId::new("save"), kind: ActionKind::Sync, state_scope: StateId::new("form") };
//! assert_eq!(action.kind, ActionKind::Sync);
//! ```

use crate::ids::{ActionId, StateId};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActionDefinition {
    pub id: ActionId,
    pub kind: ActionKind,
    pub state_scope: StateId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActionKind {
    Sync,
    Async,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActionResult {
    Completed,
    Failed(ActionError),
    Cancelled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActionError {
    pub code: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CancellationPolicy {
    NotCancellable,
    CancelByActionId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InvalidationTarget {
    pub state_id: StateId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SchedulingPolicy {
    Immediate,
    Deferred,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CoalescingKey {
    pub value: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateSlot {
    pub id: StateId,
    pub name: &'static str,
}
