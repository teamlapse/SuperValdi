//! Build graph schema.
//!
//! ```
//! use valdi_rust_ir::build_graph::{BazelTargetRef, ForbiddenDependencyCheck};
//!
//! let check = ForbiddenDependencyCheck { dependency_name: "RenderRequest", forbidden: true };
//! assert!(check.forbidden);
//! assert_eq!(BazelTargetRef::new("//valdi_rust/ir:ir").label, "//valdi_rust/ir:ir");
//! ```

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BazelTargetRef {
    pub label: &'static str,
}

impl BazelTargetRef {
    pub const fn new(label: &'static str) -> Self {
        Self { label }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CrateGraphEdge {
    pub from: BazelTargetRef,
    pub to: BazelTargetRef,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GeneratedGlueLabel {
    pub label: BazelTargetRef,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlatformAppTarget {
    pub platform: &'static str,
    pub label: BazelTargetRef,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CompatibilityLabel {
    pub label: BazelTargetRef,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForbiddenDependencyCheck {
    pub dependency_name: &'static str,
    pub forbidden: bool,
}
