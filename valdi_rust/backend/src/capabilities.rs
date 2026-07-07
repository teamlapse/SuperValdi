use crate::operations::BackendOperationFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackendTarget {
    RustHost,
    Ios,
    Android,
    WebDom,
    Png,
    RetainedBackend,
}

impl BackendTarget {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RustHost => "rust_host",
            Self::Ios => "ios",
            Self::Android => "android",
            Self::WebDom => "web_dom",
            Self::Png => "png",
            Self::RetainedBackend => "retained_backend",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackendCapability {
    ViewTree,
    AttributesAndStyle,
    Animations,
    LayoutCallbacks,
    DrawCallbacks,
    VisibilityObservers,
    FrameObservers,
    AssetLoading,
    TextMeasurement,
    NativeViewMount,
    WebViewMount,
    TransactionGroups,
}

impl BackendCapability {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ViewTree => "view_tree",
            Self::AttributesAndStyle => "attributes_and_style",
            Self::Animations => "animations",
            Self::LayoutCallbacks => "layout_callbacks",
            Self::DrawCallbacks => "draw_callbacks",
            Self::VisibilityObservers => "visibility_observers",
            Self::FrameObservers => "frame_observers",
            Self::AssetLoading => "asset_loading",
            Self::TextMeasurement => "text_measurement",
            Self::NativeViewMount => "native_view_mount",
            Self::WebViewMount => "webview_mount",
            Self::TransactionGroups => "transaction_groups",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackendCapabilityDecision {
    Supported(BackendCapability),
    Unsupported(BackendCapability),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackendCapabilitySet {
    pub target: BackendTarget,
    pub capabilities: &'static [BackendCapability],
}

impl BackendCapabilitySet {
    pub const fn new(target: BackendTarget, capabilities: &'static [BackendCapability]) -> Self {
        Self {
            target,
            capabilities,
        }
    }

    pub fn supports(self, capability: BackendCapability) -> bool {
        self.capabilities.contains(&capability)
    }

    pub const fn all_for(target: BackendTarget) -> Self {
        Self {
            target,
            capabilities: ALL_CAPABILITIES,
        }
    }
}

pub const ALL_CAPABILITIES: &[BackendCapability] = &[
    BackendCapability::ViewTree,
    BackendCapability::AttributesAndStyle,
    BackendCapability::Animations,
    BackendCapability::LayoutCallbacks,
    BackendCapability::DrawCallbacks,
    BackendCapability::VisibilityObservers,
    BackendCapability::FrameObservers,
    BackendCapability::AssetLoading,
    BackendCapability::TextMeasurement,
    BackendCapability::NativeViewMount,
    BackendCapability::WebViewMount,
    BackendCapability::TransactionGroups,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixtureTagBackendDecision {
    pub fixture_tag: &'static str,
    pub target: BackendTarget,
    pub capability: BackendCapability,
    pub operation_families: &'static [BackendOperationFamily],
}

pub const FIXTURE_TAG_BACKEND_DECISIONS: &[FixtureTagBackendDecision] = &[
    FixtureTagBackendDecision {
        fixture_tag: "android",
        target: BackendTarget::Android,
        capability: BackendCapability::NativeViewMount,
        operation_families: &[BackendOperationFamily::NativeViewMount],
    },
    FixtureTagBackendDecision {
        fixture_tag: "dom_snapshot",
        target: BackendTarget::WebDom,
        capability: BackendCapability::WebViewMount,
        operation_families: &[BackendOperationFamily::WebViewMount],
    },
    FixtureTagBackendDecision {
        fixture_tag: "dynamic_ui",
        target: BackendTarget::RustHost,
        capability: BackendCapability::ViewTree,
        operation_families: &[
            BackendOperationFamily::Create,
            BackendOperationFamily::Root,
            BackendOperationFamily::TransactionGroup,
        ],
    },
    FixtureTagBackendDecision {
        fixture_tag: "hot_reload",
        target: BackendTarget::RustHost,
        capability: BackendCapability::AttributesAndStyle,
        operation_families: &[
            BackendOperationFamily::Move,
            BackendOperationFamily::AttributeStyle,
            BackendOperationFamily::TransactionGroup,
        ],
    },
    FixtureTagBackendDecision {
        fixture_tag: "ios",
        target: BackendTarget::Ios,
        capability: BackendCapability::NativeViewMount,
        operation_families: &[BackendOperationFamily::NativeViewMount],
    },
    FixtureTagBackendDecision {
        fixture_tag: "module",
        target: BackendTarget::RustHost,
        capability: BackendCapability::TransactionGroups,
        operation_families: &[BackendOperationFamily::TransactionGroup],
    },
    FixtureTagBackendDecision {
        fixture_tag: "native_view",
        target: BackendTarget::RustHost,
        capability: BackendCapability::NativeViewMount,
        operation_families: &[BackendOperationFamily::NativeViewMount],
    },
    FixtureTagBackendDecision {
        fixture_tag: "retained_backend",
        target: BackendTarget::RetainedBackend,
        capability: BackendCapability::ViewTree,
        operation_families: &[
            BackendOperationFamily::Create,
            BackendOperationFamily::Move,
            BackendOperationFamily::Destroy,
        ],
    },
    FixtureTagBackendDecision {
        fixture_tag: "rust_backend",
        target: BackendTarget::RustHost,
        capability: BackendCapability::ViewTree,
        operation_families: &[
            BackendOperationFamily::Create,
            BackendOperationFamily::Root,
            BackendOperationFamily::Move,
        ],
    },
    FixtureTagBackendDecision {
        fixture_tag: "screenshot",
        target: BackendTarget::Png,
        capability: BackendCapability::DrawCallbacks,
        operation_families: &[BackendOperationFamily::DrawCallback],
    },
    FixtureTagBackendDecision {
        fixture_tag: "static_png",
        target: BackendTarget::Png,
        capability: BackendCapability::AssetLoading,
        operation_families: &[
            BackendOperationFamily::AssetLoad,
            BackendOperationFamily::DrawCallback,
        ],
    },
    FixtureTagBackendDecision {
        fixture_tag: "tsx_compat",
        target: BackendTarget::RetainedBackend,
        capability: BackendCapability::AttributesAndStyle,
        operation_families: &[
            BackendOperationFamily::Create,
            BackendOperationFamily::Move,
            BackendOperationFamily::AttributeStyle,
        ],
    },
    FixtureTagBackendDecision {
        fixture_tag: "web",
        target: BackendTarget::WebDom,
        capability: BackendCapability::WebViewMount,
        operation_families: &[BackendOperationFamily::WebViewMount],
    },
];
