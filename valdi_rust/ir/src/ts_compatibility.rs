//! TypeScript compatibility schema.
//!
//! ```
//! use valdi_rust_ir::ts_compatibility::{DirectRendererCompatibility, RustPathDependencyCheck};
//!
//! assert_eq!(DirectRendererCompatibility::RetainedOnly, DirectRendererCompatibility::RetainedOnly);
//! assert!(RustPathDependencyCheck { excludes_typescript_runtime: true }.excludes_typescript_runtime);
//! ```

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TsxCoverageMap {
    pub feature: &'static str,
    pub covered: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DirectRendererCompatibility {
    RetainedOnly,
    EquivalentIr,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TsxToIrEquivalence {
    pub fixture_name: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RustPathDependencyCheck {
    pub excludes_typescript_runtime: bool,
}
