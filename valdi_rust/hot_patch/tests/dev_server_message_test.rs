use valdi_rust_hot_patch::{
    sample_detected_action_body_patch, support_decision, DevActionImplementationLoader,
    HotPatchDevServerMessage, HotPatchSession, RustEditClass,
};

#[test]
fn dev_server_success_message_is_stable() {
    let patch = sample_detected_action_body_patch().unwrap();
    let mut loader = DevActionImplementationLoader::new_debug(vec![
        valdi_rust_hot_patch::sample_action_implementation(),
    ]);
    let record = loader.apply_action_body_patch(patch).unwrap();
    let message = HotPatchDevServerMessage::applied("hot_patch.session.v1", record);

    assert_eq!(
        message.stable_line(),
        "session=hot_patch.session.v1 kind=applied action=action.save code=HOT_PATCH_ACTION_BODY_SUPPORTED path=$.rust_hot_patch.action_body severity=info reason=action_body_changed message=action action.save patched generation 1 body.save.v1->business_logic"
    );
}

#[test]
fn dev_server_rebuild_required_message_is_exact() {
    let decision = support_decision(RustEditClass::Signature);
    let diagnostic = valdi_rust_hot_patch::HotPatchDiagnostic::rebuild_required(
        decision.diagnostic_code,
        RustEditClass::Signature.path(),
        decision.reason,
        "signature changed",
        None,
    );
    let mut session = HotPatchSession::new("hot_patch.session.v1");
    let message = session.publish_rebuild_required(diagnostic);

    assert_eq!(
        message.stable_line(),
        "session=hot_patch.session.v1 kind=rebuild_required action=none code=HOT_PATCH_SIGNATURE_REBUILD_REQUIRED path=$.rust_hot_patch.signature severity=error reason=action_signature_changed message=signature changed"
    );
    assert_eq!(session.messages(), &[message]);
}
