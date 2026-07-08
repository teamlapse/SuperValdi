use valdi_rust_hot_patch::{
    detect_action_body_hot_patch, detect_hot_reload_action_body_patch,
    sample_action_body_after_fingerprint, sample_action_body_before_fingerprint,
};
use valdi_rust_hot_reload::{parse_declarative_patch, ACTION_BODY_PATCH_INPUT};

#[test]
fn detects_declared_action_body_edit_as_live_patch() {
    let patch = detect_action_body_hot_patch(
        sample_action_body_before_fingerprint(),
        sample_action_body_after_fingerprint(),
    )
    .expect("sample action-body edit should be live-patchable");

    assert_eq!(patch.action_id.as_str(), "action.save");
    assert_eq!(patch.previous_body_token, "body.save.v1");
    assert_eq!(patch.next_body_token, "business_logic");
    assert_eq!(
        patch.source_span_id.map(|span| span.as_str()),
        Some("hot_patch_fixture:1:1")
    );
}

#[test]
fn detects_pr11_action_body_intent_without_ui_rebuild() {
    let intent = parse_declarative_patch(ACTION_BODY_PATCH_INPUT).unwrap();
    let patch =
        detect_hot_reload_action_body_patch(&intent, sample_action_body_before_fingerprint())
            .expect("PR11 action-body intent should feed PR12 live patch");

    assert_eq!(patch.action_id.as_str(), "action.save");
    assert_eq!(patch.next_body_token, "business_logic");
    assert_eq!(
        patch.source_span_id.map(|span| span.as_str()),
        Some("hot_reload.dsl:11:1")
    );
}
