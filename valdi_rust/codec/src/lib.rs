//! Typed codecs and validation for the PR04 fixture corpus.
//!
//! This crate serializes the fixture/debug envelope introduced by PR04. It does
//! not claim general runtime UI IR serialization for surfaces not represented in
//! the fixture corpus.

pub mod binary;
pub mod diagnostics;
pub mod inspect;
pub mod json_debug;
pub mod validator;

pub use diagnostics::{CodecDiagnostic, CodecResult, DiagnosticSeverity};
pub use json_debug::{
    ContractDocument, ContractRow, FixtureEnvelope, FixtureManifest, InvalidFixture,
    InvalidFixtureManifest,
};

pub const CRATE_ID: &str = "valdi_rust_codec";
pub const OWNER_PR: &str = "PR05";
pub const PUBLIC_API_BOUNDARY: &str = "rust_fixture_codec_validator";
pub const CURRENT_SCHEMA_VERSION: u16 = 1;
pub const CURRENT_JSON_DEBUG_VERSION: u16 = 1;
pub const CURRENT_BINARY_WIRE_VERSION: u16 = 1;

pub fn crate_id() -> &'static str {
    CRATE_ID
}

pub fn dependency_ids() -> [&'static str; 1] {
    [valdi_rust_ir::CRATE_ID]
}
