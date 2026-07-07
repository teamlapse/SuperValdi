//! Hot reload schema.
//!
//! ```
//! use valdi_rust_ir::{hot_reload::{CompatibilityClass, HotReloadPatch, PatchIdentity}, ids::HotReloadIdentityId};
//!
//! let patch = HotReloadPatch { identity: PatchIdentity::new(HotReloadIdentityId::new("patch")), compatibility: CompatibilityClass::UiOnly };
//! assert_eq!(patch.compatibility, CompatibilityClass::UiOnly);
//! ```

use crate::ids::{ActionId, AssetId, BindingId, HotReloadIdentityId, ModuleId, NativeViewId, SourceSpanId};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HotReloadPatch {
    pub identity: PatchIdentity,
    pub compatibility: CompatibilityClass,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PatchIdentity {
    pub id: HotReloadIdentityId,
    pub source_span_id: Option<SourceSpanId>,
}

impl PatchIdentity {
    pub const fn new(id: HotReloadIdentityId) -> Self {
        Self { id, source_span_id: None }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompatibilityClass {
    UiOnly,
    RequiresRustActionPatch,
    RequiresRebuild,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AssetPatchRef(pub AssetId);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BindingPatchRef(pub BindingId);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActionPatchRef(pub ActionId);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ModuleRefPatch(pub ModuleId);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeViewRefPatch(pub NativeViewId);
