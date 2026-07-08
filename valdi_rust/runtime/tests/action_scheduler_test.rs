use valdi_rust_ir::{
    actions::{ActionError, ActionResult, SchedulingPolicy},
    ids::ActionId,
};
use valdi_rust_runtime::{
    sample_action_scheduler, sample_state_store, ActionDispatchStatus,
    RUNTIME_ACTION_CANCELLED_REUSE,
};

#[test]
fn action_scheduler_runs_sync_and_deterministic_async_actions() {
    let mut store = sample_state_store().expect("state fixture loads");
    let mut scheduler = sample_action_scheduler().expect("action scheduler loads");

    let save = scheduler
        .dispatch(ActionId::new("action.save"), &mut store)
        .expect("sync action dispatches");
    assert_eq!(save.status, ActionDispatchStatus::Completed);
    assert_eq!(save.result, Some(ActionResult::Completed));
    assert_eq!(save.invalidations[0].state_id.as_str(), "app");
    assert_eq!(save.scheduling_policy, SchedulingPolicy::Immediate);

    let load = scheduler
        .dispatch(ActionId::new("action.load"), &mut store)
        .expect("async action schedules deterministically");
    assert_eq!(load.status, ActionDispatchStatus::Scheduled);
    assert_eq!(
        load.coalescing_key.expect("coalescing key").value,
        "load.user"
    );
    assert_eq!(load.scheduling_policy, SchedulingPolicy::Deferred);

    let complete = scheduler
        .complete_async(ActionId::new("action.load"), &mut store)
        .expect("async completion is deterministic");
    assert_eq!(complete.status, ActionDispatchStatus::Completed);
    assert_eq!(complete.result, Some(ActionResult::Completed));
    assert_eq!(complete.invalidations[0].state_id.as_str(), "app");
}

#[test]
fn action_scheduler_reports_typed_error_and_coalescing() {
    let mut store = sample_state_store().expect("state fixture loads");
    let mut scheduler = sample_action_scheduler().expect("action scheduler loads");

    let failed = scheduler
        .dispatch(ActionId::new("action.fail"), &mut store)
        .expect("typed error action dispatches");
    let expected_error = ActionError {
        code: "network.unavailable",
    };
    assert_eq!(failed.status, ActionDispatchStatus::Failed(expected_error));
    assert_eq!(failed.result, Some(ActionResult::Failed(expected_error)));

    scheduler
        .dispatch(ActionId::new("action.load"), &mut store)
        .expect("first load schedules");
    let coalesced = scheduler
        .dispatch(ActionId::new("action.load"), &mut store)
        .expect("second load coalesces");
    assert_eq!(coalesced.status, ActionDispatchStatus::Coalesced);
    assert_eq!(
        coalesced.coalescing_key.expect("coalescing key").value,
        "load.user"
    );
}

#[test]
fn action_scheduler_rejects_canceled_action_reuse() {
    let mut store = sample_state_store().expect("state fixture loads");
    let mut scheduler = sample_action_scheduler().expect("action scheduler loads");

    scheduler
        .dispatch(ActionId::new("action.load"), &mut store)
        .expect("load schedules");
    let cancelled = scheduler
        .cancel(ActionId::new("action.load"))
        .expect("load cancels by action id");
    assert_eq!(cancelled.status, ActionDispatchStatus::Cancelled);
    assert_eq!(cancelled.result, Some(ActionResult::Cancelled));

    let diagnostic = scheduler
        .complete_async(ActionId::new("action.load"), &mut store)
        .expect_err("canceled action completion must fail");
    assert_eq!(diagnostic.code, RUNTIME_ACTION_CANCELLED_REUSE);
    assert_eq!(diagnostic.path, "$.actions.cancellation_identity");
    assert_eq!(diagnostic.severity.as_str(), "error");
}
