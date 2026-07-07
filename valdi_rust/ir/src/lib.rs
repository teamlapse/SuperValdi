//! Rust-owned UI IR schema for SuperValdi.
//!
//! ```
//! use valdi_rust_ir::{elements::ElementKind, ids::NodeId, schema};
//!
//! let node = NodeId::new("root");
//! assert_eq!(node.as_str(), "root");
//! assert_eq!(ElementKind::View.contract_token(), "view");
//! assert_eq!(schema::CONTRACT_ROW_COVERAGE.len(), schema::CONTRACT_ROW_COUNT);
//! ```

pub mod accessibility;
pub mod actions;
pub mod animations;
pub mod assets;
pub mod bindings;
pub mod build_graph;
pub mod diagnostics;
pub mod dynamic_ui;
pub mod elements;
pub mod events;
pub mod extensions;
pub mod hot_reload;
pub mod ids;
pub mod layout;
pub mod native_modules;
pub mod native_views;
pub mod platform;
pub mod png;
pub mod schema;
pub mod styling;
pub mod text;
pub mod tree;
pub mod ts_compatibility;
pub mod web_dom;

pub const CRATE_ID: &str = "valdi_rust_ir";
pub const OWNER_PR: &str = "PR03";
pub const PUBLIC_API_BOUNDARY: &str = "rust_full_ui_ir_schema";

pub fn crate_id() -> &'static str {
    CRATE_ID
}
