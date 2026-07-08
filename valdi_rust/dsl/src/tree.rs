use valdi_rust_ir::{
    accessibility::{
        AccessibilityAction, AccessibilityHint, AccessibilityLabel, AccessibilityNode,
        AccessibilityOverride, AccessibilityState, AccessibilityValue, FocusOrder,
        GroupingBehavior, HiddenState,
    },
    actions::{
        ActionDefinition, ActionResult, CancellationPolicy, CoalescingKey, InvalidationTarget,
        SchedulingPolicy, StateSlot,
    },
    animations::{Animation, AnimationLifecycle, LayoutAnimation, TransactionGroup},
    assets::{
        AnimatedAssetMetadata, AssetCacheKey, AssetLoadingState, AssetRef, AssetVariant,
        DataAssetRef, RemoteAssetRef,
    },
    bindings::{BindingExpression, BindingSourceSpan},
    build_graph::{
        BazelTargetRef, CompatibilityLabel, CrateGraphEdge, ForbiddenDependencyCheck,
        GeneratedGlueLabel, PlatformAppTarget,
    },
    diagnostics::{
        ActionTraceId, BackendPath, CapabilityError, Diagnostic, DiagnosticFixtureId,
        ModuleTraceId, SchemaPath, UnsupportedSurface,
    },
    dynamic_ui::{
        BinaryBytesLabel, DebugJsonLabel, DynamicUiInput, DynamicUiProducer, GeneratedFixtureRef,
    },
    elements::{ElementIdentity, ElementKind, NativeViewRef},
    events::{
        DrawEventPayload, EventBinding, FrameObserverPayload, InputEventPayload,
        KeyboardSubmitPayload, LayoutEventPayload, ScrollEventPayload, VisibilityEventPayload,
    },
    hot_reload::{
        ActionPatchRef, AssetPatchRef, BindingPatchRef, HotReloadPatch, ModuleRefPatch,
        NativeViewRefPatch,
    },
    ids::SourceSpanId,
    layout::LayoutStyle,
    native_modules::{ModuleContract, TypeMatrixEntry},
    native_views::{
        FallbackSpec, MeasurementContract, NativeViewAccessibility, NativeViewAttribute,
        NativeViewContract, NativeViewEvent, ReusePolicy,
    },
    png::{
        AccessibilityDebugMetadata, DisplayList, LayoutSnapshot, NativeViewFallback, PngRenderPlan,
        WebViewFallback,
    },
    schema::{FeatureFlag, SchemaVersion, WireFormatVersions},
    styling::Style,
    text::{RichText, TextInputState, TextMeasurementHook, TextNode, Truncation},
    tree::{
        ContextScope, DestructionPolicy, FragmentNode, PoolingPolicy, PortalNode, SlotNode,
        TreeChild, UiDocument, UiNode,
    },
    ts_compatibility::{
        DirectRendererCompatibility, RustPathDependencyCheck, TsxCoverageMap, TsxToIrEquivalence,
    },
    web_dom::{
        BrowserSnapshotSpec, CssStyleMapping, DomEventMapping, DomMapping, TextMeasurementSpec,
    },
};

use crate::attributes::DslAttribute;

#[derive(Clone, Debug, PartialEq)]
pub struct DslNode {
    pub node: UiNode,
    pub source_span_id: SourceSpanId,
    pub attributes: Vec<DslAttribute>,
    pub events: Vec<EventBinding>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DslDocument {
    pub fixture_id: String,
    pub contract_row_id: String,
    pub surface: String,
    pub fixture_tags: Vec<String>,
    pub platform_targets: Vec<String>,
    pub owner_prs: Vec<String>,
    pub proof_prs: Vec<String>,
    pub serialized_artifact_path: String,
    pub root_node_id: String,
    pub component_id: String,
    pub coverage_tokens: Vec<String>,
    pub source_span: Option<String>,
    pub payload: DslSurfacePayload,
}

impl DslDocument {
    pub fn canonical_debug(&self) -> DslCanonicalDebug {
        DslCanonicalDebug {
            contract_surface: self.contract_row_id.clone(),
            root_node_id: self.root_node_id.clone(),
            component_id: self.component_id.clone(),
            source_span: self.source_span.clone(),
            coverage_tokens: self.coverage_tokens.clone(),
        }
    }

