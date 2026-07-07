//! Native view contract schema.
//!
//! ```
//! use valdi_rust_ir::{ids::NativeViewId, native_views::{LifecycleHook, NativeViewContract}};
//!
//! let contract = NativeViewContract { id: NativeViewId::new("camera"), lifecycle: LifecycleHook::Create };
//! assert_eq!(contract.lifecycle, LifecycleHook::Create);
//! ```

use crate::accessibility::AccessibilityRole;
use crate::extensions::PlatformExtension;
use crate::ids::NativeViewId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeViewContract {
    pub id: NativeViewId,
    pub lifecycle: LifecycleHook,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeViewAttribute {
    pub name: &'static str,
    pub value_type: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeViewEvent {
    pub name: &'static str,
    pub payload_type: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LifecycleHook {
    Create,
    Update,
    Destroy,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MeasurementContract {
    pub supports_intrinsic_size: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReusePolicy {
    NotReusable,
    ReuseByContract,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeViewAccessibility {
    pub role: AccessibilityRole,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FallbackSpec {
    pub png_fallback: bool,
    pub platform_extension: Option<PlatformExtension>,
}
