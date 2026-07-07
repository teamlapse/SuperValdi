use valdi_rust_ir::ids::{KeyId, StateId};
use valdi_rust_runtime::{static_tree_diff_pair, IdentityTables, RuntimeNodeRole};

#[test]
fn typed_runtime_documents_load_from_static_fixtures() {
    let (before, after) = static_tree_diff_pair().expect("typed runtime fixtures load");

    assert_eq!(before.fixture_id, "runtime.tree_diff.v1");
    assert_eq!(before.root.node_id.as_str(), "node.root.old");
    assert_eq!(after.root.node_id.as_str(), "node.root.new");
    assert!(after
        .nodes
        .iter()
        .any(|node| matches!(node.role, RuntimeNodeRole::Fragment(_))));
    assert!(after
        .nodes
        .iter()
        .any(|node| matches!(node.role, RuntimeNodeRole::Slot(_))));
    assert!(after
        .nodes
        .iter()
        .any(|node| matches!(node.role, RuntimeNodeRole::Portal(_))));
    assert!(after
        .nodes
        .iter()
        .any(|node| matches!(node.role, RuntimeNodeRole::Context(_))));

    let identity = IdentityTables::from_document(&after);
    assert!(identity
        .state_slot(StateId::new("state.item.stable"))
        .is_some());
    assert!(identity
        .keys
        .iter()
        .any(|key| key.key_id == KeyId::new("key.stable")));
}
