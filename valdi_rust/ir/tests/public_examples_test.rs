use valdi_rust_ir::{
    accessibility::{
        AccessibilityAction, AccessibilityHint, AccessibilityLabel, AccessibilityNode,
        AccessibilityOverride, AccessibilityRole, AccessibilityState, AccessibilityValue,
        FocusOrder, GroupingBehavior, HiddenState,
    },
    actions::{
        ActionDefinition, ActionError, ActionKind, ActionResult, CancellationPolicy, CoalescingKey,
        InvalidationTarget, SchedulingPolicy, StateSlot,
    },
    animations::{
        AnimatedProperty, Animation, AnimationKind, AnimationLifecycle, Easing, FillMode,
        LayoutAnimation, RepeatMode, Timing, TransactionGroup,
    },
    assets::{
        AnimatedAssetMetadata, AssetCacheKey, AssetError, AssetLoadingState, AssetRef,
        AssetResizeMode, AssetVariant, DataAssetRef, RemoteAssetRef,
    },
    bindings::{
        BindingExpression, BindingSourceSpan, ComparisonOp, ComputedProjection, FieldPath,
        ListProjection, LiteralValue, LogicOp, PlatformConstant,
    },
    build_graph::{
        BazelTargetRef, CompatibilityLabel, CrateGraphEdge, ForbiddenDependencyCheck,
        GeneratedGlueLabel, PlatformAppTarget,
    },
    diagnostics::{
        ActionTraceId, BackendPath, CapabilityError, Diagnostic, DiagnosticFixtureId,
        DiagnosticSeverity, ModuleTraceId, SchemaPath, UnsupportedSurface,
    },
    dynamic_ui::{
        BinaryBytesLabel, DebugJsonLabel, DynamicUiInput, DynamicUiProducer,
        DynamicUiValidationState, GeneratedFixtureRef, InMemoryProducerTag,
    },
    elements::{ElementIdentity, ElementKind, NativeViewRef},
    events::{
        CustomNativeEventSpec, DrawEventPayload, EventBinding, EventKind, FrameObserverPayload,
        GestureKind, InputEventPayload, KeyboardSubmitPayload, LayoutEventPayload,
        ScrollEventPayload, VisibilityEventPayload,
    },
    extensions::{
        AndroidPlatformExtension, ExtensionNamespace, ExtensionVersion, IosPlatformExtension,
        PlatformExtension, PlatformExtensionPayload, PngPlatformExtension,
        RustHostPlatformExtension, WebPlatformExtension,
    },
    hot_reload::{
        ActionPatchRef, AssetPatchRef, BindingPatchRef, CompatibilityClass, HotReloadPatch,
        ModuleRefPatch, NativeViewRefPatch, PatchIdentity,
    },
    ids::{
        ActionId, AssetId, BackendPathId, BindingId, CapabilityId, ComponentId, DiagnosticId,
        DynamicIdentityId, FixtureId, HotReloadIdentityId, KeyId, ModuleId, NativeViewId, NodeId,
        SchemaVersionId, SourceSpanId, StateId,
    },
    layout::{
        AbsoluteLayout, ClipBehavior, FlexDirection, FlexLayout, LayoutStyle, Length, MeasurePolicy,
        RtlMode, SafeAreaEdges, ScrollSizing, Transform, ZOrder,
    },
    native_modules::{
        ContractType, DispatchTarget, ModuleContract, ModuleFactoryTarget, TypeMatrixEntry,
    },
    native_views::{
        FallbackSpec, LifecycleHook, MeasurementContract, NativeViewAccessibility,
        NativeViewAttribute, NativeViewContract, NativeViewEvent, ReusePolicy,
    },
    platform::{PlatformCapability, PlatformTarget},
    png::{
        AccessibilityDebugMetadata, DisplayCommand, DisplayList, LayoutSnapshot, NativeViewFallback,
        PngRenderPlan, WebViewFallback,
    },
    schema::{
        ContractSurface, FeatureFlag, SchemaVersion, WireFormatVersions, CONTRACT_ROW_COVERAGE,
    },
    styling::{
        Background, Border, ClassMetadata, Color, Display, Overflow, Radius, Shadow, Style,
        StylePlatformExtension, Visibility,
    },
    text::{
        CompositionRange, FontStyle, LinkTarget, RichText, SelectionRange, TextInputState,
        TextMeasurementHook, TextNode, TextSpan, Truncation, Wrapping,
    },
    tree::{
        ChildOrder, ContextScope, DestructionPolicy, FragmentNode, PoolingPolicy, PortalNode,
        RootNode, SlotNode, TreeChild, UiDocument, UiNode,
    },
    ts_compatibility::{
        DirectRendererCompatibility, RustPathDependencyCheck, TsxCoverageMap, TsxToIrEquivalence,
    },
    web_dom::{
        BrowserSnapshotSpec, ClassEmission, CssStyleMapping, DomEventMapping, DomMapping,
        TextMeasurementSpec,
    },
};

