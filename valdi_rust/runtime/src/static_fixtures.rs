use valdi_rust_backend::{
    operations::{AttributeStyleChange, TransactionPhase},
    BackendOperation,
};
use valdi_rust_ir::{
    elements::ElementKind,
    ids::{ComponentId, KeyId, NodeId, SourceSpanId, StateId},
    tree::{
        ChildOrder, ContextScope, DestructionPolicy, FragmentNode, PortalNode, RootNode, SlotNode,
        TreeChild, UiNode,
    },
};

use crate::{
    diagnostics::{RuntimeDiagnostic, RuntimeResult},
    diff::diff_documents,
    document::{load_runtime_document, RuntimeDocument, RuntimeDocumentDeclaration},
    tree::{RuntimeChild, RuntimeNodeDeclaration, RuntimeNodeRole},
};

pub const TREE_FIXTURE_ID: &str = "runtime.tree_diff.v1";
pub const COMPATIBLE_PATCH_FIXTURE_ID: &str = "runtime.identity_patch.compatible.v1";
pub const INCOMPATIBLE_PATCH_FIXTURE_ID: &str = "runtime.identity_patch.incompatible.v1";

pub fn static_tree_diff_pair() -> RuntimeResult<(RuntimeDocument, RuntimeDocument)> {
    Ok((
        load_runtime_document(&TREE_DIFF_BEFORE)?,
        load_runtime_document(&TREE_DIFF_AFTER)?,
    ))
}

pub fn compatible_patch_pair() -> RuntimeResult<(RuntimeDocument, RuntimeDocument)> {
    Ok((
        load_runtime_document(&COMPATIBLE_PATCH_BEFORE)?,
        load_runtime_document(&COMPATIBLE_PATCH_AFTER)?,
    ))
}

pub fn incompatible_patch_pair() -> RuntimeResult<(RuntimeDocument, RuntimeDocument)> {
    Ok((
        load_runtime_document(&INCOMPATIBLE_PATCH_BEFORE)?,
        load_runtime_document(&INCOMPATIBLE_PATCH_AFTER)?,
    ))
}

pub fn static_fixture_backend_ops_snapshot() -> RuntimeResult<String> {
    let (before, after) = static_tree_diff_pair()?;
    let operations = diff_documents(&before, &after)?;
    Ok(format_backend_ops_snapshot(TREE_FIXTURE_ID, &operations))
}

pub fn validate_static_fixture_backend_ops_snapshot(expected: &str) -> RuntimeResult<()> {
    let actual = static_fixture_backend_ops_snapshot()?;
    if actual == expected {
        return Ok(());
    }
    Err(RuntimeDiagnostic::error(
        "RUNTIME_BACKEND_OP_SNAPSHOT_DRIFT",
        "$.runtime_tree_diff_snapshot",
        "static fixture backend operation snapshot drift",
        None,
    ))
}

fn format_backend_ops_snapshot(fixture_id: &str, operations: &[BackendOperation]) -> String {
    let mut lines = vec![
        "runtime_tree_diff_snapshot_v1".to_string(),
        format!("fixture={fixture_id}"),
    ];
    for operation in operations {
        lines.push(format!("- {}", describe_operation(operation)));
    }
    format!("{}\n", lines.join("\n"))
}

fn describe_operation(operation: &BackendOperation) -> String {
    match operation {
        BackendOperation::Create(operation) => format!(
            "family=create node={} kind={}",
            operation.identity.node_id.as_str(),
            operation.identity.kind.contract_token()
        ),
        BackendOperation::Root(operation) => {
            format!("family=root node={}", operation.node_id.as_str())
        }
        BackendOperation::Move(operation) => format!(
            "family=move parent={} child={} order={} key={}",
            operation.parent_id.as_str(),
            operation.child_id.as_str(),
            operation.order.0,
            operation.key.map(|key| key.as_str()).unwrap_or("<none>")
        ),
        BackendOperation::Destroy(operation) => format!(
            "family=destroy node={} policy={}",
            operation.node_id.as_str(),
            destruction_policy_name(operation.policy)
        ),
        BackendOperation::TransactionGroup(operation) => format!(
            "family=transaction_group phase={} group={}",
            transaction_phase_name(operation.phase),
            operation.group.name
        ),
        BackendOperation::AttributeStyle(operation) => format!(
            "family=attribute_style node={} change={}",
            operation.node_id.as_str(),
            attribute_change_name(operation.change)
        ),
        other => format!("family={}", other.family().as_str()),
    }
}

