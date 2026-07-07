//! Invalid fixture metadata for diagnostic-family coverage.

use valdi_rust_ir::ids::FixtureId;

pub const INVALID_FIXTURE_MANIFEST_JSON: &str = include_str!("../invalid_fixture_manifest.json");

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InvalidFixture {
    pub id: FixtureId,
    pub diagnostic_family: &'static str,
    pub expected_code: &'static str,
    pub expected_path: &'static str,
}

pub const INVALID_FIXTURES: &[InvalidFixture] = &[
    InvalidFixture { id: FixtureId::new("invalid.schema_path.v1"), diagnostic_family: "schema_path", expected_code: "IR_SCHEMA_PATH_INVALID", expected_path: "$.root.children[0].kind" },
    InvalidFixture { id: FixtureId::new("invalid.source_span.v1"), diagnostic_family: "source_span", expected_code: "IR_SOURCE_SPAN_MISSING", expected_path: "source://fixtures/diagnostics.valdi.rs:1:1" },
    InvalidFixture { id: FixtureId::new("invalid.backend_path.v1"), diagnostic_family: "backend_path", expected_code: "IR_BACKEND_PATH_UNRESOLVED", expected_path: "backend://rust_backend/root" },
    InvalidFixture { id: FixtureId::new("invalid.capability_error.v1"), diagnostic_family: "capability_error", expected_code: "IR_CAPABILITY_UNSUPPORTED", expected_path: "$.capabilities[missing]" },
    InvalidFixture { id: FixtureId::new("invalid.unsupported_surface.v1"), diagnostic_family: "unsupported_surface", expected_code: "IR_SURFACE_UNSUPPORTED", expected_path: "$.element.unsupported" },
    InvalidFixture { id: FixtureId::new("invalid.fixture_id.v1"), diagnostic_family: "fixture_id", expected_code: "IR_FIXTURE_ID_INVALID", expected_path: "$.fixture_id" },
    InvalidFixture { id: FixtureId::new("invalid.action_id.v1"), diagnostic_family: "action_id", expected_code: "IR_ACTION_ID_UNRESOLVED", expected_path: "$.actions[missing]" },
    InvalidFixture { id: FixtureId::new("invalid.module_id.v1"), diagnostic_family: "module_id", expected_code: "IR_MODULE_ID_UNRESOLVED", expected_path: "$.modules[missing]" },
];
