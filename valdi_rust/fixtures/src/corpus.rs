//! Static fixture manifest and serialized IR debug assets.

use valdi_rust_ir::ids::FixtureId;

pub const CONTRACT_FIXTURE_MANIFEST_JSON: &str = include_str!("../contract_fixture_manifest.json");

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SerializedFixture {
    pub id: FixtureId,
    pub path: &'static str,
    pub contents: &'static str,
}

pub const SERIALIZED_FIXTURES: &[SerializedFixture] = &[
    SerializedFixture { id: FixtureId::new("contract.schema_versioning.v1"), path: "valdi_rust/fixtures/serialized/schema_versioning.ir.json", contents: include_str!("../serialized/schema_versioning.ir.json") },
    SerializedFixture { id: FixtureId::new("contract.component_identity.v1"), path: "valdi_rust/fixtures/serialized/component_identity.ir.json", contents: include_str!("../serialized/component_identity.ir.json") },
    SerializedFixture { id: FixtureId::new("contract.tree_structure.v1"), path: "valdi_rust/fixtures/serialized/tree_structure.ir.json", contents: include_str!("../serialized/tree_structure.ir.json") },
    SerializedFixture { id: FixtureId::new("contract.element_taxonomy.v1"), path: "valdi_rust/fixtures/serialized/element_taxonomy.ir.json", contents: include_str!("../serialized/element_taxonomy.ir.json") },
    SerializedFixture { id: FixtureId::new("contract.layout.v1"), path: "valdi_rust/fixtures/serialized/layout.ir.json", contents: include_str!("../serialized/layout.ir.json") },
    SerializedFixture { id: FixtureId::new("contract.styling.v1"), path: "valdi_rust/fixtures/serialized/styling.ir.json", contents: include_str!("../serialized/styling.ir.json") },
    SerializedFixture { id: FixtureId::new("contract.text.v1"), path: "valdi_rust/fixtures/serialized/text.ir.json", contents: include_str!("../serialized/text.ir.json") },
    SerializedFixture { id: FixtureId::new("contract.assets.v1"), path: "valdi_rust/fixtures/serialized/assets.ir.json", contents: include_str!("../serialized/assets.ir.json") },
    SerializedFixture { id: FixtureId::new("contract.events_gestures.v1"), path: "valdi_rust/fixtures/serialized/events_gestures.ir.json", contents: include_str!("../serialized/events_gestures.ir.json") },
    SerializedFixture { id: FixtureId::new("contract.actions_state.v1"), path: "valdi_rust/fixtures/serialized/actions_state.ir.json", contents: include_str!("../serialized/actions_state.ir.json") },
    SerializedFixture { id: FixtureId::new("contract.bindings_expressions.v1"), path: "valdi_rust/fixtures/serialized/bindings_expressions.ir.json", contents: include_str!("../serialized/bindings_expressions.ir.json") },
    SerializedFixture { id: FixtureId::new("contract.animations.v1"), path: "valdi_rust/fixtures/serialized/animations.ir.json", contents: include_str!("../serialized/animations.ir.json") },
    SerializedFixture { id: FixtureId::new("contract.native_modules.v1"), path: "valdi_rust/fixtures/serialized/native_modules.ir.json", contents: include_str!("../serialized/native_modules.ir.json") },
    SerializedFixture { id: FixtureId::new("contract.native_views.v1"), path: "valdi_rust/fixtures/serialized/native_views.ir.json", contents: include_str!("../serialized/native_views.ir.json") },
    SerializedFixture { id: FixtureId::new("contract.accessibility.v1"), path: "valdi_rust/fixtures/serialized/accessibility.ir.json", contents: include_str!("../serialized/accessibility.ir.json") },
    SerializedFixture { id: FixtureId::new("contract.hot_reload.v1"), path: "valdi_rust/fixtures/serialized/hot_reload.ir.json", contents: include_str!("../serialized/hot_reload.ir.json") },
    SerializedFixture { id: FixtureId::new("contract.diagnostics.v1"), path: "valdi_rust/fixtures/serialized/diagnostics.ir.json", contents: include_str!("../serialized/diagnostics.ir.json") },
    SerializedFixture { id: FixtureId::new("contract.web_dom.v1"), path: "valdi_rust/fixtures/serialized/web_dom.ir.json", contents: include_str!("../serialized/web_dom.ir.json") },
    SerializedFixture { id: FixtureId::new("contract.png_backend.v1"), path: "valdi_rust/fixtures/serialized/png_backend.ir.json", contents: include_str!("../serialized/png_backend.ir.json") },
    SerializedFixture { id: FixtureId::new("contract.dynamic_ui.v1"), path: "valdi_rust/fixtures/serialized/dynamic_ui.ir.json", contents: include_str!("../serialized/dynamic_ui.ir.json") },
    SerializedFixture { id: FixtureId::new("contract.ts_compatibility.v1"), path: "valdi_rust/fixtures/serialized/ts_compatibility.ir.json", contents: include_str!("../serialized/ts_compatibility.ir.json") },
    SerializedFixture { id: FixtureId::new("contract.build_graph.v1"), path: "valdi_rust/fixtures/serialized/build_graph.ir.json", contents: include_str!("../serialized/build_graph.ir.json") },
];
