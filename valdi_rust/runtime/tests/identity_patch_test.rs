use valdi_rust_ir::ids::StateId;
use valdi_rust_runtime::{
    apply_identity_patch, compatible_patch_pair, validate_preserved_state_slots,
};

#[test]
fn compatible_identity_patch_preserves_state_slots() {
    let (before, after) = compatible_patch_pair().expect("compatible patch fixtures load");
    let result = apply_identity_patch(&before, &after).expect("compatible patch applies");

    validate_preserved_state_slots(&result, &[StateId::new("state.counter")])
        .expect("state slot is preserved");
    assert_eq!(
        result.preserved_state_slots[0].node_id.as_str(),
        "node.patch.counter"
    );
}

#[test]
fn missing_state_preservation_mapping_fails() {
    let (before, after) = compatible_patch_pair().expect("compatible patch fixtures load");
    let result = apply_identity_patch(&before, &after).expect("compatible patch applies");

    let diagnostic = validate_preserved_state_slots(
        &result,
        &[
            StateId::new("state.counter"),
            StateId::new("state.counter.missing"),
        ],
    )
    .expect_err("missing expected state slot must fail");

    assert_eq!(diagnostic.code, "RUNTIME_STATE_SLOT_DRIFT");
    assert_eq!(diagnostic.path, "$.identity_patch.preserved_state_slots");
    assert_eq!(diagnostic.severity.as_str(), "error");
}