fn destruction_policy_name(policy: DestructionPolicy) -> &'static str {
    match policy {
        DestructionPolicy::DestroySubtree => "destroy_subtree",
        DestructionPolicy::PreserveForPool => "preserve_for_pool",
    }
}

fn transaction_phase_name(phase: TransactionPhase) -> &'static str {
    match phase {
        TransactionPhase::Begin => "begin",
        TransactionPhase::Commit => "commit",
        TransactionPhase::Cancel => "cancel",
    }
}

fn attribute_change_name(change: AttributeStyleChange) -> &'static str {
    match change {
        AttributeStyleChange::Attribute(_) => "attribute",
        AttributeStyleChange::Style(_) => "style",
    }
}

const TREE_DIFF_BEFORE_NODES: &[RuntimeNodeDeclaration] = &[
    RuntimeNodeDeclaration {
        role: RuntimeNodeRole::Element(UiNode {
            node_id: NodeId::new("node.root.old"),
            kind: ElementKind::View,
            state_id: None,
        }),
        destruction_policy: DestructionPolicy::DestroySubtree,
        source_span_id: Some(SourceSpanId::new("tree_before:1:1")),
    },
    RuntimeNodeDeclaration {
        role: RuntimeNodeRole::Element(UiNode {
            node_id: NodeId::new("node.item.stable"),
            kind: ElementKind::Text,
            state_id: Some(StateId::new("state.item.stable")),
        }),
        destruction_policy: DestructionPolicy::DestroySubtree,
        source_span_id: Some(SourceSpanId::new("tree_before:2:3")),
    },
    RuntimeNodeDeclaration {
        role: RuntimeNodeRole::Element(UiNode {
            node_id: NodeId::new("node.item.removed"),
            kind: ElementKind::Image,
            state_id: None,
        }),
        destruction_policy: DestructionPolicy::PreserveForPool,
        source_span_id: Some(SourceSpanId::new("tree_before:3:3")),
    },
];

const TREE_DIFF_BEFORE_CHILDREN: &[RuntimeChild] = &[
    RuntimeChild {
        tree: TreeChild {
            parent_id: NodeId::new("node.root.old"),
            child_id: NodeId::new("node.item.stable"),
            order: ChildOrder(0),
            key: Some(KeyId::new("key.stable")),
        },
        source_span_id: Some(SourceSpanId::new("tree_before:2:3")),
    },
    RuntimeChild {
        tree: TreeChild {
            parent_id: NodeId::new("node.root.old"),
            child_id: NodeId::new("node.item.removed"),
            order: ChildOrder(1),
            key: None,
        },
        source_span_id: Some(SourceSpanId::new("tree_before:3:3")),
    },
];

