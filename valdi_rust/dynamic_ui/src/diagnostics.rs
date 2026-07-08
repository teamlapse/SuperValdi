use crate::{source::DynamicUiSourceMetadata, trust::DynamicUiTrustMetadata};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DynamicUiDiagnosticSeverity {
    Info,
    Warning,
    Error,
}

impl DynamicUiDiagnosticSeverity {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DynamicUiDiagnostic {
    pub code: String,
    pub path: String,
    pub severity: DynamicUiDiagnosticSeverity,
    pub source: DynamicUiSourceMetadata,
    pub trust: DynamicUiTrustMetadata,
    pub node_id: Option<String>,
    pub owner_pr: Option<String>,
    pub message: String,
}

impl DynamicUiDiagnostic {
    pub fn error(
        code: impl Into<String>,
        path: impl Into<String>,
        source: DynamicUiSourceMetadata,
        trust: DynamicUiTrustMetadata,
        node_id: Option<impl Into<String>>,
        owner_pr: Option<impl Into<String>>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            path: path.into(),
            severity: DynamicUiDiagnosticSeverity::Error,
            source,
            trust,
            node_id: node_id.map(Into::into),
            owner_pr: owner_pr.map(Into::into),
            message: message.into(),
        }
    }

    pub fn from_codec(
        diagnostic: valdi_rust_codec::CodecDiagnostic,
        source: DynamicUiSourceMetadata,
        trust: DynamicUiTrustMetadata,
        node_id: Option<String>,
        owner_pr: Option<String>,
    ) -> Self {
        Self {
            code: diagnostic.code,
            path: diagnostic.path,
            severity: DynamicUiDiagnosticSeverity::Error,
            source,
            trust,
            node_id,
            owner_pr,
            message: diagnostic.message,
        }
    }
}

pub type DynamicUiResult<T> = Result<T, DynamicUiDiagnostic>;

pub const DYNAMIC_UI_CAPABILITY_UNSUPPORTED: &str = "DYNAMIC_UI_CAPABILITY_UNSUPPORTED";
pub const DYNAMIC_UI_SCHEMA_PATH_INVALID: &str = "DYNAMIC_UI_SCHEMA_PATH_INVALID";
pub const DYNAMIC_UI_SOURCE_UNTRUSTED: &str = "DYNAMIC_UI_SOURCE_UNTRUSTED";
pub const DYNAMIC_UI_OWNER_PR_MISMATCH: &str = "DYNAMIC_UI_OWNER_PR_MISMATCH";
pub const DYNAMIC_UI_NODE_ID_DRIFT: &str = "DYNAMIC_UI_NODE_ID_DRIFT";
pub const DYNAMIC_UI_RUNTIME_NODE_UNKNOWN: &str = "DYNAMIC_UI_RUNTIME_NODE_UNKNOWN";
pub const DYNAMIC_UI_RUNTIME_COMPONENT_UNKNOWN: &str = "DYNAMIC_UI_RUNTIME_COMPONENT_UNKNOWN";
pub const DYNAMIC_UI_RUNTIME_FIXTURE_UNKNOWN: &str = "DYNAMIC_UI_RUNTIME_FIXTURE_UNKNOWN";
pub const DYNAMIC_UI_TRACE_DRIFT: &str = "DYNAMIC_UI_TRACE_DRIFT";
pub const DYNAMIC_UI_INVALID_DIAGNOSTIC_DRIFT: &str = "DYNAMIC_UI_INVALID_DIAGNOSTIC_DRIFT";
