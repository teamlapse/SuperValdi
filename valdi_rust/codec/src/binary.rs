use serde::{Deserialize, Serialize};

use crate::diagnostics::{CodecDiagnostic, CodecResult};
use crate::json_debug::{
    validate_fixture_shape, validate_invalid_manifest_shape, FixtureEnvelope, FixtureMetadata,
    InvalidFixtureManifest, IrDebugEnvelope, SchemaVersion,
};
use crate::{CURRENT_BINARY_WIRE_VERSION, CURRENT_SCHEMA_VERSION};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BinaryFixtureEnvelope {
    pub schema_version: u16,
    pub wire_version: u16,
    pub payload: BinaryFixturePayload,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BinaryFixturePayload {
    pub fixture_schema_version: u16,
    pub fixture_id: String,
    pub contract_row_id: String,
    pub surface: String,
    pub fixture_tags: Vec<String>,
    pub platform_targets: Vec<String>,
    pub metadata: FixtureMetadata,
    pub ir_debug: BinaryIrDebugEnvelope,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BinaryIrDebugEnvelope {
    pub schema_version: SchemaVersion,
    pub contract_surface: String,
    pub root_node_id: String,
    pub component_id: String,
    pub source_span: Option<String>,
    pub coverage_tokens: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BinaryInvalidManifestEnvelope {
    pub schema_version: u16,
    pub wire_version: u16,
    pub payload: InvalidFixtureManifest,
}

pub fn encode_fixture(fixture: &FixtureEnvelope) -> CodecResult<Vec<u8>> {
    encode_fixture_with_versions(fixture, CURRENT_SCHEMA_VERSION, CURRENT_BINARY_WIRE_VERSION)
}

pub fn encode_fixture_with_versions(
    fixture: &FixtureEnvelope,
    schema_version: u16,
    wire_version: u16,
) -> CodecResult<Vec<u8>> {
    validate_fixture_shape(fixture)?;
    let envelope = BinaryFixtureEnvelope {
        schema_version,
        wire_version,
        payload: BinaryFixturePayload::from(fixture),
    };
    postcard::to_allocvec(&envelope)
        .map_err(|err| CodecDiagnostic::error("IR_BINARY_ENCODE_ERROR", "$.binary", err.to_string()))
}

pub fn decode_fixture(bytes: &[u8]) -> CodecResult<FixtureEnvelope> {
    let envelope = postcard::from_bytes::<BinaryFixtureEnvelope>(bytes).map_err(|err| {
        CodecDiagnostic::error("IR_BINARY_PAYLOAD_CORRUPT", "$.binary", err.to_string())
    })?;
    validate_binary_versions(envelope.schema_version, envelope.wire_version)?;
    let payload = FixtureEnvelope::from(envelope.payload);
    validate_fixture_shape(&payload)?;
    Ok(payload)
}

pub fn encode_invalid_manifest(manifest: &InvalidFixtureManifest) -> CodecResult<Vec<u8>> {
    validate_invalid_manifest_shape(manifest)?;
    let envelope = BinaryInvalidManifestEnvelope {
        schema_version: CURRENT_SCHEMA_VERSION,
        wire_version: CURRENT_BINARY_WIRE_VERSION,
        payload: manifest.clone(),
    };
    postcard::to_allocvec(&envelope)
        .map_err(|err| CodecDiagnostic::error("IR_BINARY_ENCODE_ERROR", "$.binary", err.to_string()))
}

pub fn decode_invalid_manifest(bytes: &[u8]) -> CodecResult<InvalidFixtureManifest> {
    let envelope = postcard::from_bytes::<BinaryInvalidManifestEnvelope>(bytes).map_err(|err| {
        CodecDiagnostic::error("IR_BINARY_PAYLOAD_CORRUPT", "$.binary", err.to_string())
    })?;
    validate_binary_versions(envelope.schema_version, envelope.wire_version)?;
    validate_invalid_manifest_shape(&envelope.payload)?;
    Ok(envelope.payload)
}

fn validate_binary_versions(schema_version: u16, wire_version: u16) -> CodecResult<()> {
    if schema_version != CURRENT_SCHEMA_VERSION {
        return Err(CodecDiagnostic::error(
            "IR_SCHEMA_VERSION_UNSUPPORTED",
            "$.binary.schema_version",
            format!("unsupported schema version {schema_version}"),
        ));
    }
    if wire_version != CURRENT_BINARY_WIRE_VERSION {
        return Err(CodecDiagnostic::error(
            "IR_BINARY_WIRE_VERSION_UNSUPPORTED",
            "$.binary.wire_version",
            format!("unsupported binary wire version {wire_version}"),
        ));
    }
    Ok(())
}

impl From<&FixtureEnvelope> for BinaryFixturePayload {
    fn from(fixture: &FixtureEnvelope) -> Self {
        Self {
            fixture_schema_version: fixture.fixture_schema_version,
            fixture_id: fixture.fixture_id.clone(),
            contract_row_id: fixture.contract_row_id.clone(),
            surface: fixture.surface.clone(),
            fixture_tags: fixture.fixture_tags.clone(),
            platform_targets: fixture.platform_targets.clone(),
            metadata: fixture.metadata.clone(),
            ir_debug: BinaryIrDebugEnvelope {
                schema_version: fixture.ir_debug.schema_version.clone(),
                contract_surface: fixture.ir_debug.contract_surface.clone(),
                root_node_id: fixture.ir_debug.root_node_id.clone(),
                component_id: fixture.ir_debug.component_id.clone(),
                source_span: fixture.ir_debug.source_span.clone(),
                coverage_tokens: fixture.ir_debug.coverage_tokens.clone(),
            },
        }
    }
}

impl From<BinaryFixturePayload> for FixtureEnvelope {
    fn from(payload: BinaryFixturePayload) -> Self {
        Self {
            fixture_schema_version: payload.fixture_schema_version,
            fixture_id: payload.fixture_id,
            contract_row_id: payload.contract_row_id,
            surface: payload.surface,
            fixture_tags: payload.fixture_tags,
            platform_targets: payload.platform_targets,
            metadata: payload.metadata,
            ir_debug: IrDebugEnvelope {
                schema_version: payload.ir_debug.schema_version,
                contract_surface: payload.ir_debug.contract_surface,
                root_node_id: payload.ir_debug.root_node_id,
                component_id: payload.ir_debug.component_id,
                source_span: payload.ir_debug.source_span,
                coverage_tokens: payload.ir_debug.coverage_tokens,
            },
        }
    }
}