#[test]
fn schema_ids_platform_and_extensions_compile_as_public_api() {
    let version = SchemaVersion::new(1, 0);
    let _version_id = SchemaVersionId::new("ui_ir");
    let capability_id = CapabilityId::new("ios.safe_area");
    let flag = FeatureFlag { capability_id, enabled_by_default: true };
    let wire = WireFormatVersions { binary_version: 1, json_debug_version: 1 };
    let platform_capability = PlatformCapability::new(PlatformTarget::Ios, capability_id);
    let extension = PlatformExtension::new(
        ExtensionNamespace::Ios,
        ExtensionVersion::new(1, 0),
        capability_id,
        PlatformExtensionPayload::Ios(IosPlatformExtension::SafeAreaBehavior),
    );
    let _extension_variants = [
        PlatformExtensionPayload::Android(AndroidPlatformExtension::WindowInsetBehavior),
        PlatformExtensionPayload::Web(WebPlatformExtension::DomAttribute("data-valdi")),
        PlatformExtensionPayload::Png(PngPlatformExtension::RasterFallback),
        PlatformExtensionPayload::RustHost(RustHostPlatformExtension::HostCapability(
            CapabilityId::new("rust.dynamic_ui"),
        )),
    ];
    let _platform_targets = [
        PlatformTarget::Ios,
        PlatformTarget::Android,
        PlatformTarget::Web,
        PlatformTarget::Png,
        PlatformTarget::RustHost,
        PlatformTarget::Swift,
        PlatformTarget::Kotlin,
        PlatformTarget::JsDom,
        PlatformTarget::CppTransition,
        PlatformTarget::TsCompatibility,
    ];
    let _surfaces = [
        ContractSurface::SchemaVersioning,
        ContractSurface::ComponentIdentity,
        ContractSurface::TreeStructure,
        ContractSurface::ElementTaxonomy,
        ContractSurface::Layout,
        ContractSurface::Styling,
        ContractSurface::Text,
        ContractSurface::Assets,
        ContractSurface::EventsGestures,
        ContractSurface::ActionsState,
        ContractSurface::BindingsExpressions,
        ContractSurface::Animations,
        ContractSurface::NativeModules,
        ContractSurface::NativeViews,
        ContractSurface::Accessibility,
        ContractSurface::HotReload,
        ContractSurface::Diagnostics,
        ContractSurface::WebDom,
        ContractSurface::PngBackend,
        ContractSurface::DynamicUi,
        ContractSurface::TsCompatibility,
        ContractSurface::BuildGraph,
    ];

    assert_eq!(version.id.as_str(), "ui_ir");
    assert!(flag.enabled_by_default);
    assert_eq!(wire.binary_version, 1);
    assert_eq!(platform_capability.target, PlatformTarget::Ios);
    assert_eq!(extension.version.major, 1);
    assert_eq!(CONTRACT_ROW_COVERAGE.len(), 22);
}

