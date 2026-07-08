use valdi_rust_ir::{
    elements::ElementKind,
    ids::{ComponentId, KeyId, NodeId, SourceSpanId, StateId},
    tree::{ChildOrder, DestructionPolicy, RootNode, TreeChild, UiNode},
};
use valdi_rust_runtime::{
    load_runtime_document, RuntimeChild, RuntimeDocument, RuntimeDocumentDeclaration,
    RuntimeNodeDeclaration, RuntimeNodeRole,
};

use crate::{
    diagnostics::HotReloadResult, latency::hot_reload_latency_report,
    sample_host::HotReloadSampleHost, HotReloadDiagnostic,
};

pub const HOT_RELOAD_SAMPLE_SESSION_ID: &str = "hot_reload.session.v1";
pub const HOT_RELOAD_TRACE_FIXTURE_ID: &str = "hot_reload.trace.v1";

pub const SUPPORTED_PATCH_INPUTS: &[&str] = &[
    "tree patch_id=patch.tree parent=node.hot_reload.root node=node.hot_reload.dynamic kind=text key=key.dynamic order=2 span=hot_reload.dsl:1:1",
    "style patch_id=patch.style node=node.hot_reload.text color=12,34,56,255 span=hot_reload.dsl:2:1",
    "layout patch_id=patch.layout node=node.hot_reload.root direction=row span=hot_reload.dsl:3:1",
    "text patch_id=patch.text node=node.hot_reload.text value=Updated_Title span=hot_reload.dsl:4:1",
    "asset patch_id=patch.asset node=node.hot_reload.image asset=asset.logo source=logo@2x.png span=hot_reload.dsl:5:1",
    "binding patch_id=patch.binding binding=binding.hot_reload.title path=app.user.name span=hot_reload.dsl:6:1",
    "event patch_id=patch.event node=node.button event=tap action=action.save span=hot_reload.dsl:7:1",
    "accessibility patch_id=patch.accessibility node=node.button role=button label=Save span=hot_reload.dsl:8:1",
    "module_ref patch_id=patch.module module=module.storage shape=storage.v1 span=hot_reload.dsl:9:1",
    "native_view_ref patch_id=patch.native_view native_view=native.camera shape=camera.v1 span=hot_reload.dsl:10:1",
];

pub const ACTION_BODY_PATCH_INPUT: &str =
    "action_body patch_id=patch.action_body action=action.save edit=business_logic span=hot_reload.dsl:11:1";

pub const MODULE_SHAPE_DRIFT_INPUT: &str =
    "module_ref patch_id=patch.module_drift module=module.storage shape=storage.v2 span=hot_reload.dsl:12:1";

pub const NATIVE_VIEW_SHAPE_DRIFT_INPUT: &str =
    "native_view_ref patch_id=patch.native_drift native_view=native.camera shape=camera.v2 span=hot_reload.dsl:13:1";

pub fn sample_runtime_document() -> HotReloadResult<RuntimeDocument> {
    load_runtime_document(&HOT_RELOAD_DOCUMENT).map_err(|diagnostic| {
        HotReloadDiagnostic::error(
            "HOT_RELOAD_SAMPLE_DOCUMENT_INVALID",
            diagnostic.path,
            "sample document invalid",
            diagnostic.message,
            diagnostic.source_span_id,
        )
    })
}

pub fn hot_reload_trace_snapshot() -> HotReloadResult<String> {
    let mut host = HotReloadSampleHost::new()?;
    for input in SUPPORTED_PATCH_INPUTS {
        host.apply_declarative(input)?;
    }
    Ok(host.trace_snapshot())
}

pub fn validate_hot_reload_trace_snapshot(expected: &str) -> HotReloadResult<()> {
    let actual = hot_reload_trace_snapshot()?;
    if actual == expected {
        return Ok(());
    }
    Err(HotReloadDiagnostic::error(
        crate::diagnostics::HOT_RELOAD_TRACE_DRIFT,
        "$.hot_reload.trace",
        "trace snapshot drift",
        "hot reload trace snapshot drift",
        None,
    ))
}

pub fn hot_reload_latency_snapshot() -> String {
    hot_reload_latency_report()
}

