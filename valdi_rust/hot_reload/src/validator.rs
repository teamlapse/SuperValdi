use valdi_rust_ir::ids::NodeId;
use valdi_rust_runtime::{ActionScheduler, RuntimeDocument, StateStore};

use crate::{
    compatibility::validate_binding_and_action_compatibility,
    diagnostics::{HotReloadResult, HOT_RELOAD_NODE_MISSING},
    patch::{HotReloadEdit, HotReloadPatchIntent},
    registries::{MockModuleRegistry, MockNativeViewRegistry},
    HotReloadDiagnostic,
};

pub struct HotReloadValidationContext<'a> {
    pub document: &'a RuntimeDocument,
    pub state: &'a StateStore,
    pub scheduler: &'a ActionScheduler,
    pub modules: &'a MockModuleRegistry,
    pub native_views: &'a MockNativeViewRegistry,
}

pub fn validate_patch_intent(
    intent: &HotReloadPatchIntent,
    context: &HotReloadValidationContext<'_>,
) -> HotReloadResult<()> {
    match &intent.edit {
        HotReloadEdit::UiTree(tree) => {
            require_node(
                context.document,
                tree.parent_id,
                "$.patch.ui_tree.parent_id",
                intent,
            )?;
        }
        HotReloadEdit::Style(style) => {
            require_node(
                context.document,
                style.node_id,
                "$.patch.style.node_id",
                intent,
            )?;
        }
        HotReloadEdit::Layout(layout) => {
            require_node(
                context.document,
                layout.node_id,
                "$.patch.layout.node_id",
                intent,
            )?;
        }
        HotReloadEdit::Text(text) => {
            require_node(
                context.document,
                text.node_id,
                "$.patch.text.node_id",
                intent,
            )?;
        }
        HotReloadEdit::Asset(asset) => {
            require_node(
                context.document,
                asset.node_id,
                "$.patch.asset.node_id",
                intent,
            )?;
        }
        HotReloadEdit::Event(event) => {
            require_node(
                context.document,
                event.binding.node_id,
                "$.patch.event.node_id",
                intent,
            )?;
        }
        HotReloadEdit::Accessibility(accessibility) => {
            require_node(
                context.document,
                accessibility.node.node_id,
                "$.patch.accessibility.node_id",
                intent,
            )?;
        }
        HotReloadEdit::ModuleRef(module_ref) => {
            context
                .modules
                .validate(module_ref.module_id, module_ref.expected_shape)?;
        }
        HotReloadEdit::NativeViewRef(native_view_ref) => {
            context.native_views.validate(
                native_view_ref.native_view_id,
                native_view_ref.expected_shape,
            )?;
        }
        HotReloadEdit::Binding(_) | HotReloadEdit::ActionBody(_) => {}
    }

    validate_binding_and_action_compatibility(intent, context.state, |action_id| {
        context.scheduler.has_action(action_id)
    })
}

fn require_node(
    document: &RuntimeDocument,
    node_id: NodeId,
    path: &'static str,
    intent: &HotReloadPatchIntent,
) -> HotReloadResult<()> {
    if document.contains_node(node_id) {
        return Ok(());
    }
    Err(HotReloadDiagnostic::rebuild_required(
        HOT_RELOAD_NODE_MISSING,
        path,
        "patch references a node outside the retained document",
        format!("node {} is missing from session document", node_id.as_str()),
        intent.source_span_id,
    ))
}
