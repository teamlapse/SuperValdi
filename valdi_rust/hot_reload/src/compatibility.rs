use valdi_rust_ir::hot_reload::CompatibilityClass;
use valdi_rust_runtime::{StateStore, StateValueKind};

use crate::{
    diagnostics::{
        HotReloadResult, HOT_RELOAD_ACTION_MISSING, HOT_RELOAD_BINDING_MISSING,
        HOT_RELOAD_REBUILD_REQUIRED,
    },
    patch::{HotReloadEdit, HotReloadPatchIntent},
    HotReloadDiagnostic,
};

pub fn compatibility_class(intent: &HotReloadPatchIntent) -> CompatibilityClass {
    intent.edit.compatibility()
}

pub fn validate_binding_and_action_compatibility(
    intent: &HotReloadPatchIntent,
    state: &StateStore,
    has_action: impl FnOnce(valdi_rust_ir::ids::ActionId) -> bool,
) -> HotReloadResult<()> {
    match &intent.edit {
        HotReloadEdit::Binding(binding) => {
            let value = valdi_rust_runtime::evaluate_expression(
                &valdi_rust_runtime::ExpressionContext {
                    state,
                    platform_constants: &valdi_rust_runtime::RuntimePlatformConstants::new(vec![]),
                },
                &binding.binding.expression,
            )
            .map_err(|diagnostic| {
                HotReloadDiagnostic::rebuild_required(
                    HOT_RELOAD_BINDING_MISSING,
                    "$.patch.binding.expression",
                    "binding expression cannot resolve in retained state",
                    diagnostic.message,
                    binding.binding.source_span_id,
                )
            })?;
            if value.kind() == StateValueKind::Null {
                return Err(HotReloadDiagnostic::rebuild_required(
                    HOT_RELOAD_BINDING_MISSING,
                    "$.patch.binding.expression",
                    "binding resolved to null",
                    "hot reload binding patches must resolve to a typed value",
                    binding.binding.source_span_id,
                ));
            }
            Ok(())
        }
        HotReloadEdit::Event(event) => {
            if has_action(event.binding.action_id) {
                return Ok(());
            }
            Err(HotReloadDiagnostic::rebuild_required(
                HOT_RELOAD_ACTION_MISSING,
                "$.patch.event.action_id",
                "event action is not present in the retained scheduler",
                format!("action {} is missing", event.binding.action_id.as_str()),
                intent.source_span_id,
            ))
        }
        HotReloadEdit::ActionBody(action) => Err(HotReloadDiagnostic::rebuild_required(
            HOT_RELOAD_REBUILD_REQUIRED,
            "$.patch.action_body",
            "action body changes are owned by the Rust logic hot patch step",
            format!(
                "action {} body patch requires PR12",
                action.action_id.as_str()
            ),
            intent.source_span_id,
        )),
        _ => Ok(()),
    }
}
