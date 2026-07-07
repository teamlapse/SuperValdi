//! Versioned schema and contract-row coverage metadata.
//!
//! ```
//! use valdi_rust_ir::schema::{ContractSurface, SchemaVersion};
//!
//! let version = SchemaVersion::new(1, 0);
//! assert_eq!(version.major, 1);
//! assert_eq!(ContractSurface::SchemaVersioning.contract_id(), "schema_versioning");
//! ```

use crate::ids::{CapabilityId, SchemaVersionId};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SchemaVersion {
    pub id: SchemaVersionId,
    pub major: u16,
    pub minor: u16,
}

impl SchemaVersion {
    pub const fn new(major: u16, minor: u16) -> Self {
        Self { id: SchemaVersionId::new("ui_ir"), major, minor }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FeatureFlag {
    pub capability_id: CapabilityId,
    pub enabled_by_default: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WireFormatVersions {
    pub binary_version: u16,
    pub json_debug_version: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ContractRowCoverage {
    pub id: &'static str,
    pub surface: &'static str,
    pub module: &'static str,
    pub public_types: &'static [&'static str],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContractSurface {
    SchemaVersioning,
    ComponentIdentity,
    TreeStructure,
    ElementTaxonomy,
    Layout,
    Styling,
    Text,
    Assets,
    EventsGestures,
    ActionsState,
    BindingsExpressions,
    Animations,
    NativeModules,
    NativeViews,
    Accessibility,
    HotReload,
    Diagnostics,
    WebDom,
    PngBackend,
    DynamicUi,
    TsCompatibility,
    BuildGraph,
}

impl ContractSurface {
    pub const fn contract_id(self) -> &'static str {
        match self {
            Self::SchemaVersioning => "schema_versioning",
            Self::ComponentIdentity => "component_identity",
            Self::TreeStructure => "tree_structure",
            Self::ElementTaxonomy => "element_taxonomy",
            Self::Layout => "layout",
            Self::Styling => "styling",
            Self::Text => "text",
            Self::Assets => "assets",
            Self::EventsGestures => "events_gestures",
            Self::ActionsState => "actions_state",
            Self::BindingsExpressions => "bindings_expressions",
            Self::Animations => "animations",
            Self::NativeModules => "native_modules",
            Self::NativeViews => "native_views",
            Self::Accessibility => "accessibility",
            Self::HotReload => "hot_reload",
            Self::Diagnostics => "diagnostics",
            Self::WebDom => "web_dom",
            Self::PngBackend => "png_backend",
            Self::DynamicUi => "dynamic_ui",
            Self::TsCompatibility => "ts_compatibility",
            Self::BuildGraph => "build_graph",
        }
    }
}

pub const CONTRACT_ROW_COUNT: usize = 22;

pub const CONTRACT_ROW_COVERAGE: &[ContractRowCoverage] = &[
    ContractRowCoverage { id: "schema_versioning", surface: "Schema and versioning", module: "schema", public_types: &["SchemaVersion", "FeatureFlag", "WireFormatVersions", "ContractSurface"] },
    ContractRowCoverage { id: "component_identity", surface: "Component identity", module: "ids", public_types: &["ComponentId", "NodeId", "KeyId", "StateId", "SourceSpanId", "HotReloadIdentityId", "DynamicIdentityId"] },
    ContractRowCoverage { id: "tree_structure", surface: "Tree structure", module: "tree", public_types: &["UiDocument", "UiNode", "TreeChild", "FragmentNode", "SlotNode", "PortalNode", "ContextScope", "DestructionPolicy", "PoolingPolicy"] },
    ContractRowCoverage { id: "element_taxonomy", surface: "Element taxonomy", module: "elements", public_types: &["ElementKind", "ElementIdentity", "NativeViewRef"] },
    ContractRowCoverage { id: "layout", surface: "Layout", module: "layout", public_types: &["LayoutStyle", "FlexLayout", "AbsoluteLayout", "MeasurePolicy", "SafeAreaEdges", "ScrollSizing", "ZOrder", "ClipBehavior", "Transform", "RtlMode"] },
    ContractRowCoverage { id: "styling", surface: "Styling", module: "styling", public_types: &["Style", "Background", "Color", "Border", "Radius", "Shadow", "Overflow", "Visibility", "Display", "ClassMetadata", "StylePlatformExtension"] },
    ContractRowCoverage { id: "text", surface: "Text", module: "text", public_types: &["TextNode", "RichText", "TextSpan", "FontStyle", "Wrapping", "Truncation", "LinkTarget", "TextMeasurementHook", "TextInputState", "SelectionRange", "CompositionRange"] },
    ContractRowCoverage { id: "assets", surface: "Assets", module: "assets", public_types: &["AssetRef", "AssetVariant", "RemoteAssetRef", "DataAssetRef", "AssetResizeMode", "AssetLoadingState", "AssetError", "AnimatedAssetMetadata", "AssetCacheKey"] },
    ContractRowCoverage { id: "events_gestures", surface: "Events and gestures", module: "events", public_types: &["EventBinding", "EventKind", "GestureKind", "ScrollEventPayload", "InputEventPayload", "KeyboardSubmitPayload", "LayoutEventPayload", "DrawEventPayload", "VisibilityEventPayload", "FrameObserverPayload", "CustomNativeEventSpec"] },
    ContractRowCoverage { id: "actions_state", surface: "Actions and state", module: "actions", public_types: &["ActionDefinition", "ActionKind", "ActionResult", "ActionError", "CancellationPolicy", "InvalidationTarget", "SchedulingPolicy", "CoalescingKey", "StateSlot"] },
    ContractRowCoverage { id: "bindings_expressions", surface: "Bindings and expressions", module: "bindings", public_types: &["BindingExpression", "FieldPath", "LiteralValue", "ComparisonOp", "LogicOp", "ListProjection", "ComputedProjection", "PlatformConstant", "BindingSourceSpan"] },
    ContractRowCoverage { id: "animations", surface: "Animations", module: "animations", public_types: &["Animation", "AnimationKind", "AnimationLifecycle", "Timing", "Easing", "RepeatMode", "FillMode", "AnimatedProperty", "LayoutAnimation", "TransactionGroup"] },
    ContractRowCoverage { id: "native_modules", surface: "Native modules", module: "native_modules", public_types: &["ModuleContract", "ContractType", "TypeMatrixEntry", "DispatchTarget", "ModuleFactoryTarget"] },
    ContractRowCoverage { id: "native_views", surface: "Native views", module: "native_views", public_types: &["NativeViewContract", "NativeViewAttribute", "NativeViewEvent", "LifecycleHook", "MeasurementContract", "ReusePolicy", "NativeViewAccessibility", "FallbackSpec"] },
    ContractRowCoverage { id: "accessibility", surface: "Accessibility", module: "accessibility", public_types: &["AccessibilityNode", "AccessibilityRole", "AccessibilityLabel", "AccessibilityHint", "AccessibilityValue", "AccessibilityState", "AccessibilityAction", "FocusOrder", "GroupingBehavior", "HiddenState", "AccessibilityOverride"] },
    ContractRowCoverage { id: "hot_reload", surface: "Hot reload", module: "hot_reload", public_types: &["HotReloadPatch", "PatchIdentity", "CompatibilityClass", "AssetPatchRef", "BindingPatchRef", "ActionPatchRef", "ModuleRefPatch", "NativeViewRefPatch"] },
    ContractRowCoverage { id: "diagnostics", surface: "Diagnostics", module: "diagnostics", public_types: &["Diagnostic", "DiagnosticSeverity", "SchemaPath", "BackendPath", "CapabilityError", "UnsupportedSurface", "DiagnosticFixtureId", "ActionTraceId", "ModuleTraceId"] },
    ContractRowCoverage { id: "web_dom", surface: "Web DOM", module: "web_dom", public_types: &["DomMapping", "CssStyleMapping", "DomEventMapping", "TextMeasurementSpec", "ClassEmission", "BrowserSnapshotSpec"] },
    ContractRowCoverage { id: "png_backend", surface: "PNG backend", module: "png", public_types: &["PngRenderPlan", "LayoutSnapshot", "DisplayList", "DisplayCommand", "NativeViewFallback", "WebViewFallback", "AccessibilityDebugMetadata"] },
    ContractRowCoverage { id: "dynamic_ui", surface: "Dynamic UI", module: "dynamic_ui", public_types: &["DynamicUiProducer", "DynamicUiInput", "DynamicUiValidationState", "GeneratedFixtureRef", "DebugJsonLabel", "BinaryBytesLabel", "InMemoryProducerTag"] },
    ContractRowCoverage { id: "ts_compatibility", surface: "TS compatibility", module: "ts_compatibility", public_types: &["TsxCoverageMap", "DirectRendererCompatibility", "TsxToIrEquivalence", "RustPathDependencyCheck"] },
    ContractRowCoverage { id: "build_graph", surface: "Build graph", module: "build_graph", public_types: &["BazelTargetRef", "CrateGraphEdge", "GeneratedGlueLabel", "PlatformAppTarget", "CompatibilityLabel", "ForbiddenDependencyCheck"] },
];
