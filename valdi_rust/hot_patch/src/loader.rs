use valdi_rust_ir::ids::ActionId;

use crate::{
    detector::ActionBodyHotPatch,
    diagnostics::{HotPatchDiagnostic, HotPatchResult, HOT_PATCH_ACTION_MISSING},
    release_guard::HotPatchBuildProfile,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActionImplementation {
    pub action_id: ActionId,
    pub body_token: &'static str,
    pub result_token: &'static str,
}

impl ActionImplementation {
    pub const fn new(
        action_id: ActionId,
        body_token: &'static str,
        result_token: &'static str,
    ) -> Self {
        Self {
            action_id,
            body_token,
            result_token,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HotPatchLoadRecord {
    pub action_id: ActionId,
    pub previous_body_token: &'static str,
    pub active_body_token: &'static str,
    pub generation: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DevActionImplementationLoader {
    profile: HotPatchBuildProfile,
    implementations: Vec<ActionImplementation>,
    generation: u32,
}

impl DevActionImplementationLoader {
    pub fn new_debug(implementations: Vec<ActionImplementation>) -> Self {
        Self {
            profile: HotPatchBuildProfile::Debug,
            implementations,
            generation: 0,
        }
    }

    pub fn new_release() -> Self {
        Self {
            profile: HotPatchBuildProfile::Release,
            implementations: Vec::new(),
            generation: 0,
        }
    }

    pub const fn profile(&self) -> HotPatchBuildProfile {
        self.profile
    }

    pub fn apply_action_body_patch(
        &mut self,
        patch: ActionBodyHotPatch,
    ) -> HotPatchResult<HotPatchLoadRecord> {
        if self.profile == HotPatchBuildProfile::Release {
            return Err(crate::release_guard::release_excluded_diagnostic(
                patch.source_span_id,
            ));
        }

        let Some(implementation) = self
            .implementations
            .iter_mut()
            .find(|implementation| implementation.action_id == patch.action_id)
        else {
            return Err(HotPatchDiagnostic::rebuild_required(
                HOT_PATCH_ACTION_MISSING,
                "$.rust_hot_patch.action_body.action_id",
                "action_implementation_missing",
                format!("action {} is not loaded", patch.action_id.as_str()),
                patch.source_span_id,
            ));
        };

        let previous = implementation.body_token;
        implementation.body_token = patch.next_body_token;
        implementation.result_token = patch.next_body_token;
        self.generation += 1;
        Ok(HotPatchLoadRecord {
            action_id: patch.action_id,
            previous_body_token: previous,
            active_body_token: implementation.body_token,
            generation: self.generation,
        })
    }

    pub fn dispatch_token(&self, action_id: ActionId) -> HotPatchResult<&'static str> {
        self.implementations
            .iter()
            .find(|implementation| implementation.action_id == action_id)
            .map(|implementation| implementation.result_token)
            .ok_or_else(|| {
                HotPatchDiagnostic::rebuild_required(
                    HOT_PATCH_ACTION_MISSING,
                    "$.rust_hot_patch.action_body.action_id",
                    "action_implementation_missing",
                    format!("action {} is not loaded", action_id.as_str()),
                    None,
                )
            })
    }
}
