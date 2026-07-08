use valdi_rust_backend::{
    capabilities::{BackendCapabilitySet, BackendTarget},
    mock::MockBackend,
    operations::BackendOperation,
};
use valdi_rust_codec::json_debug::FixtureEnvelope;
use valdi_rust_ir::{
    elements::ElementKind,
    ids::{ComponentId, NodeId, SourceSpanId},
    tree::{DestructionPolicy, RootNode, UiNode},
};
use valdi_rust_runtime::{
    apply_diff_to_backend, load_runtime_document, RuntimeChild, RuntimeDocument,
    RuntimeDocumentDeclaration, RuntimeNodeDeclaration, RuntimeNodeRole,
};

use crate::{
    diagnostics::{
        DynamicUiDiagnostic, DynamicUiResult, DYNAMIC_UI_RUNTIME_COMPONENT_UNKNOWN,
        DYNAMIC_UI_RUNTIME_FIXTURE_UNKNOWN, DYNAMIC_UI_RUNTIME_NODE_UNKNOWN,
    },
    source::DynamicUiSourceMetadata,
    trust::DynamicUiTrustMetadata,
};

#[derive(Clone, Debug)]
pub struct DynamicUiRuntimeReceipt {
    pub operations: Vec<BackendOperation>,
    pub mock_backend_operation_count: usize,
    pub snapshot: String,
}

pub fn runtime_document_from_fixture(
    fixture: &FixtureEnvelope,
    source: DynamicUiSourceMetadata,
    trust: DynamicUiTrustMetadata,
) -> DynamicUiResult<RuntimeDocument> {
    let root_node_id = root_node_id_for(
        &fixture.ir_debug.root_node_id,
        source,
        trust,
        Some(fixture.ir_debug.root_node_id.clone()),
        Some(fixture.metadata.owner_prs.join(",")),
    )?;
    let component_id = component_id_for(
        &fixture.ir_debug.component_id,
        source,
        trust,
        Some(fixture.ir_debug.root_node_id.clone()),
        Some(fixture.metadata.owner_prs.join(",")),
    )?;
    let fixture_id = fixture_id_for(
        &fixture.fixture_id,
        source,
        trust,
        Some(fixture.ir_debug.root_node_id.clone()),
        Some(fixture.metadata.owner_prs.join(",")),
    )?;
    let nodes = nodes_for_root_id(
        &fixture.ir_debug.root_node_id,
        source,
        trust,
        Some(fixture.ir_debug.root_node_id.clone()),
        Some(fixture.metadata.owner_prs.join(",")),
    )?;
    let declaration = RuntimeDocumentDeclaration {
        fixture_id,
        component_id,
        root: RootNode {
            node_id: root_node_id,
        },
        nodes,
        children: EMPTY_CHILDREN,
        source_span_id: None,
    };
    load_runtime_document(&declaration).map_err(|diagnostic| {
        DynamicUiDiagnostic::error(
            diagnostic.code,
            diagnostic.path,
            source,
            trust,
            Some(fixture.ir_debug.root_node_id.clone()),
            Some(fixture.metadata.owner_prs.join(",")),
            diagnostic.message,
        )
    })
}

pub fn apply_to_mock_backend(
    document: &RuntimeDocument,
) -> DynamicUiResult<DynamicUiRuntimeReceipt> {
    let baseline = baseline_document()?;
    let mut backend = MockBackend::new(BackendCapabilitySet::all_for(BackendTarget::RustHost));
    let operations =
        apply_diff_to_backend(&baseline, document, &mut backend).map_err(|diagnostic| {
            DynamicUiDiagnostic::error(
                diagnostic.code,
                diagnostic.path,
                DynamicUiSourceMetadata::in_memory("runtime_bridge", "$.runtime"),
                DynamicUiTrustMetadata::first_party("PR13"),
                Some(document.root.node_id.as_str()),
                Some("PR13"),
                diagnostic.message,
            )
        })?;
    let snapshot = format_backend_snapshot(document.fixture_id, &operations);
    Ok(DynamicUiRuntimeReceipt {
        operations,
        mock_backend_operation_count: backend.operations().len(),
        snapshot,
    })
}

pub fn format_backend_snapshot(fixture_id: &str, operations: &[BackendOperation]) -> String {
    let mut lines = vec![
        "dynamic_ui_trace_v1".to_string(),
        format!("fixture={fixture_id}"),
        "operations:".to_string(),
    ];
    for operation in operations {
        lines.push(format!(
            "- family={} capability={}",
            operation.family().as_str(),
            operation.family().required_capability().as_str()
        ));
    }
    format!("{}\n", lines.join("\n"))
}

