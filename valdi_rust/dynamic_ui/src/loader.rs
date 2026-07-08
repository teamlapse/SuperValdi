use valdi_rust_codec::{binary, json_debug, ContractDocument};
use valdi_rust_fixtures::corpus::SerializedFixture;
use valdi_rust_runtime::RuntimeDocument;

use crate::{
    diagnostics::{DynamicUiDiagnostic, DynamicUiResult},
    producer::DynamicProducer,
    runtime_bridge::runtime_document_from_fixture,
    source::DynamicUiSourceMetadata,
    trust::DynamicUiTrustMetadata,
    validator::{validate_dynamic_fixture, validate_producer_capabilities},
};

#[derive(Clone, Debug)]
pub struct DynamicUiDocument {
    pub fixture: json_debug::FixtureEnvelope,
    pub source: DynamicUiSourceMetadata,
    pub trust: DynamicUiTrustMetadata,
    pub runtime_document: RuntimeDocument,
}

pub fn load_json_debug(
    input: &str,
    contract: &ContractDocument,
    source: DynamicUiSourceMetadata,
    trust: DynamicUiTrustMetadata,
) -> DynamicUiResult<DynamicUiDocument> {
    let fixture = json_debug::decode_fixture(input).map_err(|diagnostic| {
        DynamicUiDiagnostic::from_codec(
            diagnostic,
            source,
            trust,
            None,
            Some(trust.owner_pr.into()),
        )
    })?;
    finalize_fixture(fixture, contract, source, trust)
}

pub fn load_binary_bytes(
    bytes: &[u8],
    contract: &ContractDocument,
    source: DynamicUiSourceMetadata,
    trust: DynamicUiTrustMetadata,
) -> DynamicUiResult<DynamicUiDocument> {
    let fixture = binary::decode_fixture(bytes).map_err(|diagnostic| {
        DynamicUiDiagnostic::from_codec(
            diagnostic,
            source,
            trust,
            None,
            Some(trust.owner_pr.into()),
        )
    })?;
    finalize_fixture(fixture, contract, source, trust)
}

pub fn load_generated_fixture(
    serialized: &SerializedFixture,
    contract: &ContractDocument,
    source: DynamicUiSourceMetadata,
    trust: DynamicUiTrustMetadata,
) -> DynamicUiResult<DynamicUiDocument> {
    let document = load_json_debug(serialized.contents, contract, source, trust)?;
    if document.fixture.fixture_id == serialized.id.as_str() {
        return Ok(document);
    }
    Err(DynamicUiDiagnostic::error(
        "DYNAMIC_UI_GENERATED_FIXTURE_ID_DRIFT",
        "$.fixture_id",
        source,
        trust,
        Some(document.fixture.ir_debug.root_node_id),
        Some(document.fixture.metadata.owner_prs.join(",")),
        format!(
            "generated fixture ID drift: {:?} != {:?}",
            document.fixture.fixture_id,
            serialized.id.as_str()
        ),
    ))
}

pub fn load_in_memory(
    fixture: json_debug::FixtureEnvelope,
    contract: &ContractDocument,
    source: DynamicUiSourceMetadata,
    trust: DynamicUiTrustMetadata,
) -> DynamicUiResult<DynamicUiDocument> {
    finalize_fixture(fixture, contract, source, trust)
}

pub fn load_from_producer<P: DynamicProducer>(
    producer: &P,
    contract: &ContractDocument,
) -> DynamicUiResult<DynamicUiDocument> {
    let descriptor = producer.descriptor();
    validate_producer_capabilities(descriptor)?;
    let fixture = producer.produce_fixture()?;
    finalize_fixture(fixture, contract, descriptor.source, descriptor.trust)
}

fn finalize_fixture(
    fixture: json_debug::FixtureEnvelope,
    contract: &ContractDocument,
    source: DynamicUiSourceMetadata,
    trust: DynamicUiTrustMetadata,
) -> DynamicUiResult<DynamicUiDocument> {
    validate_dynamic_fixture(&fixture, contract, source, trust)?;
    let runtime_document = runtime_document_from_fixture(&fixture, source, trust)?;
    Ok(DynamicUiDocument {
        fixture,
        source,
        trust,
        runtime_document,
    })
}
