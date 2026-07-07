//! Platform target and capability identity schema.
//!
//! ```
//! use valdi_rust_ir::{ids::CapabilityId, platform::{PlatformCapability, PlatformTarget}};
//!
//! let capability = PlatformCapability::new(PlatformTarget::Ios, CapabilityId::new("safe_area"));
//! assert_eq!(capability.target, PlatformTarget::Ios);
//! ```

use crate::ids::CapabilityId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlatformTarget {
    Ios,
    Android,
    Web,
    Png,
    RustHost,
    Swift,
    Kotlin,
    JsDom,
    CppTransition,
    TsCompatibility,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlatformCapability {
    pub target: PlatformTarget,
    pub capability_id: CapabilityId,
}

impl PlatformCapability {
    pub const fn new(target: PlatformTarget, capability_id: CapabilityId) -> Self {
        Self { target, capability_id }
    }
}