#[test]
fn tree_element_layout_and_style_examples_compile_as_public_api() {
    let component_id = ComponentId::new("component");
    let node_id = NodeId::new("root");
    let child_id = NodeId::new("child");
    let state_id = StateId::new("state");
    let _key_id = KeyId::new("key");
    let root = RootNode { node_id };
    let document = UiDocument { root, component_id };
    let node = UiNode { node_id, kind: ElementKind::View, state_id: Some(state_id) };
    let child = TreeChild {
        parent_id: node_id,
        child_id,
        order: ChildOrder(0),
        key: Some(KeyId::new("child-key")),
    };
    let _fragment = FragmentNode { node_id: NodeId::new("fragment") };
    let _slot = SlotNode { node_id: NodeId::new("slot"), slot_name: "leading" };
    let _portal = PortalNode { node_id: NodeId::new("portal"), target: "overlay" };
    let _context = ContextScope { node_id, state_id };
    let _destruction = DestructionPolicy::PreserveForPool;
    let _pooling = PoolingPolicy::ReusableByKind(ElementKind::List);
    let identity = ElementIdentity { component_id, node_id, kind: ElementKind::NativeView };
    let native_ref = NativeViewRef {
        native_view_id: NativeViewId::new("native.camera"),
        host_node_id: node_id,
    };
    let layout = LayoutStyle {
        flex: FlexLayout {
            direction: FlexDirection::Column,
            grow: 1.0,
            shrink: 1.0,
            basis: Length::Auto,
        },
        absolute: Some(AbsoluteLayout {
            top: Length::Points(0.0),
            right: Length::Percent(100.0),
            bottom: Length::Points(0.0),
            left: Length::Points(0.0),
        }),
        measure: MeasurePolicy::Delegate("text"),
        safe_area: SafeAreaEdges { top: true, right: false, bottom: true, left: false },
        scroll_sizing: Some(ScrollSizing {
            horizontal: Length::Auto,
            vertical: Length::Points(320.0),
        }),
        z_order: ZOrder(1),
        clipping: ClipBehavior::Hidden,
        transform: Transform {
            translate_x: 0.0,
            translate_y: 4.0,
            scale: 1.0,
            rotate_degrees: 0.0,
        },
        rtl: RtlMode::Inherit,
    };
    let extension = PlatformExtension::new(
        ExtensionNamespace::Web,
        ExtensionVersion::new(1, 0),
        CapabilityId::new("web.class"),
        PlatformExtensionPayload::Web(WebPlatformExtension::CssCustomProperty("--color")),
    );
    let style = Style {
        background: Background::Solid(Color::rgba(12, 32, 48, 255)),
        opacity: 0.9,
        border: Some(Border { color: Color::rgba(0, 0, 0, 255), width: 1.0 }),
        radius: Some(Radius { top_left: 4.0, top_right: 4.0, bottom_right: 4.0, bottom_left: 4.0 }),
        shadow: Some(Shadow {
            color: Color::rgba(0, 0, 0, 64),
            offset_x: 0.0,
            offset_y: 2.0,
            blur: 8.0,
        }),
        overflow: Overflow::Hidden,
        visibility: Visibility::Visible,
        display: Display::Flex,
        class_metadata: Some(ClassMetadata { class_name: "card" }),
        platform_extension: Some(StylePlatformExtension { extension }),
    };

    assert_eq!(document.root.node_id.as_str(), "root");
    assert_eq!(node.kind.contract_token(), "view");
    assert_eq!(child.order.0, 0);
    assert_eq!(identity.kind, ElementKind::NativeView);
    assert_eq!(native_ref.native_view_id.as_str(), "native.camera");
    assert_eq!(layout.z_order.0, 1);
    assert_eq!(style.display, Display::Flex);
}

