use valdi_rust_backend::CRATE_ID as BACKEND_CRATE_ID;
use valdi_rust_ir::crate_id as ir_crate_id;

pub mod diagnostics;
pub mod diff;
pub mod document;
pub mod identity;
pub mod patch;
pub mod static_fixtures;
pub mod tree;

pub use diagnostics::{RuntimeDiagnostic, RuntimeDiagnosticSeverity, RuntimeResult};
pub use diff::{apply_diff_to_backend, diff_documents};
pub use document::{load_runtime_document, RuntimeDocument, RuntimeDocumentDeclaration};
pub use identity::{IdentityTables, StateSlot};
pub use patch::{apply_identity_patch, validate_preserved_state_slots, IdentityPatchResult};
pub use static_fixtures::{
    compatible_patch_pair, incompatible_patch_pair, static_fixture_backend_ops_snapshot,
    static_tree_diff_pair, validate_static_fixture_backend_ops_snapshot,
};
pub use tree::{RuntimeChild, RuntimeNode, RuntimeNodeDeclaration, RuntimeNodeRole};

pub const CRATE_ID: &str = "valdi_rust_runtime";
pub const OWNER_PR: &str = "PR02";
pub const RUNTIME_TREE_DIFF_OWNER_PR: &str = "PR07";
pub const PUBLIC_API_BOUNDARY: &str = "rust_runtime_foundation";

pub fn foundation_dependency_ids() -> [&'static str; 2] {
    [BACKEND_CRATE_ID, ir_crate_id()]
}
