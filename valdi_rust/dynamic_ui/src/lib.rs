//! Dynamic UI ingestion for validated Rust UI IR.
//!
//! This crate accepts typed PR05 fixture envelopes from JSON debug, postcard
//! binary bytes, generated fixtures, or in-memory Rust producers, validates them
//! against the same PR01/PR04 contract metadata used by the DSL proof, and
//! bridges the result into the PR07 runtime/mock-backend path.

pub mod capabilities;
pub mod diagnostics;
pub mod fixtures;
pub mod loader;
pub mod producer;
pub mod runtime_bridge;
pub mod source;
pub mod trust;
pub mod validator;

pub use capabilities::{
    negotiate_capabilities, required_capabilities, DynamicUiCapabilityDecision,
    DynamicUiProducerCapability, DynamicUiProducerCapabilitySet,
};
pub use diagnostics::{
    DynamicUiDiagnostic, DynamicUiDiagnosticSeverity, DynamicUiResult,
    DYNAMIC_UI_CAPABILITY_UNSUPPORTED, DYNAMIC_UI_INVALID_DIAGNOSTIC_DRIFT,
    DYNAMIC_UI_NODE_ID_DRIFT, DYNAMIC_UI_OWNER_PR_MISMATCH, DYNAMIC_UI_RUNTIME_COMPONENT_UNKNOWN,
    DYNAMIC_UI_RUNTIME_FIXTURE_UNKNOWN, DYNAMIC_UI_RUNTIME_NODE_UNKNOWN,
    DYNAMIC_UI_SCHEMA_PATH_INVALID, DYNAMIC_UI_SOURCE_UNTRUSTED, DYNAMIC_UI_TRACE_DRIFT,
};
pub use loader::{
    load_binary_bytes, load_from_producer, load_generated_fixture, load_in_memory, load_json_debug,
    DynamicUiDocument,
};
pub use producer::{DynamicProducer, DynamicProducerDescriptor, InMemoryDynamicProducer};
pub use runtime_bridge::{
    apply_to_mock_backend, format_backend_snapshot, runtime_document_from_fixture,
    validate_dynamic_ui_trace_snapshot, DynamicUiRuntimeReceipt,
};
pub use source::{DynamicUiSourceKind, DynamicUiSourceMetadata, VALIDATED_IR_SCHEMA_PATH};
pub use trust::{DynamicUiTrustLevel, DynamicUiTrustMetadata};
pub use validator::{
    validate_capability_set, validate_dynamic_fixture, validate_producer_capabilities,
};

pub const CRATE_ID: &str = "valdi_rust_dynamic_ui";
pub const OWNER_PR: &str = "PR13";
pub const PUBLIC_API_BOUNDARY: &str = "rust_dynamic_ui_ingestion";

pub fn crate_id() -> &'static str {
    CRATE_ID
}

pub fn dependency_ids() -> [&'static str; 5] {
    [
        valdi_rust_backend::CRATE_ID,
        valdi_rust_codec::CRATE_ID,
        valdi_rust_fixtures::CRATE_ID,
        valdi_rust_ir::CRATE_ID,
        valdi_rust_runtime::CRATE_ID,
    ]
}
