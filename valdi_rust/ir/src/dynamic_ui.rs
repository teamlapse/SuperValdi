//! Dynamic UI ingestion schema.
//!
//! ```
//! use valdi_rust_ir::dynamic_ui::{DynamicUiProducer, DynamicUiValidationState};
//!
//! let producer = DynamicUiProducer { name: "remote_home", validation_state: DynamicUiValidationState::Validated };
//! assert_eq!(producer.validation_state, DynamicUiValidationState::Validated);
//! ```

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DynamicUiProducer {
    pub name: &'static str,
    pub validation_state: DynamicUiValidationState,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DynamicUiInput {
    RustDsl,
    GeneratedFixture(GeneratedFixtureRef),
    InMemory(InMemoryProducerTag),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DynamicUiValidationState {
    Unvalidated,
    Validated,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GeneratedFixtureRef {
    pub fixture_name: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DebugJsonLabel {
    pub label: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BinaryBytesLabel {
    pub label: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InMemoryProducerTag {
    pub name: &'static str,
}
