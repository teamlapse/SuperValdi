//! Rust app-facing UI DSL for producing typed SuperValdi IR.
//!
//! ```
//! use valdi_rust_dsl::{
//!     attributes::DslAttribute,
//!     elements,
//!     source::source_span,
//! };
//! use valdi_rust_ir::{
//!     ids::{ActionId, NodeId},
//!     events::EventKind,
//!     styling::{Color, Style},
//! };
//!
//! let root = elements::view(
//!     NodeId::new("root"),
//!     source_span("example:1:1", "app.rs", 1, 1),
//! )
//! .attr(DslAttribute::Style(Style::minimal(Color::rgba(0, 0, 0, 255))))?
//! .on(EventKind::Tap, ActionId::new("tap"))?
//! .build();
//!
//! assert_eq!(root.node.node_id.as_str(), "root");
//! # Ok::<(), valdi_rust_dsl::diagnostics::DslDiagnostic>(())
//! ```

pub mod accessibility;
pub mod attributes;
pub mod bindings;
pub mod diagnostics;
pub mod elements;
pub mod events;
pub mod fixtures;
pub mod modules;
pub mod native_views;
pub mod platform_extensions;
pub mod source;
pub mod tree;

pub use diagnostics::{
    DslDiagnostic, DslDiagnosticSeverity, DslResult, DSL_ATTRIBUTE_ELEMENT_MISMATCH,
    DSL_CANONICAL_IR_DRIFT, DSL_CONTRACT_ROW_UNSUPPORTED, DSL_EVENT_ELEMENT_MISMATCH,
    DSL_FIXTURE_INPUT_INVALID, DSL_FORBIDDEN_DEPENDENCY, DSL_SOURCE_SPAN_REQUIRED,
};
pub use tree::{DslCanonicalDebug, DslDocument, DslNode, DslSurfacePayload};

pub const CRATE_ID: &str = "valdi_rust_dsl";
pub const OWNER_PR: &str = "PR09";
pub const PUBLIC_API_BOUNDARY: &str = "rust_ui_dsl_app_authoring";

pub fn crate_id() -> &'static str {
    CRATE_ID
}

pub fn dependency_ids() -> [&'static str; 1] {
    [valdi_rust_ir::CRATE_ID]
}
