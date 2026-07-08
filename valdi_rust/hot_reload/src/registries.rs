use valdi_rust_ir::ids::{ModuleId, NativeViewId};

use crate::{
    diagnostics::{
        HotReloadResult, HOT_RELOAD_MODULE_CONTRACT_SHAPE_CHANGED, HOT_RELOAD_MODULE_REF_MISSING,
        HOT_RELOAD_NATIVE_VIEW_CONTRACT_SHAPE_CHANGED, HOT_RELOAD_NATIVE_VIEW_REF_MISSING,
    },
    HotReloadDiagnostic,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ModuleRefContract {
    pub module_id: ModuleId,
    pub shape: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeViewRefContract {
    pub native_view_id: NativeViewId,
    pub shape: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MockModuleRegistry {
    contracts: Vec<ModuleRefContract>,
}

impl MockModuleRegistry {
    pub fn new(contracts: Vec<ModuleRefContract>) -> Self {
        Self { contracts }
    }

    pub fn validate(
        &self,
        module_id: ModuleId,
        expected_shape: &'static str,
    ) -> HotReloadResult<()> {
        let contract = self
            .contracts
            .iter()
            .find(|contract| contract.module_id == module_id)
            .ok_or_else(|| {
                HotReloadDiagnostic::rebuild_required(
                    HOT_RELOAD_MODULE_REF_MISSING,
                    "$.patch.module_ref.module_id",
                    "module reference is not registered",
                    format!("module {} is missing", module_id.as_str()),
                    None,
                )
            })?;
        if contract.shape == expected_shape {
            return Ok(());
        }
        Err(HotReloadDiagnostic::rebuild_required(
            HOT_RELOAD_MODULE_CONTRACT_SHAPE_CHANGED,
            "$.patch.module_ref.shape",
            "module contract shape changed",
            format!(
                "module {} shape {} does not match expected {}",
                module_id.as_str(),
                contract.shape,
                expected_shape
            ),
            None,
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MockNativeViewRegistry {
    contracts: Vec<NativeViewRefContract>,
}

impl MockNativeViewRegistry {
    pub fn new(contracts: Vec<NativeViewRefContract>) -> Self {
        Self { contracts }
    }

    pub fn validate(
        &self,
        native_view_id: NativeViewId,
        expected_shape: &'static str,
    ) -> HotReloadResult<()> {
        let contract = self
            .contracts
            .iter()
            .find(|contract| contract.native_view_id == native_view_id)
            .ok_or_else(|| {
                HotReloadDiagnostic::rebuild_required(
                    HOT_RELOAD_NATIVE_VIEW_REF_MISSING,
                    "$.patch.native_view_ref.native_view_id",
                    "native view reference is not registered",
                    format!("native view {} is missing", native_view_id.as_str()),
                    None,
                )
            })?;
        if contract.shape == expected_shape {
            return Ok(());
        }
        Err(HotReloadDiagnostic::rebuild_required(
            HOT_RELOAD_NATIVE_VIEW_CONTRACT_SHAPE_CHANGED,
            "$.patch.native_view_ref.shape",
            "native view contract shape changed",
            format!(
                "native view {} shape {} does not match expected {}",
                native_view_id.as_str(),
                contract.shape,
                expected_shape
            ),
            None,
        ))
    }
}

pub fn sample_module_registry() -> MockModuleRegistry {
    MockModuleRegistry::new(vec![ModuleRefContract {
        module_id: ModuleId::new("module.storage"),
        shape: "storage.v1",
    }])
}

pub fn sample_native_view_registry() -> MockNativeViewRegistry {
    MockNativeViewRegistry::new(vec![NativeViewRefContract {
        native_view_id: NativeViewId::new("native.camera"),
        shape: "camera.v1",
    }])
}
