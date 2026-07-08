use valdi_rust_codec::json_debug::FixtureEnvelope;

use crate::{
    capabilities::DynamicUiProducerCapabilitySet, diagnostics::DynamicUiResult,
    source::DynamicUiSourceMetadata, trust::DynamicUiTrustMetadata,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DynamicProducerDescriptor {
    pub name: &'static str,
    pub owner_pr: &'static str,
    pub source: DynamicUiSourceMetadata,
    pub trust: DynamicUiTrustMetadata,
    pub capabilities: DynamicUiProducerCapabilitySet,
}

pub trait DynamicProducer {
    fn descriptor(&self) -> DynamicProducerDescriptor;

    fn produce_fixture(&self) -> DynamicUiResult<FixtureEnvelope>;
}

#[derive(Clone, Debug)]
pub struct InMemoryDynamicProducer {
    descriptor: DynamicProducerDescriptor,
    fixture: FixtureEnvelope,
}

impl InMemoryDynamicProducer {
    pub fn new(descriptor: DynamicProducerDescriptor, fixture: FixtureEnvelope) -> Self {
        Self {
            descriptor,
            fixture,
        }
    }
}

impl DynamicProducer for InMemoryDynamicProducer {
    fn descriptor(&self) -> DynamicProducerDescriptor {
        self.descriptor
    }

    fn produce_fixture(&self) -> DynamicUiResult<FixtureEnvelope> {
        Ok(self.fixture.clone())
    }
}
