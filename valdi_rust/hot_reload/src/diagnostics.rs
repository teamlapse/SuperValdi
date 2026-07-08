use valdi_rust_ir::{hot_reload::CompatibilityClass, ids::SourceSpanId};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HotReloadDiagnosticSeverity {
    Info,
    Warning,
    Error,
}

impl HotReloadDiagnosticSeverity {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HotReloadDiagnostic {
    pub code: &'static str,
    pub path: &'static str,
    pub severity: HotReloadDiagnosticSeverity,
    pub compatibility: CompatibilityClass,
    pub reason: &'static str,
    pub message: String,
    pub source_span_id: Option<SourceSpanId>,
}

impl HotReloadDiagnostic {
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
            severity: HotReloadDiagnosticSeverity::Error,
            compatibility: CompatibilityClass::RequiresRebuild,
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
            severity: HotReloadDiagnosticSeverity::Error,
            compatibility: CompatibilityClass::UiOnly,
            reason,
            message: message.into(),
            source_span_id,
        }
    }
}

pub type HotReloadResult<T> = Result<T, HotReloadDiagnostic>;

pub const HOT_RELOAD_PARSE_ERROR: &str = "HOT_RELOAD_PARSE_ERROR";
pub const HOT_RELOAD_NODE_MISSING: &str = "HOT_RELOAD_NODE_MISSING";
pub const HOT_RELOAD_BINDING_MISSING: &str = "HOT_RELOAD_BINDING_MISSING";
pub const HOT_RELOAD_ACTION_MISSING: &str = "HOT_RELOAD_ACTION_MISSING";
pub const HOT_RELOAD_REBUILD_REQUIRED: &str = "HOT_RELOAD_REBUILD_REQUIRED";
pub const HOT_RELOAD_MODULE_REF_MISSING: &str = "HOT_RELOAD_MODULE_REF_MISSING";
pub const HOT_RELOAD_NATIVE_VIEW_REF_MISSING: &str = "HOT_RELOAD_NATIVE_VIEW_REF_MISSING";
pub const HOT_RELOAD_MODULE_CONTRACT_SHAPE_CHANGED: &str =
    "HOT_RELOAD_MODULE_CONTRACT_SHAPE_CHANGED";
pub const HOT_RELOAD_NATIVE_VIEW_CONTRACT_SHAPE_CHANGED: &str =
    "HOT_RELOAD_NATIVE_VIEW_CONTRACT_SHAPE_CHANGED";
pub const HOT_RELOAD_TRACE_DRIFT: &str = "HOT_RELOAD_TRACE_DRIFT";
pub const HOT_RELOAD_LATENCY_DRIFT: &str = "HOT_RELOAD_LATENCY_DRIFT";
