use valdi_rust_ir::ids::StateId;
use valdi_rust_runtime::{
    sample_state_store, StateValue, StateValueKind, RUNTIME_STATE_MISSING,
    RUNTIME_STATE_TYPE_MISMATCH,
};

#[test]
fn state_store_gets_sets_updates_and_invalidates_typed_values() {
    let mut store = sample_state_store().expect("state fixture loads");
    assert_eq!(
        store
            .expect_kind(StateId::new("app"), StateValueKind::Record)
            .expect("app state is a record")
            .kind(),
        StateValueKind::Record
    );

    store.set(StateId::new("draft"), StateValue::Text("hello"), None);
    assert_eq!(
        store
            .value(StateId::new("draft"))
            .expect("draft state")
            .as_text(),
        Some("hello")
    );

    store
        .update(
            StateId::new("draft"),
            StateValue::Text("world"),
            "test_update",
        )
        .expect("update invalidates state");
    assert_eq!(
        store
            .value(StateId::new("draft"))
            .expect("updated draft")
            .as_text(),
        Some("world")
    );
    assert_eq!(store.invalidations()[0].state_id.as_str(), "draft");
    assert_eq!(store.invalidations()[0].reason, "test_update");

    let invalidation = store
        .invalidate(StateId::new("app"), "manual")
        .expect("manual invalidation");
    assert_eq!(invalidation.state_id.as_str(), "app");
    assert_eq!(store.invalidations().len(), 2);
}

#[test]
fn state_store_rejects_missing_and_wrong_typed_values() {
    let store = sample_state_store().expect("state fixture loads");

    let missing = store
        .value(StateId::new("missing"))
        .expect_err("missing state must fail");
    assert_eq!(missing.code, RUNTIME_STATE_MISSING);
    assert_eq!(missing.path, "$.state");
    assert_eq!(missing.severity.as_str(), "error");

    let wrong_type = store
        .expect_kind(StateId::new("app"), StateValueKind::Bool)
        .expect_err("wrong state type must fail");
    assert_eq!(wrong_type.code, RUNTIME_STATE_TYPE_MISMATCH);
    assert_eq!(wrong_type.path, "$.state.value");
    assert_eq!(wrong_type.severity.as_str(), "error");
}
