use valdi_rust_hot_patch::{
    sample_action_implementation, sample_detected_action_body_patch,
    validate_hot_patch_trace_snapshot, DevActionImplementationLoader,
};
use valdi_rust_ir::ids::ActionId;

#[test]
fn live_action_body_patch_replaces_loaded_action_implementation() {
    let action_id = ActionId::new("action.save");
    let patch = sample_detected_action_body_patch().unwrap();
    let mut loader = DevActionImplementationLoader::new_debug(vec![sample_action_implementation()]);

    assert_eq!(loader.dispatch_token(action_id).unwrap(), "body.save.v1");
    let record = loader.apply_action_body_patch(patch).unwrap();
    assert_eq!(record.previous_body_token, "body.save.v1");
    assert_eq!(record.active_body_token, "business_logic");
    assert_eq!(record.generation, 1);
    assert_eq!(loader.dispatch_token(action_id).unwrap(), "business_logic");
}

#[test]
fn hot_patch_trace_snapshot_is_stable_and_drift_checked() {
    let expected = include_str!("../snapshots/hot_patch_trace.snap");
    validate_hot_patch_trace_snapshot(&expected).unwrap();

    let drift = expected.replace("after_dispatch=business_logic", "after_dispatch=stale");
    let diagnostic = validate_hot_patch_trace_snapshot(&drift).unwrap_err();
    assert_eq!(diagnostic.code, "HOT_PATCH_TRACE_DRIFT");
    assert_eq!(diagnostic.path, "$.rust_hot_patch.trace");
}
