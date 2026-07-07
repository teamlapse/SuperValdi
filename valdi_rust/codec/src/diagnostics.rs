use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Error,
}

impl DiagnosticSeverity {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CodecDiagnostic {
    pub code: String,
    pub path: String,
    pub severity: DiagnosticSeverity,
    pub message: String,
}

impl CodecDiagnostic {
    pub fn error(code: impl Into<String>, path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            path: path.into(),
            severity: DiagnosticSeverity::Error,
            message: message.into(),
        }
    }
}

pub type CodecResult<T> = Result<T, CodecDiagnostic>;