const HOT_RELOAD_NODES: &[RuntimeNodeDeclaration] = &[
    RuntimeNodeDeclaration {
        role: RuntimeNodeRole::Element(UiNode {
            node_id: NodeId::new("node.hot_reload.root"),
            kind: ElementKind::View,
            state_id: None,
        }),
        destruction_policy: DestructionPolicy::DestroySubtree,
        source_span_id: Some(SourceSpanId::new("hot_reload_fixture:1:1")),
    },
    RuntimeNodeDeclaration {
        role: RuntimeNodeRole::Element(UiNode {
            node_id: NodeId::new("node.hot_reload.text"),
            kind: ElementKind::Text,
            state_id: Some(StateId::new("app")),
        }),
        destruction_policy: DestructionPolicy::DestroySubtree,
        source_span_id: Some(SourceSpanId::new("hot_reload_fixture:2:3")),
    },
    RuntimeNodeDeclaration {
        role: RuntimeNodeRole::Element(UiNode {
            node_id: NodeId::new("node.hot_reload.image"),
            kind: ElementKind::Image,
            state_id: None,
        }),
        destruction_policy: DestructionPolicy::DestroySubtree,
        source_span_id: Some(SourceSpanId::new("hot_reload_fixture:3:3")),
    },
    RuntimeNodeDeclaration {
        role: RuntimeNodeRole::Element(UiNode {
            node_id: NodeId::new("node.button"),
            kind: ElementKind::Control,
            state_id: Some(StateId::new("app")),
        }),
        destruction_policy: DestructionPolicy::DestroySubtree,
        source_span_id: Some(SourceSpanId::new("hot_reload_fixture:4:3")),
    },
    RuntimeNodeDeclaration {
        role: RuntimeNodeRole::Element(UiNode {
            node_id: NodeId::new("node.hot_reload.native_view"),
            kind: ElementKind::NativeView,
            state_id: None,
        }),
        destruction_policy: DestructionPolicy::DestroySubtree,
        source_span_id: Some(SourceSpanId::new("hot_reload_fixture:5:3")),
    },
];

const HOT_RELOAD_CHILDREN: &[RuntimeChild] = &[
    RuntimeChild {
        tree: TreeChild {
            parent_id: NodeId::new("node.hot_reload.root"),
            child_id: NodeId::new("node.hot_reload.text"),
            order: ChildOrder(0),
            key: Some(KeyId::new("key.text")),
        },
        source_span_id: Some(SourceSpanId::new("hot_reload_fixture:2:3")),
    },
    RuntimeChild {
        tree: TreeChild {
            parent_id: NodeId::new("node.hot_reload.root"),
            child_id: NodeId::new("node.hot_reload.image"),
            order: ChildOrder(1),
            key: Some(KeyId::new("key.image")),
        },
        source_span_id: Some(SourceSpanId::new("hot_reload_fixture:3:3")),
    },
    RuntimeChild {
        tree: TreeChild {
            parent_id: NodeId::new("node.hot_reload.root"),
            child_id: NodeId::new("node.button"),
            order: ChildOrder(2),
            key: Some(KeyId::new("key.button")),
        },
        source_span_id: Some(SourceSpanId::new("hot_reload_fixture:4:3")),
    },
    RuntimeChild {
        tree: TreeChild {
            parent_id: NodeId::new("node.hot_reload.root"),
            child_id: NodeId::new("node.hot_reload.native_view"),
            order: ChildOrder(3),
            key: Some(KeyId::new("key.native_view")),
        },
        source_span_id: Some(SourceSpanId::new("hot_reload_fixture:5:3")),
    },
];

const HOT_RELOAD_DOCUMENT: RuntimeDocumentDeclaration = RuntimeDocumentDeclaration {
    fixture_id: HOT_RELOAD_TRACE_FIXTURE_ID,
    component_id: ComponentId::new("component.hot_reload"),
    root: RootNode {
        node_id: NodeId::new("node.hot_reload.root"),
    },
    nodes: HOT_RELOAD_NODES,
    children: HOT_RELOAD_CHILDREN,
    source_span_id: Some(SourceSpanId::new("hot_reload_fixture:1:1")),
};
