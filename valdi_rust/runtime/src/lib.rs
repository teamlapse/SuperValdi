use valdi_rust_backend::CRATE_ID as BACKEND_CRATE_ID;
use valdi_rust_ir::crate_id as ir_crate_id;

pub const CRATE_ID: &str = "valdi_rust_runtime";
pub const OWNER_PR: &str = "PR02";
pub const PUBLIC_API_BOUNDARY: &str = "rust_runtime_foundation";

pub fn foundation_dependency_ids() -> [&'static str; 2] {
    [BACKEND_CRATE_ID, ir_crate_id()]
}
