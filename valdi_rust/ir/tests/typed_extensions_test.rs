use valdi_rust_ir::{
    extensions::{
        AndroidPlatformExtension, ExtensionNamespace, ExtensionVersion, IosPlatformExtension,
        PlatformExtension, PlatformExtensionPayload, PngPlatformExtension, RustHostPlatformExtension,
        WebPlatformExtension,
    },
    ids::CapabilityId,
};

#[test]
fn platform_extensions_are_named_versioned_and_typed() {
    let extensions = [
        PlatformExtension::new(
            ExtensionNamespace::Ios,
            ExtensionVersion::new(1, 0),
            CapabilityId::new("ios.safe_area"),
            PlatformExtensionPayload::Ios(IosPlatformExtension::SafeAreaBehavior),
        ),
        PlatformExtension::new(
            ExtensionNamespace::Android,
            ExtensionVersion::new(1, 0),
            CapabilityId::new("android.insets"),
            PlatformExtensionPayload::Android(AndroidPlatformExtension::WindowInsetBehavior),
        ),
        PlatformExtension::new(
            ExtensionNamespace::Web,
            ExtensionVersion::new(1, 0),
            CapabilityId::new("web.css"),
            PlatformExtensionPayload::Web(WebPlatformExtension::CssCustomProperty("--valdi-color")),
        ),
        PlatformExtension::new(
            ExtensionNamespace::Png,
            ExtensionVersion::new(1, 0),
            CapabilityId::new("png.debug"),
            PlatformExtensionPayload::Png(PngPlatformExtension::SnapshotDebugLayer),
        ),
        PlatformExtension::new(
            ExtensionNamespace::RustHost,
            ExtensionVersion::new(1, 0),
            CapabilityId::new("rust.host"),
            PlatformExtensionPayload::RustHost(RustHostPlatformExtension::HostCapability(
                CapabilityId::new("dynamic_ui"),
            )),
        ),
    ];

    for extension in extensions {
        assert_eq!(extension.version.major, 1);
        assert!(extension.capability_id.as_str().contains('.'));
        match extension.payload {
            PlatformExtensionPayload::Ios(_)
            | PlatformExtensionPayload::Android(_)
            | PlatformExtensionPayload::Web(_)
            | PlatformExtensionPayload::Png(_)
            | PlatformExtensionPayload::RustHost(_) => {}
        }
    }
}
