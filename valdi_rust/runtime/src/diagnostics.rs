use valdi_rust_ir::ids::SourceSpanId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeDiagnosticSeverity {
    Info,
    Warning,
    Error,
}

impl RuntimeDiagnosticSeverity {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeDiagnostic {
    pub code: &'static str,
    pub path: &'static str,
    pub severity: RuntimeDiagnosticSeverity,
    pub message: String,
    pub source_span_id: Option<SourceSpanId>,
}

impl RuntimeDiagnostic {
    pub fn error(
        code: &'static str,
        path: &'static str,
        message: impl Into<String>,
        source_span_id: Option<SourceSpanId>,
    ) -> Self {
        Self {
            code,
            path,
            severity: RuntimeDiagnosticSeverity::Error,
            message: message.into(),
            source_span_id,
        }
    }
}

pub type RuntimeResult<T> = Result<T, RuntimeDiagnostic>;
