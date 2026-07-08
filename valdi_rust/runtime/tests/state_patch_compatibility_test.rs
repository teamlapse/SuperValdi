use valdi_rust_ir::ids::StateId;
use valdi_rust_runtime::{
    apply_identity_patch, compatible_patch_pair, retain_compatible_state, StateEntry, StateStore,
    StateValue, RUNTIME_STATE_IDENTITY_INCOMPATIBLE,
};

#[test]
fn compatible_runtime_patch_retains_state_slots() {
    let store = StateStore::new(vec![StateEntry::new(
        StateId::new("state.counter"),
        StateValue::I64(7),
        None,
    )])
    .expect("state store loads");
    let (before, after) = compatible_patch_pair().expect("patch fixtures load");
    let patch = apply_identity_patch(&before, &after).expect("compatible patch applies");

    let retained = retain_compatible_state(&store, &patch, &[StateId::new("state.counter")])
        .expect("compatible state is retained");
    assert_eq!(
        retained
            .value(StateId::new("state.counter"))
            .expect("retained state")
            .as_i64(),
        Some(7)
    );
}

#[test]
fn incompatible_state_identity_returns_exact_diagnostic() {
    let store = StateStore::new(vec![StateEntry::new(
        StateId::new("state.counter"),
        StateValue::I64(7),
        None,
    )])
    .expect("state store loads");
    let (before, after) = compatible_patch_pair().expect("patch fixtures load");
    let patch = apply_identity_patch(&before, &after).expect("compatible patch applies");

    let diagnostic = retain_compatible_state(&store, &patch, &[StateId::new("state.missing")])
        .expect_err("missing preserved state identity must fail");
    assert_eq!(diagnostic.code, RUNTIME_STATE_IDENTITY_INCOMPATIBLE);
    assert_eq!(diagnostic.path, "$.identity_patch.preserved_state_slots");
    assert_eq!(diagnostic.severity.as_str(), "error");
}
