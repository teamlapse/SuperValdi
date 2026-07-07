use valdi_rust_ir::crate_id as ir_crate_id;

pub const CRATE_ID: &str = "valdi_rust_backend";
pub const OWNER_PR: &str = "PR02";
pub const PUBLIC_API_BOUNDARY: &str = "rust_backend_traits_foundation";

pub fn foundation_dependency_ids() -> [&'static str; 1] {
    [ir_crate_id()]
}