pub fn validate_dynamic_ui_trace_snapshot(
    document: &RuntimeDocument,
    expected: &str,
) -> DynamicUiResult<()> {
    let receipt = apply_to_mock_backend(document)?;
    if receipt.snapshot == expected {
        return Ok(());
    }
    Err(DynamicUiDiagnostic::error(
        crate::diagnostics::DYNAMIC_UI_TRACE_DRIFT,
        "$.dynamic_ui.trace",
        DynamicUiSourceMetadata::in_memory("snapshot", "$.runtime"),
        DynamicUiTrustMetadata::first_party("PR13"),
        Some(document.root.node_id.as_str()),
        Some("PR13"),
        "dynamic UI trace snapshot drift",
    ))
}

fn baseline_document() -> DynamicUiResult<RuntimeDocument> {
    let declaration = RuntimeDocumentDeclaration {
        fixture_id: "dynamic_ui.baseline.v1",
        component_id: ComponentId::new("component.dynamic_ui.baseline"),
        root: RootNode {
            node_id: NodeId::new("node.dynamic_ui.baseline.root"),
        },
        nodes: BASELINE_NODES,
        children: EMPTY_CHILDREN,
        source_span_id: Some(SourceSpanId::new("dynamic_ui:baseline")),
    };
    load_runtime_document(&declaration).map_err(|diagnostic| {
        DynamicUiDiagnostic::error(
            diagnostic.code,
            diagnostic.path,
            DynamicUiSourceMetadata::in_memory("baseline", "$.runtime"),
            DynamicUiTrustMetadata::first_party("PR13"),
            Some("node.dynamic_ui.baseline.root"),
            Some("PR13"),
            diagnostic.message,
        )
    })
}

macro_rules! runtime_nodes {
    ($name:ident, $node_id:literal) => {
        const $name: &[RuntimeNodeDeclaration] = &[RuntimeNodeDeclaration {
            role: RuntimeNodeRole::Element(UiNode {
                node_id: NodeId::new($node_id),
                kind: ElementKind::View,
                state_id: None,
            }),
            destruction_policy: DestructionPolicy::DestroySubtree,
            source_span_id: None,
        }];
    };
}

runtime_nodes!(BASELINE_NODES, "node.dynamic_ui.baseline.root");
runtime_nodes!(SCHEMA_VERSIONING_NODES, "node.schema_versioning.root");
runtime_nodes!(COMPONENT_IDENTITY_NODES, "node.component_identity.root");
runtime_nodes!(TREE_STRUCTURE_NODES, "node.tree_structure.root");
runtime_nodes!(ELEMENT_TAXONOMY_NODES, "node.element_taxonomy.root");
runtime_nodes!(LAYOUT_NODES, "node.layout.root");
runtime_nodes!(STYLING_NODES, "node.styling.root");
runtime_nodes!(TEXT_NODES, "node.text.root");
runtime_nodes!(ASSETS_NODES, "node.assets.root");
runtime_nodes!(EVENTS_GESTURES_NODES, "node.events_gestures.root");
runtime_nodes!(ACTIONS_STATE_NODES, "node.actions_state.root");
runtime_nodes!(BINDINGS_EXPRESSIONS_NODES, "node.bindings_expressions.root");
runtime_nodes!(ANIMATIONS_NODES, "node.animations.root");
runtime_nodes!(NATIVE_MODULES_NODES, "node.native_modules.root");
runtime_nodes!(NATIVE_VIEWS_NODES, "node.native_views.root");
runtime_nodes!(ACCESSIBILITY_NODES, "node.accessibility.root");
runtime_nodes!(HOT_RELOAD_NODES, "node.hot_reload.root");
runtime_nodes!(DIAGNOSTICS_NODES, "node.diagnostics.root");
runtime_nodes!(WEB_DOM_NODES, "node.web_dom.root");
runtime_nodes!(PNG_BACKEND_NODES, "node.png_backend.root");
runtime_nodes!(DYNAMIC_UI_NODES, "node.dynamic_ui.root");
runtime_nodes!(TS_COMPATIBILITY_NODES, "node.ts_compatibility.root");
runtime_nodes!(BUILD_GRAPH_NODES, "node.build_graph.root");

const EMPTY_CHILDREN: &[RuntimeChild] = &[];

