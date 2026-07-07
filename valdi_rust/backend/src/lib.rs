//! Backend operation contract shared by future Rust renderers.

pub mod capabilities;
pub mod diagnostics;
pub mod mock;
pub mod operations;
pub mod validator;

pub use capabilities::{
    BackendCapability, BackendCapabilityDecision, BackendCapabilitySet, BackendTarget,
    FixtureTagBackendDecision, FIXTURE_TAG_BACKEND_DECISIONS,
};
pub use diagnostics::{BackendDiagnostic, BackendDiagnosticSeverity, BackendResult};
pub use mock::{mock_backend_snapshot, sample_backend_operations, MockBackend};
pub use operations::{BackendOperation, BackendOperationFamily, REQUIRED_OPERATION_FAMILIES};
pub use validator::{
    validate_fixture_tag_decisions, validate_operation_capabilities,
    validate_required_operation_families,
};

pub const CRATE_ID: &str = "valdi_rust_backend";
pub const OWNER_PR: &str = "PR02";
pub const BACKEND_OPERATION_OWNER_PR: &str = "PR06";
pub const PUBLIC_API_BOUNDARY: &str = "rust_backend_traits_foundation";

pub fn foundation_dependency_ids() -> [&'static str; 1] {
    [valdi_rust_ir::CRATE_ID]
}

pub trait RenderBackend {
    fn target(&self) -> BackendTarget;

    fn capabilities(&self) -> BackendCapabilitySet;

    fn negotiate(&self, capability: BackendCapability) -> BackendCapabilityDecision {
        if self.capabilities().supports(capability) {
            BackendCapabilityDecision::Supported(capability)
        } else {
            BackendCapabilityDecision::Unsupported(capability)
        }
    }

    fn apply(&mut self, operation: BackendOperation) -> BackendResult<()>;
}
