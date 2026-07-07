//! Typed, named, versioned platform extension schema.
//!
//! ```
//! use valdi_rust_ir::{
//!     extensions::{ExtensionNamespace, ExtensionVersion, IosPlatformExtension, PlatformExtension, PlatformExtensionPayload},
//!     ids::CapabilityId,
//! };
//!
//! let extension = PlatformExtension::new(
//!     ExtensionNamespace::Ios,
//!     ExtensionVersion::new(1, 0),
//!     CapabilityId::new("safe_area"),
//!     PlatformExtensionPayload::Ios(IosPlatformExtension::SafeAreaBehavior),
//! );
//! assert_eq!(extension.version.major, 1);
//! ```

use crate::ids::CapabilityId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExtensionVersion {
    pub major: u16,
    pub minor: u16,
}

impl ExtensionVersion {
    pub const fn new(major: u16, minor: u16) -> Self {
        Self { major, minor }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExtensionNamespace {
    Ios,
    Android,
    Web,
    Png,
    RustHost,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlatformExtension {
    pub namespace: ExtensionNamespace,
    pub version: ExtensionVersion,
    pub capability_id: CapabilityId,
    pub payload: PlatformExtensionPayload,
}

impl PlatformExtension {
    pub const fn new(
        namespace: ExtensionNamespace,
        version: ExtensionVersion,
        capability_id: CapabilityId,
        payload: PlatformExtensionPayload,
    ) -> Self {
        Self { namespace, version, capability_id, payload }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlatformExtensionPayload {
    Ios(IosPlatformExtension),
    Android(AndroidPlatformExtension),
    Web(WebPlatformExtension),
    Png(PngPlatformExtension),
    RustHost(RustHostPlatformExtension),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IosPlatformExtension {
    SafeAreaBehavior,
    UIKitClass(&'static str),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AndroidPlatformExtension {
    ViewClass(&'static str),
    WindowInsetBehavior,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WebPlatformExtension {
    DomAttribute(&'static str),
    CssCustomProperty(&'static str),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PngPlatformExtension {
    SnapshotDebugLayer,
    RasterFallback,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RustHostPlatformExtension {
    HostCapability(CapabilityId),
}
