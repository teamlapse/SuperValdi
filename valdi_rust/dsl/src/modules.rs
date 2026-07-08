use valdi_rust_ir::{
    ids::ModuleId,
    native_modules::{
        ContractType, DispatchTarget, ModuleContract, ModuleFactoryTarget, TypeMatrixEntry,
    },
};

pub const fn module_contract(id: ModuleId, dispatch_target: DispatchTarget) -> ModuleContract {
    ModuleContract {
        id,
        dispatch_target,
    }
}

pub const fn type_matrix_entry(
    rust_type: ContractType,
    platform_target: ModuleFactoryTarget,
) -> TypeMatrixEntry {
    TypeMatrixEntry {
        rust_type,
        platform_target,
    }
}