    pub fn snapshot_line(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}",
            self.contract_row_id,
            self.fixture_id,
            self.root_node_id,
            self.component_id,
            self.payload.kind_token(),
            self.coverage_tokens.join(",")
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DslCanonicalDebug {
    pub contract_surface: String,
    pub root_node_id: String,
    pub component_id: String,
    pub source_span: Option<String>,
    pub coverage_tokens: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DslSurfacePayload {
    SchemaVersioning {
        version: SchemaVersion,
        feature: FeatureFlag,
        wire: WireFormatVersions,
    },
    ComponentIdentity {
        document: UiDocument,
        identity: ElementIdentity,
        hot_reload_identity: valdi_rust_ir::ids::HotReloadIdentityId,
        dynamic_identity: valdi_rust_ir::ids::DynamicIdentityId,
    },
    TreeStructure {
        document: UiDocument,
        nodes: Vec<UiNode>,
        children: Vec<TreeChild>,
        fragment: FragmentNode,
        slot: SlotNode,
        portal: PortalNode,
        context: ContextScope,
        destruction_policy: DestructionPolicy,
        pooling_policy: PoolingPolicy,
    },
    ElementTaxonomy {
        elements: Vec<ElementKind>,
        identity: ElementIdentity,
        native_view_ref: NativeViewRef,
    },
    Layout {
        style: LayoutStyle,
    },
    Styling {
        style: Style,
    },
    Text {
        text: TextNode,
        rich_text: RichText,
        text_input: TextInputState,
        truncation: Truncation,
        measurement_hook: TextMeasurementHook,
    },
    Assets {
        asset: AssetRef,
        variant: AssetVariant,
        remote: RemoteAssetRef,
        data: DataAssetRef,
        loading_state: AssetLoadingState,
        animated: AnimatedAssetMetadata,
        cache_key: AssetCacheKey,
    },
    EventsGestures {
        bindings: Vec<EventBinding>,
        scroll: ScrollEventPayload,
        input: InputEventPayload,
        keyboard: KeyboardSubmitPayload,
        layout: LayoutEventPayload,
        draw: DrawEventPayload,
        visibility: VisibilityEventPayload,
        frame: FrameObserverPayload,
    },
    ActionsState {
        action: ActionDefinition,
        result: ActionResult,
        cancellation: CancellationPolicy,
        invalidation: InvalidationTarget,
        scheduling: SchedulingPolicy,
        coalescing_key: CoalescingKey,
        state_slot: StateSlot,
    },
    BindingsExpressions {
        expressions: Vec<BindingExpression>,
        source_span: BindingSourceSpan,
    },
    Animations {
        animation: Animation,
        lifecycle: AnimationLifecycle,
        layout_animation: LayoutAnimation,
        transaction_group: TransactionGroup,
    },
    NativeModules {
        contract: ModuleContract,
        type_matrix_entry: TypeMatrixEntry,
    },
    NativeViews {
        contract: NativeViewContract,
        attribute: NativeViewAttribute,
        event: NativeViewEvent,
        measurement: MeasurementContract,
        reuse_policy: ReusePolicy,
        accessibility: NativeViewAccessibility,
        fallback: FallbackSpec,
    },
    Accessibility {
        node: AccessibilityNode,
        label: AccessibilityLabel,
        hint: AccessibilityHint,
        value: AccessibilityValue,
        state: AccessibilityState,
        action: AccessibilityAction,
        focus_order: FocusOrder,
        grouping: GroupingBehavior,
        hidden: HiddenState,
        override_spec: AccessibilityOverride,
    },
    HotReload {
        patch: HotReloadPatch,
        asset: AssetPatchRef,
        binding: BindingPatchRef,
        action: ActionPatchRef,
        module: ModuleRefPatch,
        native_view: NativeViewRefPatch,
    },
    Diagnostics {
        diagnostic: Diagnostic,
        schema_path: SchemaPath,
        backend_path: BackendPath,
        capability_error: CapabilityError,
        unsupported_surface: UnsupportedSurface,
        fixture_id: DiagnosticFixtureId,
        action_trace: ActionTraceId,
        module_trace: ModuleTraceId,
    },
    WebDom {
        dom: DomMapping,
        css: CssStyleMapping,
        event: DomEventMapping,
        text_measurement: TextMeasurementSpec,
        browser_snapshot: BrowserSnapshotSpec,
    },
    PngBackend {
        plan: PngRenderPlan,
        layout: LayoutSnapshot,
        display_list: DisplayList,
        native_fallback: NativeViewFallback,
        web_fallback: WebViewFallback,
        accessibility: AccessibilityDebugMetadata,
    },
    DynamicUi {
        producer: DynamicUiProducer,
        input: DynamicUiInput,
        generated_fixture: GeneratedFixtureRef,
        debug_json: DebugJsonLabel,
        binary_bytes: BinaryBytesLabel,
    },
    TsCompatibility {
        coverage: TsxCoverageMap,
        compatibility: DirectRendererCompatibility,
        equivalence: TsxToIrEquivalence,
        dependency_check: RustPathDependencyCheck,
    },
    BuildGraph {
        target: BazelTargetRef,
        edge: CrateGraphEdge,
        generated_glue: GeneratedGlueLabel,
        platform_app: PlatformAppTarget,
        compatibility_label: CompatibilityLabel,
        forbidden_dependency: ForbiddenDependencyCheck,
    },
}

impl DslSurfacePayload {
    pub const fn kind_token(&self) -> &'static str {
        match self {
            Self::SchemaVersioning { .. } => "schema_versioning",
            Self::ComponentIdentity { .. } => "component_identity",
            Self::TreeStructure { .. } => "tree_structure",
            Self::ElementTaxonomy { .. } => "element_taxonomy",
            Self::Layout { .. } => "layout",
            Self::Styling { .. } => "styling",
            Self::Text { .. } => "text",
            Self::Assets { .. } => "assets",
            Self::EventsGestures { .. } => "events_gestures",
            Self::ActionsState { .. } => "actions_state",
            Self::BindingsExpressions { .. } => "bindings_expressions",
            Self::Animations { .. } => "animations",
            Self::NativeModules { .. } => "native_modules",
            Self::NativeViews { .. } => "native_views",
            Self::Accessibility { .. } => "accessibility",
            Self::HotReload { .. } => "hot_reload",
            Self::Diagnostics { .. } => "diagnostics",
            Self::WebDom { .. } => "web_dom",
            Self::PngBackend { .. } => "png_backend",
            Self::DynamicUi { .. } => "dynamic_ui",
            Self::TsCompatibility { .. } => "ts_compatibility",
            Self::BuildGraph { .. } => "build_graph",
        }
    }
}

pub fn root_node_id(contract_row_id: &str) -> String {
    format!("node.{contract_row_id}.root")
}

pub fn component_id(contract_row_id: &str) -> String {
    format!("component.{contract_row_id}")
}

pub const REQUIRED_ELEMENT_KINDS: &[ElementKind] = &[
    ElementKind::View,
    ElementKind::Layout,
    ElementKind::Scroll,
    ElementKind::Image,
    ElementKind::Text,
    ElementKind::RichText,
    ElementKind::TextInput,
    ElementKind::Control,
    ElementKind::List,
    ElementKind::WebView,
    ElementKind::NativeView,
    ElementKind::DrawingHost,
];
