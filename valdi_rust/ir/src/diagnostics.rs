//! Diagnostic schema.
//!
//! ```
//! use valdi_rust_ir::{diagnostics::{Diagnostic, DiagnosticSeverity}, ids::DiagnosticId};
//!
//! let diagnostic = Diagnostic { id: DiagnosticId::new("d1"), severity: DiagnosticSeverity::Error };
//! assert_eq!(diagnostic.severity, DiagnosticSeverity::Error);
//! ```

use crate::ids::{ActionId, BackendPathId, CapabilityId, DiagnosticId, FixtureId, ModuleId, SourceSpanId};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Diagnostic {
    pub id: DiagnosticId,
    pub severity: DiagnosticSeverity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Error,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SchemaPath(pub &'static str);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackendPath(pub BackendPathId);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CapabilityError {
    pub capability_id: CapabilityId,
    pub reason: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnsupportedSurface {
    pub schema_path: SchemaPath,
    pub source_span_id: Option<SourceSpanId>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActionTraceId(pub ActionId);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ModuleTraceId(pub ModuleId);

pub type DiagnosticFixtureId = FixtureId;
