use valdi_rust_ir::ids::SourceSpanId;

pub const DSL_ATTRIBUTE_ELEMENT_MISMATCH: &str = "DSL_ATTRIBUTE_ELEMENT_MISMATCH";
pub const DSL_EVENT_ELEMENT_MISMATCH: &str = "DSL_EVENT_ELEMENT_MISMATCH";
pub const DSL_SOURCE_SPAN_REQUIRED: &str = "DSL_SOURCE_SPAN_REQUIRED";
pub const DSL_CONTRACT_ROW_UNSUPPORTED: &str = "DSL_CONTRACT_ROW_UNSUPPORTED";
pub const DSL_FIXTURE_INPUT_INVALID: &str = "DSL_FIXTURE_INPUT_INVALID";
pub const DSL_CANONICAL_IR_DRIFT: &str = "DSL_CANONICAL_IR_DRIFT";
pub const DSL_FORBIDDEN_DEPENDENCY: &str = "DSL_FORBIDDEN_DEPENDENCY";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DslDiagnosticSeverity {
    Info,
    Warning,
    Error,
}

impl DslDiagnosticSeverity {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DslDiagnostic {
    pub code: &'static str,
    pub path: &'static str,
    pub severity: DslDiagnosticSeverity,
    pub message: String,
    pub source_span_id: Option<SourceSpanId>,
}

impl DslDiagnostic {
    pub fn error(
        code: &'static str,
        path: &'static str,
        message: impl Into<String>,
        source_span_id: Option<SourceSpanId>,
    ) -> Self {
        Self {
            code,
            path,
            severity: DslDiagnosticSeverity::Error,
            message: message.into(),
            source_span_id,
        }
    }
}

pub type DslResult<T> = Result<T, DslDiagnostic>;