#[test]
fn text_asset_event_action_binding_and_animation_examples_compile_as_public_api() {
    let span = TextSpan {
        value: "hello",
        font: FontStyle { family: "System", size_points: 14, bold: true, italic: false },
        link: Some(LinkTarget { href: "https://example.invalid", source_span_id: SourceSpanId::new("text:1") }),
    };
    static SPANS: &[TextSpan] = &[TextSpan {
        value: "static",
        font: FontStyle { family: "System", size_points: 12, bold: false, italic: false },
        link: None,
    }];
    let rich = RichText { spans: SPANS };
    let text = TextNode { value: "value", wrapping: Wrapping::Word };
    let input = TextInputState {
        value_binding: BindingId::new("input.value"),
        selection: Some(SelectionRange { start: 0, end: 1 }),
        composition: Some(CompositionRange { start: 0, end: 1 }),
    };
    let _truncation = Truncation::Tail;
    let _measure = TextMeasurementHook { hook_name: "body" };
    let asset_id = AssetId::new("hero");
    let asset = AssetRef { id: asset_id, source: "hero.png", resize_mode: AssetResizeMode::Cover };
    let _asset_variant = AssetVariant { platform: "ios", scale: 3, path: "hero@3x.png" };
    let _remote = RemoteAssetRef { url: "https://example.invalid/hero.png" };
    let _data = DataAssetRef { media_type: "image/png", bytes_label: "hero_bytes" };
    let _loading = AssetLoadingState::Failed(AssetError { code: "missing", recoverable: false });
    let _animated = AnimatedAssetMetadata { frame_count: 4, duration_ms: 1000 };
    let cache_key = AssetCacheKey { id: asset_id, variant: "ios@3x" };
    let action_id = ActionId::new("submit");
    let event = EventBinding { node_id: NodeId::new("button"), kind: EventKind::Tap, action_id };
    let _event_payloads = (
        GestureKind::Pan,
        ScrollEventPayload { offset_x: 1.0, offset_y: 2.0 },
        InputEventPayload { value: "abc" },
        KeyboardSubmitPayload { value: "abc" },
        LayoutEventPayload { width: 100.0, height: 50.0 },
        DrawEventPayload { frame_number: 1 },
        VisibilityEventPayload { visible: true },
        FrameObserverPayload { frame_number: 2 },
        CustomNativeEventSpec { event_name: "nativeChanged" },
    );
    let action = ActionDefinition { id: action_id, kind: ActionKind::Async, state_scope: StateId::new("form") };
    let _action_result = ActionResult::Failed(ActionError { code: "invalid" });
    let _action_policy = (
        CancellationPolicy::CancelByActionId,
        InvalidationTarget { state_id: StateId::new("form") },
        SchedulingPolicy::Deferred,
        CoalescingKey { value: "submit" },
        StateSlot { id: StateId::new("form"), name: "form" },
    );
    let field = FieldPath::new("form.title");
    let _binding = BindingExpression::Field(field);
    let _binding_parts = (
        BindingExpression::Literal(LiteralValue::Text("title")),
        BindingExpression::Logic(LogicOp::And),
        BindingExpression::Comparison(ComparisonOp::Equal),
        BindingExpression::List(ListProjection { source: field }),
        BindingExpression::Computed(ComputedProjection { name: "title_upper" }),
        BindingExpression::PlatformConstant(PlatformConstant { name: "platform.os" }),
        BindingSourceSpan { source_span_id: SourceSpanId::new("binding:1") },
    );
    let animation = Animation {
        node_id: NodeId::new("box"),
        kind: AnimationKind::Property,
        timing: Timing { duration_ms: 250, delay_ms: 50, easing: Easing::EaseOut },
    };
    let _animation_parts = (
        AnimationLifecycle::Start,
        RepeatMode::Count(2),
        FillMode::Forwards,
        AnimatedProperty::Opacity,
        LayoutAnimation { property: AnimatedProperty::Layout, timing: Timing::milliseconds(300) },
        TransactionGroup { name: "entry" },
    );

    assert_eq!(span.value, "hello");
    assert_eq!(rich.spans.len(), 1);
    assert_eq!(text.wrapping, Wrapping::Word);
    assert_eq!(input.value_binding.as_str(), "input.value");
    assert_eq!(asset.id.as_str(), "hero");
    assert_eq!(cache_key.variant, "ios@3x");
    assert_eq!(event.kind, EventKind::Tap);
    assert_eq!(action.kind, ActionKind::Async);
    assert_eq!(animation.timing.duration_ms, 250);
}

#[test]
fn module_native_view_accessibility_hot_reload_and_diagnostic_examples_compile_as_public_api() {
    let module_id = ModuleId::new("storage");
    let module = ModuleContract { id: module_id, dispatch_target: DispatchTarget::RustHost };
    let _module_matrix = TypeMatrixEntry {
        rust_type: ContractType::Result,
        platform_target: ModuleFactoryTarget::Swift,
    };
    let _module_targets = [
        DispatchTarget::Swift,
        DispatchTarget::Kotlin,
        DispatchTarget::JsDom,
        DispatchTarget::RustHost,
        DispatchTarget::CppTransition,
        DispatchTarget::TsCompatibility,
    ];
    let view_id = NativeViewId::new("camera");
    let native_view = NativeViewContract { id: view_id, lifecycle: LifecycleHook::Create };
    let _native_view_parts = (
        NativeViewAttribute { name: "mode", value_type: "Text" },
        NativeViewEvent { name: "capture", payload_type: "CapturePayload" },
        MeasurementContract { supports_intrinsic_size: true },
        ReusePolicy::ReuseByContract,
        NativeViewAccessibility { role: AccessibilityRole::Image },
        FallbackSpec { png_fallback: true, platform_extension: None },
    );
    let accessibility = AccessibilityNode { node_id: NodeId::new("title"), role: AccessibilityRole::Header };
    let _accessibility_parts = (
        AccessibilityLabel("Title"),
        AccessibilityHint("Reads the title"),
        AccessibilityValue("Title"),
        AccessibilityState { disabled: false, selected: true, checked: false },
        AccessibilityAction { name: "activate" },
        FocusOrder(1),
        GroupingBehavior::GroupChildren,
        HiddenState(false),
        AccessibilityOverride { platform: "ios", label: Some(AccessibilityLabel("iOS title")) },
    );
    let patch = HotReloadPatch {
        identity: PatchIdentity {
            id: HotReloadIdentityId::new("patch"),
            source_span_id: Some(SourceSpanId::new("src:1")),
        },
        compatibility: CompatibilityClass::UiOnly,
    };
    let _patch_refs = (
        AssetPatchRef(AssetId::new("asset")),
        BindingPatchRef(BindingId::new("binding")),
        ActionPatchRef(ActionId::new("action")),
        ModuleRefPatch(module_id),
        NativeViewRefPatch(view_id),
    );
    let diagnostic = Diagnostic { id: DiagnosticId::new("diag"), severity: DiagnosticSeverity::Error };
    let _diagnostic_parts = (
        SchemaPath("/nodes/0"),
        BackendPath(BackendPathId::new("backend.root")),
        CapabilityError { capability_id: CapabilityId::new("capability"), reason: "unsupported" },
        UnsupportedSurface { schema_path: SchemaPath("/attrs/unknown"), source_span_id: Some(SourceSpanId::new("src:2")) },
        DiagnosticFixtureId::new("fixture"),
        FixtureId::new("fixture_alias"),
        ActionTraceId(ActionId::new("trace_action")),
        ModuleTraceId(module_id),
    );

    assert_eq!(module.id.as_str(), "storage");
    assert_eq!(native_view.id.as_str(), "camera");
    assert_eq!(accessibility.role, AccessibilityRole::Header);
    assert_eq!(patch.compatibility, CompatibilityClass::UiOnly);
    assert_eq!(diagnostic.severity, DiagnosticSeverity::Error);
}

