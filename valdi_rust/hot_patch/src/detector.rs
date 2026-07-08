use valdi_rust_hot_reload::{ActionBodyPatch, HotReloadEdit, HotReloadPatchIntent};
use valdi_rust_ir::ids::{ActionId, SourceSpanId};

use crate::{
    diagnostics::{
        HotPatchDiagnostic, HotPatchResult, HOT_PATCH_CRATE_GRAPH_REBUILD_REQUIRED,
        HOT_PATCH_DEPENDENCY_REBUILD_REQUIRED, HOT_PATCH_MACRO_REBUILD_REQUIRED,
        HOT_PATCH_MODULE_REBUILD_REQUIRED, HOT_PATCH_NO_ACTION_BODY_CHANGE,
        HOT_PATCH_PLATFORM_BOUNDARY_REBUILD_REQUIRED, HOT_PATCH_SIGNATURE_REBUILD_REQUIRED,
        HOT_PATCH_STATE_SHAPE_REBUILD_REQUIRED, HOT_PATCH_TYPE_REBUILD_REQUIRED,
    },
    support_matrix::support_decision,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RustEditClass {
    ActionBody,
    Signature,
    Type,
    Module,
    StateShape,
    Dependency,
    Macro,
    CrateGraph,
    PlatformBoundary,
}

impl RustEditClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActionBody => "action_body",
            Self::Signature => "signature",
            Self::Type => "type",
            Self::Module => "module",
            Self::StateShape => "state_shape",
            Self::Dependency => "dependency",
            Self::Macro => "macro",
            Self::CrateGraph => "crate_graph",
            Self::PlatformBoundary => "platform_boundary",
        }
    }

    pub const fn path(self) -> &'static str {
        match self {
            Self::ActionBody => "$.rust_hot_patch.action_body",
            Self::Signature => "$.rust_hot_patch.signature",
            Self::Type => "$.rust_hot_patch.type",
            Self::Module => "$.rust_hot_patch.module",
            Self::StateShape => "$.rust_hot_patch.state_shape",
            Self::Dependency => "$.rust_hot_patch.dependency",
            Self::Macro => "$.rust_hot_patch.macro",
            Self::CrateGraph => "$.rust_hot_patch.crate_graph",
            Self::PlatformBoundary => "$.rust_hot_patch.platform_boundary",
        }
    }

    pub const fn diagnostic_code(self) -> &'static str {
        match self {
            Self::ActionBody => crate::diagnostics::HOT_PATCH_ACTION_BODY_SUPPORTED,
            Self::Signature => HOT_PATCH_SIGNATURE_REBUILD_REQUIRED,
            Self::Type => HOT_PATCH_TYPE_REBUILD_REQUIRED,
            Self::Module => HOT_PATCH_MODULE_REBUILD_REQUIRED,
            Self::StateShape => HOT_PATCH_STATE_SHAPE_REBUILD_REQUIRED,
            Self::Dependency => HOT_PATCH_DEPENDENCY_REBUILD_REQUIRED,
            Self::Macro => HOT_PATCH_MACRO_REBUILD_REQUIRED,
            Self::CrateGraph => HOT_PATCH_CRATE_GRAPH_REBUILD_REQUIRED,
            Self::PlatformBoundary => HOT_PATCH_PLATFORM_BOUNDARY_REBUILD_REQUIRED,
        }
    }

    pub const fn rebuild_reason(self) -> &'static str {
        match self {
            Self::ActionBody => "action_body_changed",
            Self::Signature => "action_signature_changed",
            Self::Type => "action_type_contract_changed",
            Self::Module => "module_contract_changed",
            Self::StateShape => "state_shape_changed",
            Self::Dependency => "dependency_set_changed",
            Self::Macro => "macro_expansion_changed",
            Self::CrateGraph => "crate_graph_changed",
            Self::PlatformBoundary => "platform_boundary_changed",
        }
    }
}

