//! Native module contract schema.
//!
//! ```
//! use valdi_rust_ir::{ids::ModuleId, native_modules::{DispatchTarget, ModuleContract}};
//!
//! let contract = ModuleContract { id: ModuleId::new("storage"), dispatch_target: DispatchTarget::RustHost };
//! assert_eq!(contract.dispatch_target, DispatchTarget::RustHost);
//! ```

use crate::ids::ModuleId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ModuleContract {
    pub id: ModuleId,
    pub dispatch_target: DispatchTarget,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContractType {
    Bool,
    I64,
    F64,
    Text,
    Bytes,
    List,
    Object,
    Optional,
    Result,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TypeMatrixEntry {
    pub rust_type: ContractType,
    pub platform_target: ModuleFactoryTarget,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DispatchTarget {
    Swift,
    Kotlin,
    JsDom,
    RustHost,
    CppTransition,
    TsCompatibility,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModuleFactoryTarget {
    Swift,
    Kotlin,
    JsDom,
    RustHost,
    CppTransition,
    TsCompatibility,
}
