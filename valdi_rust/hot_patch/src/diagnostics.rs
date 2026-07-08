use valdi_rust_ir::ids::SourceSpanId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HotPatchDiagnosticSeverity {
    Info,
    Warning,
    Error,
}

impl HotPatchDiagnosticSeverity {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HotPatchDiagnostic {
    pub code: &'static str,
    pub path: &'static str,
    pub severity: HotPatchDiagnosticSeverity,
    pub reason: &'static str,
    pub message: String,
    pub source_span_id: Option<SourceSpanId>,
}

impl HotPatchDiagnostic {
    pub fn rebuild_required(
        code: &'static str,
        path: &'static str,
        reason: &'static str,
        message: impl Into<String>,
        source_span_id: Option<SourceSpanId>,
    ) -> Self {
        Self {
            code,
            path,
            severity: HotPatchDiagnosticSeverity::Error,
            reason,
            message: message.into(),
            source_span_id,
        }
    }

    pub fn error(
        code: &'static str,
        path: &'static str,
        reason: &'static str,
        message: impl Into<String>,
        source_span_id: Option<SourceSpanId>,
    ) -> Self {
        Self {
            code,
            path,
            severity: HotPatchDiagnosticSeverity::Error,
            reason,
            message: message.into(),
            source_span_id,
        }
    }
}

pub type HotPatchResult<T> = Result<T, HotPatchDiagnostic>;

pub const HOT_PATCH_ACTION_BODY_SUPPORTED: &str = "HOT_PATCH_ACTION_BODY_SUPPORTED";
pub const HOT_PATCH_SIGNATURE_REBUILD_REQUIRED: &str = "HOT_PATCH_SIGNATURE_REBUILD_REQUIRED";
pub const HOT_PATCH_TYPE_REBUILD_REQUIRED: &str = "HOT_PATCH_TYPE_REBUILD_REQUIRED";
pub const HOT_PATCH_MODULE_REBUILD_REQUIRED: &str = "HOT_PATCH_MODULE_REBUILD_REQUIRED";
pub const HOT_PATCH_STATE_SHAPE_REBUILD_REQUIRED: &str = "HOT_PATCH_STATE_SHAPE_REBUILD_REQUIRED";
pub const HOT_PATCH_DEPENDENCY_REBUILD_REQUIRED: &str = "HOT_PATCH_DEPENDENCY_REBUILD_REQUIRED";
pub const HOT_PATCH_MACRO_REBUILD_REQUIRED: &str = "HOT_PATCH_MACRO_REBUILD_REQUIRED";
pub const HOT_PATCH_CRATE_GRAPH_REBUILD_REQUIRED: &str = "HOT_PATCH_CRATE_GRAPH_REBUILD_REQUIRED";
pub const HOT_PATCH_PLATFORM_BOUNDARY_REBUILD_REQUIRED: &str =
    "HOT_PATCH_PLATFORM_BOUNDARY_REBUILD_REQUIRED";
pub const HOT_PATCH_NO_ACTION_BODY_CHANGE: &str = "HOT_PATCH_NO_ACTION_BODY_CHANGE";
pub const HOT_PATCH_ACTION_MISSING: &str = "HOT_PATCH_ACTION_MISSING";
pub const HOT_PATCH_RELEASE_EXCLUDED: &str = "HOT_PATCH_RELEASE_EXCLUDED";
pub const HOT_PATCH_TRACE_DRIFT: &str = "HOT_PATCH_TRACE_DRIFT";
pub const HOT_PATCH_SUPPORT_MATRIX_DRIFT: &str = "HOT_PATCH_SUPPORT_MATRIX_DRIFT";
