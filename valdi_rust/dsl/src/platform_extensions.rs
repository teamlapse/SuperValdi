use valdi_rust_ir::{
    extensions::{
        AndroidPlatformExtension, ExtensionNamespace, ExtensionVersion, IosPlatformExtension,
        PlatformExtension, PlatformExtensionPayload, PngPlatformExtension,
        RustHostPlatformExtension, WebPlatformExtension,
    },
    ids::CapabilityId,
};

pub fn ios_safe_area(capability_id: CapabilityId) -> PlatformExtension {
    PlatformExtension::new(
        ExtensionNamespace::Ios,
        ExtensionVersion::new(1, 0),
        capability_id,
        PlatformExtensionPayload::Ios(IosPlatformExtension::SafeAreaBehavior),
    )
}

pub fn android_insets(capability_id: CapabilityId) -> PlatformExtension {
    PlatformExtension::new(
        ExtensionNamespace::Android,
        ExtensionVersion::new(1, 0),
        capability_id,
        PlatformExtensionPayload::Android(AndroidPlatformExtension::WindowInsetBehavior),
    )
}

pub fn web_css_property(capability_id: CapabilityId, property: &'static str) -> PlatformExtension {
    PlatformExtension::new(
        ExtensionNamespace::Web,
        ExtensionVersion::new(1, 0),
        capability_id,
        PlatformExtensionPayload::Web(WebPlatformExtension::CssCustomProperty(property)),
    )
}

pub fn png_debug_layer(capability_id: CapabilityId) -> PlatformExtension {
    PlatformExtension::new(
        ExtensionNamespace::Png,
        ExtensionVersion::new(1, 0),
        capability_id,
        PlatformExtensionPayload::Png(PngPlatformExtension::SnapshotDebugLayer),
    )
}

pub fn rust_host_capability(capability_id: CapabilityId) -> PlatformExtension {
    PlatformExtension::new(
        ExtensionNamespace::RustHost,
        ExtensionVersion::new(1, 0),
        capability_id,
        PlatformExtensionPayload::RustHost(RustHostPlatformExtension::HostCapability(
            capability_id,
        )),
    )
}
