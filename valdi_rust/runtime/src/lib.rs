use valdi_rust_backend::CRATE_ID as BACKEND_CRATE_ID;
use valdi_rust_ir::crate_id as ir_crate_id;

pub mod actions;
pub mod bindings;
pub mod diagnostics;
pub mod diff;
pub mod document;
pub mod event_bindings;
pub mod expressions;
pub mod identity;
pub mod patch;
pub mod scheduler;
pub mod state;
pub mod state_fixtures;
pub mod static_fixtures;
pub mod tree;

pub use actions::{
    ActionDispatchRecord, ActionDispatchStatus, RuntimeActionBehavior, RuntimeActionDefinition,
};
pub use bindings::{BindingResolution, BindingResolver, RuntimeBinding, RUNTIME_BINDING_MISSING};
pub use diagnostics::{RuntimeDiagnostic, RuntimeDiagnosticSeverity, RuntimeResult};
pub use diff::{apply_diff_to_backend, diff_documents};
pub use document::{load_runtime_document, RuntimeDocument, RuntimeDocumentDeclaration};
pub use event_bindings::{
    EventActionBindings, RUNTIME_EVENT_ACTION_MISSING, RUNTIME_EVENT_BINDING_MISSING,
};
pub use expressions::{
    evaluate_expression, literal_value, ExpressionContext, RuntimeComparisonExpression,
    RuntimeComputedProjectionExpression, RuntimeExpression, RuntimeFieldExpression,
    RuntimeListProjectionExpression, RuntimeLiteralExpression, RuntimeLogicExpression,
    RuntimeNullableExpression, RuntimePlatformConstantEntry, RuntimePlatformConstantExpression,
    RuntimePlatformConstants, RUNTIME_BINDING_TYPE_MISMATCH, RUNTIME_COMPUTED_PROJECTION_MISSING,
    RUNTIME_PLATFORM_CONSTANT_MISSING,
};
pub use identity::{IdentityTables, StateSlot};
pub use patch::{apply_identity_patch, validate_preserved_state_slots, IdentityPatchResult};
pub use scheduler::{
    typed_action_error, ActionScheduler, PendingAction, RUNTIME_ACTION_CANCELLED_REUSE,
    RUNTIME_ACTION_DUPLICATE, RUNTIME_ACTION_MISSING, RUNTIME_ACTION_PENDING_MISSING,
};
pub use state::{
    retain_compatible_state, StateEntry, StateField, StateInvalidation, StateStore, StateValue,
    StateValueKind, RUNTIME_BINDING_FIELD_MISSING, RUNTIME_STATE_DUPLICATE,
    RUNTIME_STATE_IDENTITY_INCOMPATIBLE, RUNTIME_STATE_MISSING, RUNTIME_STATE_TYPE_MISMATCH,
};
pub use state_fixtures::{
    sample_action_scheduler, sample_binding_resolver, sample_event_bindings,
    sample_ir_binding_expression, sample_list_projection_expression, sample_logic_expression,
    sample_null_literal_expression, sample_state_store, state_binding_action_trace_snapshot,
    validate_state_binding_action_trace_snapshot, RUNTIME_STATE_ACTION_TRACE_DRIFT,
    STATE_BINDING_ACTION_TRACE_FIXTURE_ID,
};
pub use static_fixtures::{
    compatible_patch_pair, incompatible_patch_pair, static_fixture_backend_ops_snapshot,
    static_tree_diff_pair, validate_static_fixture_backend_ops_snapshot,
};
pub use tree::{RuntimeChild, RuntimeNode, RuntimeNodeDeclaration, RuntimeNodeRole};

pub const CRATE_ID: &str = "valdi_rust_runtime";
pub const OWNER_PR: &str = "PR02";
pub const RUNTIME_TREE_DIFF_OWNER_PR: &str = "PR07";
pub const STATE_BINDINGS_ACTIONS_OWNER_PR: &str = "PR08";
pub const PUBLIC_API_BOUNDARY: &str = "rust_runtime_foundation";

pub fn foundation_dependency_ids() -> [&'static str; 2] {
    [BACKEND_CRATE_ID, ir_crate_id()]
}