fn root_node_id_for(
    value: &str,
    source: DynamicUiSourceMetadata,
    trust: DynamicUiTrustMetadata,
    node_id: Option<String>,
    owner_pr: Option<String>,
) -> DynamicUiResult<NodeId> {
    match value {
        "node.schema_versioning.root" => Ok(NodeId::new("node.schema_versioning.root")),
        "node.component_identity.root" => Ok(NodeId::new("node.component_identity.root")),
        "node.tree_structure.root" => Ok(NodeId::new("node.tree_structure.root")),
        "node.element_taxonomy.root" => Ok(NodeId::new("node.element_taxonomy.root")),
        "node.layout.root" => Ok(NodeId::new("node.layout.root")),
        "node.styling.root" => Ok(NodeId::new("node.styling.root")),
        "node.text.root" => Ok(NodeId::new("node.text.root")),
        "node.assets.root" => Ok(NodeId::new("node.assets.root")),
        "node.events_gestures.root" => Ok(NodeId::new("node.events_gestures.root")),
        "node.actions_state.root" => Ok(NodeId::new("node.actions_state.root")),
        "node.bindings_expressions.root" => Ok(NodeId::new("node.bindings_expressions.root")),
        "node.animations.root" => Ok(NodeId::new("node.animations.root")),
        "node.native_modules.root" => Ok(NodeId::new("node.native_modules.root")),
        "node.native_views.root" => Ok(NodeId::new("node.native_views.root")),
        "node.accessibility.root" => Ok(NodeId::new("node.accessibility.root")),
        "node.hot_reload.root" => Ok(NodeId::new("node.hot_reload.root")),
        "node.diagnostics.root" => Ok(NodeId::new("node.diagnostics.root")),
        "node.web_dom.root" => Ok(NodeId::new("node.web_dom.root")),
        "node.png_backend.root" => Ok(NodeId::new("node.png_backend.root")),
        "node.dynamic_ui.root" => Ok(NodeId::new("node.dynamic_ui.root")),
        "node.ts_compatibility.root" => Ok(NodeId::new("node.ts_compatibility.root")),
        "node.build_graph.root" => Ok(NodeId::new("node.build_graph.root")),
        _ => Err(DynamicUiDiagnostic::error(
            DYNAMIC_UI_RUNTIME_NODE_UNKNOWN,
            "$.ir_debug.root_node_id",
            source,
            trust,
            node_id,
            owner_pr,
            format!("runtime bridge does not know node {value:?}"),
        )),
    }
}

fn nodes_for_root_id(
    value: &str,
    source: DynamicUiSourceMetadata,
    trust: DynamicUiTrustMetadata,
    node_id: Option<String>,
    owner_pr: Option<String>,
) -> DynamicUiResult<&'static [RuntimeNodeDeclaration]> {
    match value {
        "node.schema_versioning.root" => Ok(SCHEMA_VERSIONING_NODES),
        "node.component_identity.root" => Ok(COMPONENT_IDENTITY_NODES),
        "node.tree_structure.root" => Ok(TREE_STRUCTURE_NODES),
        "node.element_taxonomy.root" => Ok(ELEMENT_TAXONOMY_NODES),
        "node.layout.root" => Ok(LAYOUT_NODES),
        "node.styling.root" => Ok(STYLING_NODES),
        "node.text.root" => Ok(TEXT_NODES),
        "node.assets.root" => Ok(ASSETS_NODES),
        "node.events_gestures.root" => Ok(EVENTS_GESTURES_NODES),
        "node.actions_state.root" => Ok(ACTIONS_STATE_NODES),
        "node.bindings_expressions.root" => Ok(BINDINGS_EXPRESSIONS_NODES),
        "node.animations.root" => Ok(ANIMATIONS_NODES),
        "node.native_modules.root" => Ok(NATIVE_MODULES_NODES),
        "node.native_views.root" => Ok(NATIVE_VIEWS_NODES),
        "node.accessibility.root" => Ok(ACCESSIBILITY_NODES),
        "node.hot_reload.root" => Ok(HOT_RELOAD_NODES),
        "node.diagnostics.root" => Ok(DIAGNOSTICS_NODES),
        "node.web_dom.root" => Ok(WEB_DOM_NODES),
        "node.png_backend.root" => Ok(PNG_BACKEND_NODES),
        "node.dynamic_ui.root" => Ok(DYNAMIC_UI_NODES),
        "node.ts_compatibility.root" => Ok(TS_COMPATIBILITY_NODES),
        "node.build_graph.root" => Ok(BUILD_GRAPH_NODES),
        _ => Err(DynamicUiDiagnostic::error(
            DYNAMIC_UI_RUNTIME_NODE_UNKNOWN,
            "$.ir_debug.root_node_id",
            source,
            trust,
            node_id,
            owner_pr,
            format!("runtime bridge does not know node {value:?}"),
        )),
    }
}

