#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackendDiagnosticSeverity {
    Info,
    Warning,
    Error,
}

impl BackendDiagnosticSeverity {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackendDiagnostic {
    pub code: &'static str,
    pub path: &'static str,
    pub severity: BackendDiagnosticSeverity,
    pub message: String,
}

impl BackendDiagnostic {
    pub fn error(code: &'static str, path: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            path,
            severity: BackendDiagnosticSeverity::Error,
            message: message.into(),
        }
    }
}

pub type BackendResult<T> = Result<T, BackendDiagnostic>;
