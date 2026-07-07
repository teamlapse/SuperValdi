//! Canonical fixture corpus metadata for the Rust-first SuperValdi migration.

pub mod corpus;
pub mod invalid;

pub const CRATE_ID: &str = "valdi_rust_fixtures";
pub const OWNER_PR: &str = "PR04";
pub const PUBLIC_API_BOUNDARY: &str = "rust_fixture_corpus_test_metadata";

pub fn crate_id() -> &'static str {
    CRATE_ID
}

pub fn dependency_ids() -> [&'static str; 1] {
    [valdi_rust_ir::CRATE_ID]
}
