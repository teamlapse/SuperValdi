//! Rust logic hot patch support for the declared action-body edit class.

pub mod detector;
pub mod dev_server;
pub mod diagnostics;
pub mod fixtures;
pub mod loader;
pub mod release_guard;
pub mod support_matrix;

pub use detector::{
    detect_action_body_hot_patch, detect_hot_reload_action_body_patch, ActionBodyFingerprint,
    ActionBodyHotPatch, RustEditClass, UnsupportedEditDiagnostic, UnsupportedEditInput,
    REQUIRED_UNSUPPORTED_EDIT_CLASSES,
};
pub use dev_server::{HotPatchDevServerMessage, HotPatchMessageKind, HotPatchSession};
pub use diagnostics::{
    HotPatchDiagnostic, HotPatchDiagnosticSeverity, HotPatchResult,
    HOT_PATCH_ACTION_BODY_SUPPORTED, HOT_PATCH_ACTION_MISSING,
    HOT_PATCH_CRATE_GRAPH_REBUILD_REQUIRED, HOT_PATCH_DEPENDENCY_REBUILD_REQUIRED,
    HOT_PATCH_MACRO_REBUILD_REQUIRED, HOT_PATCH_MODULE_REBUILD_REQUIRED,
    HOT_PATCH_NO_ACTION_BODY_CHANGE, HOT_PATCH_PLATFORM_BOUNDARY_REBUILD_REQUIRED,
    HOT_PATCH_RELEASE_EXCLUDED, HOT_PATCH_SIGNATURE_REBUILD_REQUIRED,
    HOT_PATCH_STATE_SHAPE_REBUILD_REQUIRED, HOT_PATCH_SUPPORT_MATRIX_DRIFT, HOT_PATCH_TRACE_DRIFT,
    HOT_PATCH_TYPE_REBUILD_REQUIRED,
};
pub use fixtures::{
    hot_patch_support_matrix_snapshot, hot_patch_trace_snapshot,
    sample_action_body_after_fingerprint, sample_action_body_before_fingerprint,
    sample_action_implementation, sample_detected_action_body_patch,
    sample_unsupported_after_fingerprint, validate_hot_patch_support_matrix_snapshot,
    validate_hot_patch_trace_snapshot, HOT_PATCH_SAMPLE_SESSION_ID, HOT_PATCH_TRACE_FIXTURE_ID,
};
pub use loader::{ActionImplementation, DevActionImplementationLoader, HotPatchLoadRecord};
pub use release_guard::{
    debug_build_report, release_build_report, validate_release_exclusion, HotPatchBuildProfile,
    ReleaseExclusionReport,
};
pub use support_matrix::{
    support_decision, support_matrix, support_matrix_report, validate_support_matrix,
    HotPatchSupportDecision, HotPatchSupportStatus,
};

pub const CRATE_ID: &str = "valdi_rust_hot_patch";
pub const OWNER_PR: &str = "PR12";
pub const PUBLIC_API_BOUNDARY: &str = "rust_logic_hot_patch_dev_tooling";

pub fn foundation_dependency_ids() -> [&'static str; 3] {
    [
        valdi_rust_hot_reload::CRATE_ID,
        valdi_rust_ir::CRATE_ID,
        valdi_rust_runtime::CRATE_ID,
    ]
}