#[test]
fn web_png_dynamic_ts_and_build_graph_examples_compile_as_public_api() {
    let dom = DomMapping { tag_name: "button", class_emission: ClassEmission::Generated };
    let _web_parts = (
        CssStyleMapping { property: "color", value_token: "primary" },
        DomEventMapping { event_name: "click", event_kind: EventKind::Press },
        TextMeasurementSpec { measurement_id: "body" },
        BrowserSnapshotSpec { snapshot_name: "home" },
    );
    static COMMANDS: &[DisplayCommand] = &[
        DisplayCommand::Save,
        DisplayCommand::DrawText("Hello"),
        DisplayCommand::DrawImage("hero"),
        DisplayCommand::Restore,
    ];
    let display_list = DisplayList { commands: COMMANDS };
    let png_plan = PngRenderPlan {
        layout_snapshot: LayoutSnapshot { width: 320, height: 480 },
        display_list,
    };
    let _png_parts = (
        NativeViewFallback { label: "native" },
        WebViewFallback { label: "webview" },
        AccessibilityDebugMetadata { role: AccessibilityRole::Button, label: "CTA" },
    );
    let producer = DynamicUiProducer {
        name: "home",
        validation_state: DynamicUiValidationState::Validated,
    };
    let _dynamic_parts = (
        DynamicUiInput::RustDsl,
        DynamicUiInput::GeneratedFixture(GeneratedFixtureRef { fixture_name: "home" }),
        DynamicUiInput::InMemory(InMemoryProducerTag { name: "server" }),
        DebugJsonLabel { label: "home_debug" },
        BinaryBytesLabel { label: "home_bin" },
    );
    let _dynamic_identity = DynamicIdentityId::new("dynamic.home");
    let ts_check = RustPathDependencyCheck { excludes_typescript_runtime: true };
    let _ts_parts = (
        TsxCoverageMap { feature: "view", covered: true },
        DirectRendererCompatibility::EquivalentIr,
        TsxToIrEquivalence { fixture_name: "view" },
    );
    let ir_target = BazelTargetRef::new("//valdi_rust/ir:ir");
    let test_target = BazelTargetRef::new("//valdi_rust/ir:public_examples_test");
    let _build_parts = (
        CrateGraphEdge { from: test_target, to: ir_target },
        GeneratedGlueLabel { label: BazelTargetRef::new("//valdi_rust/tests:rust_host_generated_glue_placeholder") },
        PlatformAppTarget { platform: "ios", label: BazelTargetRef::new("//apps/rust:ios") },
        CompatibilityLabel { label: BazelTargetRef::new("//valdi_rust:crate_graph") },
        ForbiddenDependencyCheck { dependency_name: "RenderRequest", forbidden: true },
    );

    assert_eq!(dom.tag_name, "button");
    assert_eq!(png_plan.display_list.commands.len(), 4);
    assert_eq!(producer.validation_state, DynamicUiValidationState::Validated);
    assert!(ts_check.excludes_typescript_runtime);
}
