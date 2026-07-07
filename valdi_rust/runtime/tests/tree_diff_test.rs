use valdi_rust_backend::{
    BackendCapabilitySet, BackendOperationFamily, BackendTarget, MockBackend,
};
use valdi_rust_ir::tree::DestructionPolicy;
use valdi_rust_runtime::{apply_diff_to_backend, diff_documents, static_tree_diff_pair};

#[test]
fn tree_diff_emits_ordered_backend_ops_for_tree_identity_changes() {
    let (before, after) = static_tree_diff_pair().expect("fixtures load");
    let operations = diff_documents(&before, &after).expect("diff emits operations");
    let families = operations
        .iter()
        .map(|operation| operation.family())
        .collect::<Vec<_>>();

    assert_eq!(families[0], BackendOperationFamily::TransactionGroup);
    assert_eq!(families[1], BackendOperationFamily::Create);
    assert!(families.contains(&BackendOperationFamily::Root));
    assert!(families.contains(&BackendOperationFamily::Move));
    assert_eq!(
        families[families.len() - 2],
        BackendOperationFamily::Destroy
    );
    assert_eq!(
        families[families.len() - 1],
        BackendOperationFamily::TransactionGroup
    );

    let keyed_move = operations
        .iter()
        .find_map(|operation| match operation {
            valdi_rust_backend::BackendOperation::Move(move_op)
                if move_op.child_id.as_str() == "node.item.stable" =>
            {
                Some(move_op)
            }
            _ => None,
        })
        .expect("keyed stable node moves");
    assert_eq!(keyed_move.parent_id.as_str(), "node.root.new");
    assert_eq!(keyed_move.key.expect("keyed move").as_str(), "key.stable");

    let pooled_destroy = operations
        .iter()
        .find_map(|operation| match operation {
            valdi_rust_backend::BackendOperation::Destroy(destroy)
                if destroy.node_id.as_str() == "node.item.removed" =>
            {
                Some(destroy)
            }
            _ => None,
        })
        .expect("removed node destroy op");
    assert_eq!(pooled_destroy.policy, DestructionPolicy::PreserveForPool);

    let mut backend = MockBackend::new(BackendCapabilitySet::all_for(BackendTarget::RustHost));
    let applied =
        apply_diff_to_backend(&before, &after, &mut backend).expect("mock backend accepts diff");
    assert_eq!(applied, operations);
    assert_eq!(backend.operations().len(), 15);
}
