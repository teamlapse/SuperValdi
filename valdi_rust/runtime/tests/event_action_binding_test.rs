use valdi_rust_ir::{
    events::{EventBinding, EventKind},
    ids::{ActionId, NodeId},
};
use valdi_rust_runtime::{
    sample_action_scheduler, sample_event_bindings, EventActionBindings,
    RUNTIME_EVENT_ACTION_MISSING, RUNTIME_EVENT_BINDING_MISSING,
};

#[test]
fn event_action_binding_resolves_typed_event_to_action() {
    let scheduler = sample_action_scheduler().expect("action scheduler loads");
    let bindings = sample_event_bindings();

    let action = bindings
        .resolve(NodeId::new("node.button"), EventKind::Tap, &scheduler)
        .expect("tap resolves to action");
    assert_eq!(action.as_str(), "action.save");
}

#[test]
fn event_action_binding_reports_missing_action_and_mapping() {
    let scheduler = sample_action_scheduler().expect("action scheduler loads");
    let missing_action_bindings = EventActionBindings::new(vec![EventBinding {
        node_id: NodeId::new("node.button"),
        kind: EventKind::Tap,
        action_id: ActionId::new("action.missing"),
    }]);
    let missing_action = missing_action_bindings
        .resolve(NodeId::new("node.button"), EventKind::Tap, &scheduler)
        .expect_err("missing action must fail");
    assert_eq!(missing_action.code, RUNTIME_EVENT_ACTION_MISSING);
    assert_eq!(missing_action.path, "$.event_bindings.action_id");
    assert_eq!(missing_action.severity.as_str(), "error");

    let missing_mapping = sample_event_bindings()
        .resolve(NodeId::new("node.other"), EventKind::Tap, &scheduler)
        .expect_err("missing event mapping must fail");
    assert_eq!(missing_mapping.code, RUNTIME_EVENT_BINDING_MISSING);
    assert_eq!(missing_mapping.path, "$.event_bindings");
    assert_eq!(missing_mapping.severity.as_str(), "error");
}