const TREE_DIFF_AFTER_NODES: &[RuntimeNodeDeclaration] = &[
    RuntimeNodeDeclaration {
        role: RuntimeNodeRole::Element(UiNode {
            node_id: NodeId::new("node.root.new"),
            kind: ElementKind::View,
            state_id: None,
        }),
        destruction_policy: DestructionPolicy::DestroySubtree,
        source_span_id: Some(SourceSpanId::new("tree_after:1:1")),
    },
    RuntimeNodeDeclaration {
        role: RuntimeNodeRole::Element(UiNode {
            node_id: NodeId::new("node.item.stable"),
            kind: ElementKind::Text,
            state_id: Some(StateId::new("state.item.stable")),
        }),
        destruction_policy: DestructionPolicy::DestroySubtree,
        source_span_id: Some(SourceSpanId::new("tree_after:2:3")),
    },
    RuntimeNodeDeclaration {
        role: RuntimeNodeRole::Fragment(FragmentNode {
            node_id: NodeId::new("node.fragment"),
        }),
        destruction_policy: DestructionPolicy::DestroySubtree,
        source_span_id: Some(SourceSpanId::new("tree_after:3:3")),
    },
    RuntimeNodeDeclaration {
        role: RuntimeNodeRole::Slot(SlotNode {
            node_id: NodeId::new("node.slot"),
            slot_name: "primary",
        }),
        destruction_policy: DestructionPolicy::DestroySubtree,
        source_span_id: Some(SourceSpanId::new("tree_after:4:3")),
    },
    RuntimeNodeDeclaration {
        role: RuntimeNodeRole::Portal(PortalNode {
            node_id: NodeId::new("node.portal"),
            target: "overlay",
        }),
        destruction_policy: DestructionPolicy::DestroySubtree,
        source_span_id: Some(SourceSpanId::new("tree_after:5:3")),
    },
    RuntimeNodeDeclaration {
        role: RuntimeNodeRole::Context(ContextScope {
            node_id: NodeId::new("node.context"),
            state_id: StateId::new("state.context"),
        }),
        destruction_policy: DestructionPolicy::DestroySubtree,
        source_span_id: Some(SourceSpanId::new("tree_after:6:3")),
    },
];

const TREE_DIFF_AFTER_CHILDREN: &[RuntimeChild] = &[
    RuntimeChild {
        tree: TreeChild {
            parent_id: NodeId::new("node.root.new"),
            child_id: NodeId::new("node.item.stable"),
            order: ChildOrder(0),
            key: Some(KeyId::new("key.stable")),
        },
        source_span_id: Some(SourceSpanId::new("tree_after:2:3")),
    },
    RuntimeChild {
        tree: TreeChild {
            parent_id: NodeId::new("node.root.new"),
            child_id: NodeId::new("node.fragment"),
            order: ChildOrder(1),
            key: Some(KeyId::new("key.fragment")),
        },
        source_span_id: Some(SourceSpanId::new("tree_after:3:3")),
    },
    RuntimeChild {
        tree: TreeChild {
            parent_id: NodeId::new("node.fragment"),
            child_id: NodeId::new("node.slot"),
            order: ChildOrder(0),
            key: Some(KeyId::new("key.slot")),
        },
        source_span_id: Some(SourceSpanId::new("tree_after:4:3")),
    },
    RuntimeChild {
        tree: TreeChild {
            parent_id: NodeId::new("node.root.new"),
            child_id: NodeId::new("node.portal"),
            order: ChildOrder(2),
            key: Some(KeyId::new("key.portal")),
        },
        source_span_id: Some(SourceSpanId::new("tree_after:5:3")),
    },
    RuntimeChild {
        tree: TreeChild {
            parent_id: NodeId::new("node.root.new"),
            child_id: NodeId::new("node.context"),
            order: ChildOrder(3),
            key: Some(KeyId::new("key.context")),
        },
        source_span_id: Some(SourceSpanId::new("tree_after:6:3")),
    },
];

const TREE_DIFF_BEFORE: RuntimeDocumentDeclaration = RuntimeDocumentDeclaration {
    fixture_id: TREE_FIXTURE_ID,
    component_id: ComponentId::new("component.runtime.tree"),
    root: RootNode {
        node_id: NodeId::new("node.root.old"),
    },
    nodes: TREE_DIFF_BEFORE_NODES,
    children: TREE_DIFF_BEFORE_CHILDREN,
    source_span_id: Some(SourceSpanId::new("tree_before:1:1")),
};

