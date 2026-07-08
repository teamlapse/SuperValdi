use crate::{
    detector::{RustEditClass, REQUIRED_UNSUPPORTED_EDIT_CLASSES},
    diagnostics::{HotPatchDiagnostic, HotPatchResult, HOT_PATCH_SUPPORT_MATRIX_DRIFT},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HotPatchSupportStatus {
    LivePatch,
    RebuildRequired,
}

impl HotPatchSupportStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LivePatch => "live_patch",
            Self::RebuildRequired => "rebuild_required",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HotPatchSupportDecision {
    pub edit_class: RustEditClass,
    pub status: HotPatchSupportStatus,
    pub diagnostic_code: &'static str,
    pub reason: &'static str,
}

pub const SUPPORT_MATRIX: &[HotPatchSupportDecision] = &[
    HotPatchSupportDecision {
        edit_class: RustEditClass::ActionBody,
        status: HotPatchSupportStatus::LivePatch,
        diagnostic_code: crate::diagnostics::HOT_PATCH_ACTION_BODY_SUPPORTED,
        reason: "action_body_changed",
    },
    HotPatchSupportDecision {
        edit_class: RustEditClass::Signature,
        status: HotPatchSupportStatus::RebuildRequired,
        diagnostic_code: crate::diagnostics::HOT_PATCH_SIGNATURE_REBUILD_REQUIRED,
        reason: "action_signature_changed",
    },
    HotPatchSupportDecision {
        edit_class: RustEditClass::Type,
        status: HotPatchSupportStatus::RebuildRequired,
        diagnostic_code: crate::diagnostics::HOT_PATCH_TYPE_REBUILD_REQUIRED,
        reason: "action_type_contract_changed",
    },
    HotPatchSupportDecision {
        edit_class: RustEditClass::Module,
        status: HotPatchSupportStatus::RebuildRequired,
        diagnostic_code: crate::diagnostics::HOT_PATCH_MODULE_REBUILD_REQUIRED,
        reason: "module_contract_changed",
    },
    HotPatchSupportDecision {
        edit_class: RustEditClass::StateShape,
        status: HotPatchSupportStatus::RebuildRequired,
        diagnostic_code: crate::diagnostics::HOT_PATCH_STATE_SHAPE_REBUILD_REQUIRED,
        reason: "state_shape_changed",
    },
    HotPatchSupportDecision {
        edit_class: RustEditClass::Dependency,
        status: HotPatchSupportStatus::RebuildRequired,
        diagnostic_code: crate::diagnostics::HOT_PATCH_DEPENDENCY_REBUILD_REQUIRED,
        reason: "dependency_set_changed",
    },
    HotPatchSupportDecision {
        edit_class: RustEditClass::Macro,
        status: HotPatchSupportStatus::RebuildRequired,
        diagnostic_code: crate::diagnostics::HOT_PATCH_MACRO_REBUILD_REQUIRED,
        reason: "macro_expansion_changed",
    },
    HotPatchSupportDecision {
        edit_class: RustEditClass::CrateGraph,
        status: HotPatchSupportStatus::RebuildRequired,
        diagnostic_code: crate::diagnostics::HOT_PATCH_CRATE_GRAPH_REBUILD_REQUIRED,
        reason: "crate_graph_changed",
    },
    HotPatchSupportDecision {
        edit_class: RustEditClass::PlatformBoundary,
        status: HotPatchSupportStatus::RebuildRequired,
        diagnostic_code: crate::diagnostics::HOT_PATCH_PLATFORM_BOUNDARY_REBUILD_REQUIRED,
        reason: "platform_boundary_changed",
    },
];

pub fn support_matrix() -> &'static [HotPatchSupportDecision] {
    SUPPORT_MATRIX
}

pub fn support_decision(edit_class: RustEditClass) -> HotPatchSupportDecision {
    SUPPORT_MATRIX
        .iter()
        .copied()
        .find(|decision| decision.edit_class == edit_class)
        .expect("all Rust edit classes have support decisions")
}

pub fn validate_support_matrix() -> HotPatchResult<()> {
    let supported = SUPPORT_MATRIX
        .iter()
        .filter(|decision| decision.status == HotPatchSupportStatus::LivePatch)
        .map(|decision| decision.edit_class)
        .collect::<Vec<_>>();
    if supported.as_slice() != [RustEditClass::ActionBody] {
        return Err(matrix_drift("only action_body may be live-patched"));
    }

    for required in REQUIRED_UNSUPPORTED_EDIT_CLASSES {
        let decision = support_decision(*required);
        if decision.status != HotPatchSupportStatus::RebuildRequired
            || decision.diagnostic_code != required.diagnostic_code()
            || decision.reason != required.rebuild_reason()
        {
            return Err(matrix_drift("unsupported edit class decision drift"));
        }
    }

    Ok(())
}

pub fn support_matrix_report() -> String {
    let mut lines = vec!["hot_patch_support_matrix_v1".to_string()];
    lines.extend(SUPPORT_MATRIX.iter().map(|decision| {
        format!(
            "{}|{}|{}|{}",
            decision.edit_class.as_str(),
            decision.status.as_str(),
            decision.diagnostic_code,
            decision.reason
        )
    }));
    format!("{}\n", lines.join("\n"))
}

fn matrix_drift(message: &'static str) -> HotPatchDiagnostic {
    HotPatchDiagnostic::error(
        HOT_PATCH_SUPPORT_MATRIX_DRIFT,
        "$.rust_hot_patch.support_matrix",
        "support_matrix_drift",
        message,
        None,
    )
}