fn component_id_for(
    value: &str,
    source: DynamicUiSourceMetadata,
    trust: DynamicUiTrustMetadata,
    node_id: Option<String>,
    owner_pr: Option<String>,
) -> DynamicUiResult<ComponentId> {
    match value {
        "component.schema_versioning" => Ok(ComponentId::new("component.schema_versioning")),
        "component.component_identity" => Ok(ComponentId::new("component.component_identity")),
        "component.tree_structure" => Ok(ComponentId::new("component.tree_structure")),
        "component.element_taxonomy" => Ok(ComponentId::new("component.element_taxonomy")),
        "component.layout" => Ok(ComponentId::new("component.layout")),
        "component.styling" => Ok(ComponentId::new("component.styling")),
        "component.text" => Ok(ComponentId::new("component.text")),
        "component.assets" => Ok(ComponentId::new("component.assets")),
        "component.events_gestures" => Ok(ComponentId::new("component.events_gestures")),
        "component.actions_state" => Ok(ComponentId::new("component.actions_state")),
        "component.bindings_expressions" => Ok(ComponentId::new("component.bindings_expressions")),
        "component.animations" => Ok(ComponentId::new("component.animations")),
        "component.native_modules" => Ok(ComponentId::new("component.native_modules")),
        "component.native_views" => Ok(ComponentId::new("component.native_views")),
        "component.accessibility" => Ok(ComponentId::new("component.accessibility")),
        "component.hot_reload" => Ok(ComponentId::new("component.hot_reload")),
        "component.diagnostics" => Ok(ComponentId::new("component.diagnostics")),
        "component.web_dom" => Ok(ComponentId::new("component.web_dom")),
        "component.png_backend" => Ok(ComponentId::new("component.png_backend")),
        "component.dynamic_ui" => Ok(ComponentId::new("component.dynamic_ui")),
        "component.ts_compatibility" => Ok(ComponentId::new("component.ts_compatibility")),
        "component.build_graph" => Ok(ComponentId::new("component.build_graph")),
        _ => Err(DynamicUiDiagnostic::error(
            DYNAMIC_UI_RUNTIME_COMPONENT_UNKNOWN,
            "$.ir_debug.component_id",
            source,
            trust,
            node_id,
            owner_pr,
            format!("runtime bridge does not know component {value:?}"),
        )),
    }
}

fn fixture_id_for(
    value: &str,
    source: DynamicUiSourceMetadata,
    trust: DynamicUiTrustMetadata,
    node_id: Option<String>,
    owner_pr: Option<String>,
) -> DynamicUiResult<&'static str> {
    match value {
        "contract.schema_versioning.v1" => Ok("contract.schema_versioning.v1"),
        "contract.component_identity.v1" => Ok("contract.component_identity.v1"),
        "contract.tree_structure.v1" => Ok("contract.tree_structure.v1"),
        "contract.element_taxonomy.v1" => Ok("contract.element_taxonomy.v1"),
        "contract.layout.v1" => Ok("contract.layout.v1"),
        "contract.styling.v1" => Ok("contract.styling.v1"),
        "contract.text.v1" => Ok("contract.text.v1"),
        "contract.assets.v1" => Ok("contract.assets.v1"),
        "contract.events_gestures.v1" => Ok("contract.events_gestures.v1"),
        "contract.actions_state.v1" => Ok("contract.actions_state.v1"),
        "contract.bindings_expressions.v1" => Ok("contract.bindings_expressions.v1"),
        "contract.animations.v1" => Ok("contract.animations.v1"),
        "contract.native_modules.v1" => Ok("contract.native_modules.v1"),
        "contract.native_views.v1" => Ok("contract.native_views.v1"),
        "contract.accessibility.v1" => Ok("contract.accessibility.v1"),
        "contract.hot_reload.v1" => Ok("contract.hot_reload.v1"),
        "contract.diagnostics.v1" => Ok("contract.diagnostics.v1"),
        "contract.web_dom.v1" => Ok("contract.web_dom.v1"),
        "contract.png_backend.v1" => Ok("contract.png_backend.v1"),
        "contract.dynamic_ui.v1" => Ok("contract.dynamic_ui.v1"),
        "contract.ts_compatibility.v1" => Ok("contract.ts_compatibility.v1"),
        "contract.build_graph.v1" => Ok("contract.build_graph.v1"),
        _ => Err(DynamicUiDiagnostic::error(
            DYNAMIC_UI_RUNTIME_FIXTURE_UNKNOWN,
            "$.fixture_id",
            source,
            trust,
            node_id,
            owner_pr,
            format!("runtime bridge does not know fixture {value:?}"),
        )),
    }
}
