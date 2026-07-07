use valdi_rust_backend::{
    operations::{
        DestroyOperation, MoveOperation, RootOperation, TransactionGroupOperation, TransactionPhase,
    },
    BackendOperation, RenderBackend,
};
use valdi_rust_ir::animations::TransactionGroup;

use crate::{
    diagnostics::{RuntimeDiagnostic, RuntimeResult},
    document::RuntimeDocument,
};

pub fn diff_documents(
    before: &RuntimeDocument,
    after: &RuntimeDocument,
) -> RuntimeResult<Vec<BackendOperation>> {
    let mut operations = vec![BackendOperation::TransactionGroup(
        TransactionGroupOperation {
            group: TransactionGroup {
                name: "runtime_tree_diff",
            },
            phase: TransactionPhase::Begin,
        },
    )];

    for node in &after.nodes {
        if before.node(node.node_id()).is_none() {
            operations.push(BackendOperation::Create(
                valdi_rust_backend::operations::CreateOperation {
                    identity: node.identity(after.component_id),
                },
            ));
        }
    }

    if before.root.node_id != after.root.node_id {
        operations.push(BackendOperation::Root(RootOperation {
            node_id: after.root.node_id,
        }));
    }

    for child in &after.children {
        if before
            .child_for(child.tree.child_id)
            .map(|previous| previous.tree == child.tree)
            .unwrap_or(false)
        {
            continue;
        }
        operations.push(BackendOperation::Move(MoveOperation {
            parent_id: child.tree.parent_id,
            child_id: child.tree.child_id,
            order: child.tree.order,
            key: child.tree.key,
        }));
    }

    for node in before.nodes.iter().rev() {
        if after.node(node.node_id()).is_none() {
            operations.push(BackendOperation::Destroy(DestroyOperation {
                node_id: node.node_id(),
                policy: node.destruction_policy,
            }));
        }
    }

    operations.push(BackendOperation::TransactionGroup(
        TransactionGroupOperation {
            group: TransactionGroup {
                name: "runtime_tree_diff",
            },
            phase: TransactionPhase::Commit,
        },
    ));

    Ok(operations)
}

pub fn apply_diff_to_backend<B: RenderBackend>(
    before: &RuntimeDocument,
    after: &RuntimeDocument,
    backend: &mut B,
) -> RuntimeResult<Vec<BackendOperation>> {
    let operations = diff_documents(before, after)?;
    for operation in &operations {
        backend.apply(*operation).map_err(|diagnostic| {
            RuntimeDiagnostic::error(diagnostic.code, diagnostic.path, diagnostic.message, None)
        })?;
    }
    Ok(operations)
}