pub const REQUIRED_UNSUPPORTED_EDIT_CLASSES: &[RustEditClass] = &[
    RustEditClass::Signature,
    RustEditClass::Type,
    RustEditClass::Module,
    RustEditClass::StateShape,
    RustEditClass::Dependency,
    RustEditClass::Macro,
    RustEditClass::CrateGraph,
    RustEditClass::PlatformBoundary,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActionBodyFingerprint {
    pub action_id: ActionId,
    pub signature: &'static str,
    pub type_contract: &'static str,
    pub module_contract: &'static str,
    pub state_shape: &'static str,
    pub dependency_set: &'static str,
    pub macro_expansion: &'static str,
    pub crate_graph: &'static str,
    pub platform_boundary: &'static str,
    pub body_token: &'static str,
    pub source_span_id: Option<SourceSpanId>,
}

impl ActionBodyFingerprint {
    pub fn with_body_token(mut self, body_token: &'static str) -> Self {
        self.body_token = body_token;
        self
    }

    pub fn with_edit_class_drift(mut self, edit_class: RustEditClass) -> Self {
        match edit_class {
            RustEditClass::ActionBody => {
                self.body_token = "body.save.v2";
            }
            RustEditClass::Signature => {
                self.signature = "fn save(model: SaveModel, audit: AuditTrail) -> ActionResult";
            }
            RustEditClass::Type => {
                self.type_contract = "action.save.types.v2";
            }
            RustEditClass::Module => {
                self.module_contract = "module.actions.v2";
            }
            RustEditClass::StateShape => {
                self.state_shape = "state.app.v2";
            }
            RustEditClass::Dependency => {
                self.dependency_set = "deps.validation.v2";
            }
            RustEditClass::Macro => {
                self.macro_expansion = "macro.action_dispatch.v2";
            }
            RustEditClass::CrateGraph => {
                self.crate_graph = "crate_graph.hot_patch.v2";
            }
            RustEditClass::PlatformBoundary => {
                self.platform_boundary = "platform_boundary.actions.v2";
            }
        }
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActionBodyHotPatch {
    pub action_id: ActionId,
    pub previous_body_token: &'static str,
    pub next_body_token: &'static str,
    pub source_span_id: Option<SourceSpanId>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnsupportedEditInput {
    pub edit_class: RustEditClass,
    pub source_span_id: Option<SourceSpanId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnsupportedEditDiagnostic {
    pub edit_class: RustEditClass,
    pub diagnostic: HotPatchDiagnostic,
}

pub fn detect_hot_reload_action_body_patch(
    intent: &HotReloadPatchIntent,
    before: ActionBodyFingerprint,
) -> HotPatchResult<ActionBodyHotPatch> {
    let HotReloadEdit::ActionBody(ActionBodyPatch {
        action_id,
        edit_token,
    }) = intent.edit
    else {
        return Err(unsupported_edit(UnsupportedEditInput {
            edit_class: RustEditClass::Signature,
            source_span_id: intent.source_span_id,
        })
        .diagnostic);
    };

    detect_action_body_hot_patch(
        before,
        ActionBodyFingerprint {
            action_id,
            body_token: edit_token,
            source_span_id: intent.source_span_id,
            ..before
        },
    )
}

pub fn detect_action_body_hot_patch(
    before: ActionBodyFingerprint,
    after: ActionBodyFingerprint,
) -> HotPatchResult<ActionBodyHotPatch> {
    if before.action_id != after.action_id || before.signature != after.signature {
        return Err(rebuild_required(
            RustEditClass::Signature,
            after.source_span_id,
        ));
    }
    if before.type_contract != after.type_contract {
        return Err(rebuild_required(RustEditClass::Type, after.source_span_id));
    }
    if before.module_contract != after.module_contract {
        return Err(rebuild_required(
            RustEditClass::Module,
            after.source_span_id,
        ));
    }
    if before.state_shape != after.state_shape {
        return Err(rebuild_required(
            RustEditClass::StateShape,
            after.source_span_id,
        ));
    }
    if before.dependency_set != after.dependency_set {
        return Err(rebuild_required(
            RustEditClass::Dependency,
            after.source_span_id,
        ));
    }
    if before.macro_expansion != after.macro_expansion {
        return Err(rebuild_required(RustEditClass::Macro, after.source_span_id));
    }
    if before.crate_graph != after.crate_graph {
        return Err(rebuild_required(
            RustEditClass::CrateGraph,
            after.source_span_id,
        ));
    }
    if before.platform_boundary != after.platform_boundary {
        return Err(rebuild_required(
            RustEditClass::PlatformBoundary,
            after.source_span_id,
        ));
    }
    if before.body_token == after.body_token {
        return Err(HotPatchDiagnostic::rebuild_required(
            HOT_PATCH_NO_ACTION_BODY_CHANGE,
            RustEditClass::ActionBody.path(),
            "action_body_unchanged",
            format!("action {} body did not change", before.action_id.as_str()),
            after.source_span_id,
        ));
    }
    Ok(ActionBodyHotPatch {
        action_id: before.action_id,
        previous_body_token: before.body_token,
        next_body_token: after.body_token,
        source_span_id: after.source_span_id,
    })
}

pub fn unsupported_edit(input: UnsupportedEditInput) -> UnsupportedEditDiagnostic {
    UnsupportedEditDiagnostic {
        edit_class: input.edit_class,
        diagnostic: rebuild_required(input.edit_class, input.source_span_id),
    }
}

fn rebuild_required(
    edit_class: RustEditClass,
    source_span_id: Option<SourceSpanId>,
) -> HotPatchDiagnostic {
    let decision = support_decision(edit_class);
    HotPatchDiagnostic::rebuild_required(
        decision.diagnostic_code,
        edit_class.path(),
        decision.reason,
        format!(
            "{} edits require a rebuild; only action_body edits patch live",
            edit_class.as_str()
        ),
        source_span_id,
    )
}
