use crate::source::{DynamicUiSourceKind, DynamicUiSourceMetadata};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DynamicUiProducerCapability {
    JsonDebugInput,
    BinaryBytesInput,
    GeneratedFixtureInput,
    InMemoryInput,
    RuntimeValidation,
    RuntimeBridge,
    MockBackendSink,
}

impl DynamicUiProducerCapability {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::JsonDebugInput => "json_debug_input",
            Self::BinaryBytesInput => "binary_bytes_input",
            Self::GeneratedFixtureInput => "generated_fixture_input",
            Self::InMemoryInput => "in_memory_input",
            Self::RuntimeValidation => "runtime_validation",
            Self::RuntimeBridge => "runtime_bridge",
            Self::MockBackendSink => "mock_backend_sink",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DynamicUiProducerCapabilitySet {
    pub capabilities: &'static [DynamicUiProducerCapability],
}

impl DynamicUiProducerCapabilitySet {
    pub const fn new(capabilities: &'static [DynamicUiProducerCapability]) -> Self {
        Self { capabilities }
    }

    pub const fn all() -> Self {
        Self {
            capabilities: ALL_DYNAMIC_UI_CAPABILITIES,
        }
    }

    pub fn supports(self, capability: DynamicUiProducerCapability) -> bool {
        self.capabilities.contains(&capability)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DynamicUiCapabilityDecision {
    pub capability: DynamicUiProducerCapability,
    pub supported: bool,
}

pub const ALL_DYNAMIC_UI_CAPABILITIES: &[DynamicUiProducerCapability] = &[
    DynamicUiProducerCapability::JsonDebugInput,
    DynamicUiProducerCapability::BinaryBytesInput,
    DynamicUiProducerCapability::GeneratedFixtureInput,
    DynamicUiProducerCapability::InMemoryInput,
    DynamicUiProducerCapability::RuntimeValidation,
    DynamicUiProducerCapability::RuntimeBridge,
    DynamicUiProducerCapability::MockBackendSink,
];

pub const BASE_DYNAMIC_UI_CAPABILITIES: &[DynamicUiProducerCapability] = &[
    DynamicUiProducerCapability::RuntimeValidation,
    DynamicUiProducerCapability::RuntimeBridge,
    DynamicUiProducerCapability::MockBackendSink,
];

pub fn negotiate_capabilities(
    capabilities: DynamicUiProducerCapabilitySet,
    source: DynamicUiSourceMetadata,
) -> Vec<DynamicUiCapabilityDecision> {
    required_capabilities(source)
        .into_iter()
        .map(|capability| DynamicUiCapabilityDecision {
            capability,
            supported: capabilities.supports(capability),
        })
        .collect()
}

pub fn required_capabilities(source: DynamicUiSourceMetadata) -> Vec<DynamicUiProducerCapability> {
    let mut required = Vec::with_capacity(BASE_DYNAMIC_UI_CAPABILITIES.len() + 1);
    required.push(match source.kind {
        DynamicUiSourceKind::JsonDebug(_) => DynamicUiProducerCapability::JsonDebugInput,
        DynamicUiSourceKind::BinaryBytes(_) => DynamicUiProducerCapability::BinaryBytesInput,
        DynamicUiSourceKind::GeneratedFixture(_) => {
            DynamicUiProducerCapability::GeneratedFixtureInput
        }
        DynamicUiSourceKind::InMemory(_) => DynamicUiProducerCapability::InMemoryInput,
    });
    required.extend(BASE_DYNAMIC_UI_CAPABILITIES.iter().copied());
    required
}