const TREE_DIFF_AFTER: RuntimeDocumentDeclaration = RuntimeDocumentDeclaration {
    fixture_id: TREE_FIXTURE_ID,
    component_id: ComponentId::new("component.runtime.tree"),
    root: RootNode {
        node_id: NodeId::new("node.root.new"),
    },
    nodes: TREE_DIFF_AFTER_NODES,
    children: TREE_DIFF_AFTER_CHILDREN,
    source_span_id: Some(SourceSpanId::new("tree_after:1:1")),
};

const COMPATIBLE_PATCH_NODES: &[RuntimeNodeDeclaration] = &[
    RuntimeNodeDeclaration {
        role: RuntimeNodeRole::Element(UiNode {
            node_id: NodeId::new("node.patch.root"),
            kind: ElementKind::View,
            state_id: None,
        }),
        destruction_policy: DestructionPolicy::DestroySubtree,
        source_span_id: Some(SourceSpanId::new("patch:1:1")),
    },
    RuntimeNodeDeclaration {
        role: RuntimeNodeRole::Element(UiNode {
            node_id: NodeId::new("node.patch.counter"),
            kind: ElementKind::Control,
            state_id: Some(StateId::new("state.counter")),
        }),
        destruction_policy: DestructionPolicy::DestroySubtree,
        source_span_id: Some(SourceSpanId::new("patch:2:3")),
    },
];

const COMPATIBLE_PATCH_CHILDREN: &[RuntimeChild] = &[RuntimeChild {
    tree: TreeChild {
        parent_id: NodeId::new("node.patch.root"),
        child_id: NodeId::new("node.patch.counter"),
        order: ChildOrder(0),
        key: Some(KeyId::new("key.counter")),
    },
    source_span_id: Some(SourceSpanId::new("patch:2:3")),
}];

const COMPATIBLE_PATCH_BEFORE: RuntimeDocumentDeclaration = RuntimeDocumentDeclaration {
    fixture_id: COMPATIBLE_PATCH_FIXTURE_ID,
    component_id: ComponentId::new("component.patch"),
    root: RootNode {
        node_id: NodeId::new("node.patch.root"),
    },
    nodes: COMPATIBLE_PATCH_NODES,
    children: COMPATIBLE_PATCH_CHILDREN,
    source_span_id: Some(SourceSpanId::new("patch_before:1:1")),
};

const COMPATIBLE_PATCH_AFTER: RuntimeDocumentDeclaration = RuntimeDocumentDeclaration {
    fixture_id: COMPATIBLE_PATCH_FIXTURE_ID,
    component_id: ComponentId::new("component.patch"),
    root: RootNode {
        node_id: NodeId::new("node.patch.root"),
    },
    nodes: COMPATIBLE_PATCH_NODES,
    children: COMPATIBLE_PATCH_CHILDREN,
    source_span_id: Some(SourceSpanId::new("patch_after:1:1")),
};

const INCOMPATIBLE_PATCH_BEFORE: RuntimeDocumentDeclaration = RuntimeDocumentDeclaration {
    fixture_id: INCOMPATIBLE_PATCH_FIXTURE_ID,
    component_id: ComponentId::new("component.patch.before"),
    root: RootNode {
        node_id: NodeId::new("node.patch.root"),
    },
    nodes: COMPATIBLE_PATCH_NODES,
    children: COMPATIBLE_PATCH_CHILDREN,
    source_span_id: Some(SourceSpanId::new("incompatible_before:1:1")),
};

const INCOMPATIBLE_PATCH_AFTER: RuntimeDocumentDeclaration = RuntimeDocumentDeclaration {
    fixture_id: INCOMPATIBLE_PATCH_FIXTURE_ID,
    component_id: ComponentId::new("component.patch.after"),
    root: RootNode {
        node_id: NodeId::new("node.patch.root"),
    },
    nodes: COMPATIBLE_PATCH_NODES,
    children: COMPATIBLE_PATCH_CHILDREN,
    source_span_id: Some(SourceSpanId::new("incompatible_after:9:5")),
};
