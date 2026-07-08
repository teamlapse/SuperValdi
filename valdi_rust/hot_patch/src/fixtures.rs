use valdi_rust_hot_reload::{parse_declarative_patch, ACTION_BODY_PATCH_INPUT};
use valdi_rust_ir::{
    actions::{ActionDefinition, ActionKind},
    ids::{ActionId, SourceSpanId, StateId},
};
use valdi_rust_runtime::{RuntimeActionBehavior, RuntimeActionDefinition};

use crate::{
    detector::{
        detect_action_body_hot_patch, detect_hot_reload_action_body_patch, ActionBodyFingerprint,
        RustEditClass,
    },
    dev_server::HotPatchSession,
    diagnostics::{
        HotPatchDiagnostic, HotPatchResult, HOT_PATCH_SUPPORT_MATRIX_DRIFT, HOT_PATCH_TRACE_DRIFT,
    },
    loader::{ActionImplementation, DevActionImplementationLoader},
    release_guard::{release_build_report, validate_release_exclusion},
    support_matrix::support_matrix_report,
};

pub const HOT_PATCH_SAMPLE_SESSION_ID: &str = "hot_patch.session.v1";
pub const HOT_PATCH_TRACE_FIXTURE_ID: &str = "hot_patch.trace.v1";

pub fn sample_action_definition() -> RuntimeActionDefinition {
    RuntimeActionDefinition {
        definition: ActionDefinition {
            id: ActionId::new("action.save"),
            kind: ActionKind::Sync,
            state_scope: StateId::new("app"),
        },
        cancellation_policy: valdi_rust_ir::actions::CancellationPolicy::NotCancellable,
        scheduling_policy: valdi_rust_ir::actions::SchedulingPolicy::Immediate,
        coalescing_key: None,
        invalidation_targets: vec![],
        behavior: RuntimeActionBehavior::Complete,
        source_span_id: Some(SourceSpanId::new("hot_patch_fixture:1:1")),
    }
}

pub fn sample_action_implementation() -> ActionImplementation {
    ActionImplementation::new(ActionId::new("action.save"), "body.save.v1", "body.save.v1")
}

pub fn sample_action_body_before_fingerprint() -> ActionBodyFingerprint {
    ActionBodyFingerprint {
        action_id: sample_action_definition().definition.id,
        signature: "fn save(model: SaveModel) -> ActionResult",
        type_contract: "action.save.types.v1",
        module_contract: "module.actions.v1",
        state_shape: "state.app.v1",
        dependency_set: "deps.validation.v1",
        macro_expansion: "macro.action_dispatch.v1",
        crate_graph: "crate_graph.hot_patch.v1",
        platform_boundary: "platform_boundary.actions.v1",
        body_token: "body.save.v1",
        source_span_id: Some(SourceSpanId::new("hot_patch_fixture:1:1")),
    }
}

pub fn sample_action_body_after_fingerprint() -> ActionBodyFingerprint {
    sample_action_body_before_fingerprint().with_body_token("business_logic")
}

pub fn sample_unsupported_after_fingerprint(edit_class: RustEditClass) -> ActionBodyFingerprint {
    sample_action_body_before_fingerprint().with_edit_class_drift(edit_class)
}

pub fn hot_patch_trace_snapshot() -> HotPatchResult<String> {
    let before = sample_action_body_before_fingerprint();
    let intent = parse_declarative_patch(ACTION_BODY_PATCH_INPUT).map_err(|diagnostic| {
        HotPatchDiagnostic::rebuild_required(
            diagnostic.code,
            diagnostic.path,
            diagnostic.reason,
            diagnostic.message,
            diagnostic.source_span_id,
        )
    })?;
    let patch = detect_hot_reload_action_body_patch(&intent, before)?;
    let mut loader = DevActionImplementationLoader::new_debug(vec![sample_action_implementation()]);
    let mut session = HotPatchSession::new(HOT_PATCH_SAMPLE_SESSION_ID);
    let before_dispatch = loader.dispatch_token(patch.action_id)?;
    let record = loader.apply_action_body_patch(patch)?;
    let message = session.publish_applied(record);
    let after_dispatch = loader.dispatch_token(patch.action_id)?;
    let release = release_build_report();
    validate_release_exclusion(release)?;
    Ok(format!(
        "hot_patch_trace_v1\nfixture={}\nsession={}\naction={}\nbefore_dispatch={}\n{}\nafter_dispatch={}\nrelease_hot_patch_machinery_present={}\nrelease_dev_loader_symbols={}\n",
        HOT_PATCH_TRACE_FIXTURE_ID,
        HOT_PATCH_SAMPLE_SESSION_ID,
        patch.action_id.as_str(),
        before_dispatch,
        message.stable_line(),
        after_dispatch,
        release.hot_patch_machinery_present,
        release.dev_loader_symbols.len()
    ))
}

pub fn validate_hot_patch_trace_snapshot(expected: &str) -> HotPatchResult<()> {
    let actual = hot_patch_trace_snapshot()?;
    if actual == expected {
        return Ok(());
    }
    Err(HotPatchDiagnostic::error(
        HOT_PATCH_TRACE_DRIFT,
        "$.rust_hot_patch.trace",
        "hot_patch_trace_drift",
        "hot patch trace snapshot drift",
        None,
    ))
}

pub fn hot_patch_support_matrix_snapshot() -> String {
    support_matrix_report()
}

pub fn validate_hot_patch_support_matrix_snapshot(expected: &str) -> HotPatchResult<()> {
    let actual = hot_patch_support_matrix_snapshot();
    if actual == expected {
        return Ok(());
    }
    Err(HotPatchDiagnostic::error(
        HOT_PATCH_SUPPORT_MATRIX_DRIFT,
        "$.rust_hot_patch.support_matrix",
        "support_matrix_drift",
        "hot patch support matrix snapshot drift",
        None,
    ))
}

pub fn sample_detected_action_body_patch() -> HotPatchResult<crate::ActionBodyHotPatch> {
    detect_action_body_hot_patch(
        sample_action_body_before_fingerprint(),
        sample_action_body_after_fingerprint(),
    )
}
