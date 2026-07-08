use valdi_rust_ir::{
    accessibility::{
        AccessibilityAction, AccessibilityLabel, AccessibilityNode, AccessibilityOverride,
        AccessibilityRole, AccessibilityState, AccessibilityValue, FocusOrder, GroupingBehavior,
        HiddenState,
    },
    actions::{
        ActionDefinition, ActionError, ActionKind, ActionResult, CancellationPolicy, CoalescingKey,
        InvalidationTarget, SchedulingPolicy, StateSlot,
    },
    animations::{
        AnimatedProperty, Animation, AnimationKind, AnimationLifecycle, Easing, LayoutAnimation,
        Timing, TransactionGroup,
    },
    assets::{
        AnimatedAssetMetadata, AssetCacheKey, AssetError, AssetLoadingState, AssetRef,
        AssetResizeMode, AssetVariant, DataAssetRef, RemoteAssetRef,
    },
    bindings::{ComparisonOp, LiteralValue, LogicOp},
    build_graph::{
        BazelTargetRef, CompatibilityLabel, CrateGraphEdge, ForbiddenDependencyCheck,
        GeneratedGlueLabel, PlatformAppTarget,
    },
    diagnostics::{
        BackendPath, CapabilityError, Diagnostic, DiagnosticSeverity, SchemaPath,
        UnsupportedSurface,
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
        ExtensionNamespace, ExtensionVersion, PlatformExtension, PlatformExtensionPayload,
        WebPlatformExtension,
    },
    hot_reload::{CompatibilityClass, HotReloadPatch, PatchIdentity},
    ids::{
        ActionId, AssetId, BackendPathId, BindingId, CapabilityId, ComponentId, DiagnosticId,
        DynamicIdentityId, FixtureId, HotReloadIdentityId, ModuleId, NativeViewId, NodeId,
        SourceSpanId, StateId,
    },
    layout::{
        AbsoluteLayout, ClipBehavior, FlexDirection, FlexLayout, LayoutStyle, Length,
        MeasurePolicy, RtlMode, SafeAreaEdges, ScrollSizing, Transform, ZOrder,
    },
    native_modules::{ContractType, DispatchTarget, ModuleFactoryTarget},
    native_views::{
        FallbackSpec, LifecycleHook, MeasurementContract, NativeViewAccessibility, ReusePolicy,
    },
    png::{
        AccessibilityDebugMetadata, DisplayCommand, DisplayList, LayoutSnapshot,
        NativeViewFallback, PngRenderPlan, WebViewFallback,
    },
    schema::{FeatureFlag, SchemaVersion, WireFormatVersions},
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

use crate::{
    bindings,
    diagnostics::{
        DslDiagnostic, DslResult, DSL_CANONICAL_IR_DRIFT, DSL_CONTRACT_ROW_UNSUPPORTED,
        DSL_FIXTURE_INPUT_INVALID,
    },
    modules, native_views, platform_extensions,
    tree::{component_id, root_node_id, DslDocument, DslSurfacePayload, REQUIRED_ELEMENT_KINDS},
};

pub const REQUIRED_CONTRACT_ROW_IDS: &[&str] = &[
    "schema_versioning",
    "component_identity",
    "tree_structure",
    "element_taxonomy",
    "layout",
    "styling",
    "text",
    "assets",
    "events_gestures",
    "actions_state",
    "bindings_expressions",
    "animations",
    "native_modules",
    "native_views",
    "accessibility",
    "hot_reload",
    "diagnostics",
    "web_dom",
    "png_backend",
    "dynamic_ui",
    "ts_compatibility",
    "build_graph",
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DslFixtureInput {
    pub fixture_id: String,
    pub contract_row_id: String,
    pub surface: String,
    pub fixture_tags: Vec<String>,
    pub platform_targets: Vec<String>,
    pub owner_prs: Vec<String>,
    pub proof_prs: Vec<String>,
    pub serialized_artifact_path: String,
    pub coverage_tokens: Vec<String>,
}

pub fn contract_fixture(input: DslFixtureInput) -> DslResult<DslDocument> {
    validate_fixture_input(&input)?;
    let payload = payload_for_contract_row(&input.contract_row_id)?;
    Ok(DslDocument {
        root_node_id: root_node_id(&input.contract_row_id),
        component_id: component_id(&input.contract_row_id),
        source_span: None,
        fixture_id: input.fixture_id,
        contract_row_id: input.contract_row_id,
        surface: input.surface,
        fixture_tags: input.fixture_tags,
        platform_targets: input.platform_targets,
        owner_prs: input.owner_prs,
        proof_prs: input.proof_prs,
        serialized_artifact_path: input.serialized_artifact_path,
        coverage_tokens: input.coverage_tokens,
        payload,
    })
}

pub fn canonical_snapshot(inputs: Vec<DslFixtureInput>) -> DslResult<String> {
    let mut documents = Vec::new();
    for input in inputs {
        documents.push(contract_fixture(input)?);
    }
    let mut lines = vec!["dsl_canonical_ir_v1".to_string()];
    for document in documents {
        lines.push(document.snapshot_line());
    }
    Ok(format!("{}\n", lines.join("\n")))
}

pub fn validate_canonical_snapshot(inputs: Vec<DslFixtureInput>, expected: &str) -> DslResult<()> {
    let actual = canonical_snapshot(inputs)?;
    if actual == expected {
        return Ok(());
    }
    Err(DslDiagnostic::error(
        DSL_CANONICAL_IR_DRIFT,
        "$.dsl_canonical_ir",
        "DSL canonical IR snapshot drift",
        None,
    ))
}

fn validate_fixture_input(input: &DslFixtureInput) -> DslResult<()> {
    let checks = [
        ("fixture_id", "$.fixture_id", input.fixture_id.as_str()),
        (
            "contract_row_id",
            "$.contract_row_id",
            input.contract_row_id.as_str(),
        ),
        ("surface", "$.surface", input.surface.as_str()),
        (
            "serialized_artifact_path",
            "$.metadata.serialized_artifact_path",
            input.serialized_artifact_path.as_str(),
        ),
    ];
    for (name, path, value) in checks {
        if value.trim().is_empty() {
            return Err(DslDiagnostic::error(
                DSL_FIXTURE_INPUT_INVALID,
                path,
                format!("{name} is required"),
                None,
            ));
        }
    }
    let vector_checks = [
        ("fixture tags", "$.fixture_tags", input.fixture_tags.len()),
        (
            "platform targets",
            "$.platform_targets",
            input.platform_targets.len(),
        ),
        ("owner PRs", "$.metadata.owner_prs", input.owner_prs.len()),
        ("proof PRs", "$.metadata.proof_prs", input.proof_prs.len()),
        (
            "coverage tokens",
            "$.ir_debug.coverage_tokens",
            input.coverage_tokens.len(),
        ),
    ];
    for (name, path, len) in vector_checks {
        if len == 0 {
            return Err(DslDiagnostic::error(
                DSL_FIXTURE_INPUT_INVALID,
                path,
                format!("{name} are required"),
                None,
            ));
        }
    }
    let expected_fixture_id = format!("contract.{}.v1", input.contract_row_id);
    if input.fixture_id != expected_fixture_id {
        return Err(DslDiagnostic::error(
            DSL_FIXTURE_INPUT_INVALID,
            "$.fixture_id",
            format!(
                "fixture ID drift: {:?} != {:?}",
                input.fixture_id, expected_fixture_id
            ),
            None,
        ));
    }
    let expected_path = format!(
        "valdi_rust/fixtures/serialized/{}.ir.json",
        input.contract_row_id
    );
    if input.serialized_artifact_path != expected_path {
        return Err(DslDiagnostic::error(
            DSL_FIXTURE_INPUT_INVALID,
            "$.metadata.serialized_artifact_path",
            format!(
                "serialized artifact path drift: {:?} != {:?}",
                input.serialized_artifact_path, expected_path
            ),
            None,
        ));
    }
    Ok(())
}

pub fn payload_for_contract_row(contract_row_id: &str) -> DslResult<DslSurfacePayload> {
    match contract_row_id {
        "schema_versioning" => Ok(schema_payload()),
        "component_identity" => Ok(component_identity_payload()),
        "tree_structure" => Ok(tree_structure_payload()),
        "element_taxonomy" => Ok(element_taxonomy_payload()),
        "layout" => Ok(DslSurfacePayload::Layout {
            style: sample_layout(),
        }),
        "styling" => Ok(DslSurfacePayload::Styling {
            style: sample_style(),
        }),
        "text" => Ok(text_payload()),
        "assets" => Ok(assets_payload()),
        "events_gestures" => Ok(events_payload()),
        "actions_state" => Ok(actions_payload()),
        "bindings_expressions" => Ok(bindings_payload()),
        "animations" => Ok(animations_payload()),
        "native_modules" => Ok(native_modules_payload()),
        "native_views" => Ok(native_views_payload()),
        "accessibility" => Ok(accessibility_payload()),
        "hot_reload" => Ok(hot_reload_payload()),
        "diagnostics" => Ok(diagnostics_payload()),
        "web_dom" => Ok(web_dom_payload()),
        "png_backend" => Ok(png_payload()),
        "dynamic_ui" => Ok(dynamic_ui_payload()),
        "ts_compatibility" => Ok(ts_compatibility_payload()),
        "build_graph" => Ok(build_graph_payload()),
        _ => Err(DslDiagnostic::error(
            DSL_CONTRACT_ROW_UNSUPPORTED,
            "$.contract_row_id",
            format!("unsupported contract row {contract_row_id}"),
            None,
        )),
    }
}

fn schema_payload() -> DslSurfacePayload {
    DslSurfacePayload::SchemaVersioning {
        version: SchemaVersion::new(1, 0),
        feature: FeatureFlag {
            capability_id: CapabilityId::new("dsl.schema.versioning"),
            enabled_by_default: true,
        },
        wire: WireFormatVersions {
            binary_version: 1,
            json_debug_version: 1,
        },
    }
}

fn component_identity_payload() -> DslSurfacePayload {
    let component = ComponentId::new("component.component_identity");
    let node = NodeId::new("node.component_identity.root");
    DslSurfacePayload::ComponentIdentity {
        document: UiDocument {
            root: RootNode { node_id: node },
            component_id: component,
        },
        identity: ElementIdentity {
            component_id: component,
            node_id: node,
            kind: ElementKind::View,
        },
        hot_reload_identity: HotReloadIdentityId::new("hot_reload.component_identity"),
        dynamic_identity: DynamicIdentityId::new("dynamic.component_identity"),
    }
}

fn tree_structure_payload() -> DslSurfacePayload {
    let component = ComponentId::new("component.tree_structure");
    let root = NodeId::new("node.tree_structure.root");
    let child = NodeId::new("node.tree_structure.child");
    let state = StateId::new("state.tree_structure.context");
    DslSurfacePayload::TreeStructure {
        document: UiDocument {
            root: RootNode { node_id: root },
            component_id: component,
        },
        nodes: vec![
            UiNode {
                node_id: root,
                kind: ElementKind::View,
                state_id: Some(state),
            },
            UiNode {
                node_id: child,
                kind: ElementKind::Text,
                state_id: None,
            },
        ],
        children: vec![TreeChild {
            parent_id: root,
            child_id: child,
            order: ChildOrder(0),
            key: Some(valdi_rust_ir::ids::KeyId::new("key.tree_structure.child")),
        }],
        fragment: FragmentNode {
            node_id: NodeId::new("node.tree_structure.fragment"),
        },
        slot: SlotNode {
            node_id: NodeId::new("node.tree_structure.slot"),
            slot_name: "leading",
        },
        portal: PortalNode {
            node_id: NodeId::new("node.tree_structure.portal"),
            target: "overlay",
        },
        context: ContextScope {
            node_id: root,
            state_id: state,
        },
        destruction_policy: DestructionPolicy::PreserveForPool,
        pooling_policy: PoolingPolicy::ReusableByKind(ElementKind::View),
    }
}

fn element_taxonomy_payload() -> DslSurfacePayload {
    DslSurfacePayload::ElementTaxonomy {
        elements: REQUIRED_ELEMENT_KINDS.to_vec(),
        identity: ElementIdentity {
            component_id: ComponentId::new("component.element_taxonomy"),
            node_id: NodeId::new("node.element_taxonomy.native"),
            kind: ElementKind::NativeView,
        },
        native_view_ref: NativeViewRef {
            native_view_id: NativeViewId::new("native.element_taxonomy.camera"),
            host_node_id: NodeId::new("node.element_taxonomy.native"),
        },
    }
}

fn text_payload() -> DslSurfacePayload {
    DslSurfacePayload::Text {
        text: TextNode {
            value: "Rust DSL text",
            wrapping: Wrapping::Word,
        },
        rich_text: RichText {
            spans: RICH_TEXT_SPANS,
        },
        text_input: TextInputState {
            value_binding: BindingId::new("binding.text.value"),
            selection: Some(SelectionRange { start: 0, end: 4 }),
            composition: Some(CompositionRange { start: 1, end: 2 }),
        },
        truncation: Truncation::Tail,
        measurement_hook: TextMeasurementHook {
            hook_name: "measure.title",
        },
    }
}

fn assets_payload() -> DslSurfacePayload {
    let asset_id = AssetId::new("asset.logo");
    DslSurfacePayload::Assets {
        asset: AssetRef {
            id: asset_id,
            source: "logo.png",
            resize_mode: AssetResizeMode::Contain,
        },
        variant: AssetVariant {
            platform: "ios",
            scale: 2,
            path: "logo@2x.png",
        },
        remote: RemoteAssetRef {
            url: "https://example.invalid/logo.png",
        },
        data: DataAssetRef {
            media_type: "image/png",
            bytes_label: "logo.bytes",
        },
        loading_state: AssetLoadingState::Failed(AssetError {
            code: "asset.offline",
            recoverable: true,
        }),
        animated: AnimatedAssetMetadata {
            frame_count: 12,
            duration_ms: 750,
        },
        cache_key: AssetCacheKey {
            id: asset_id,
            variant: "ios@2x",
        },
    }
}

fn events_payload() -> DslSurfacePayload {
    let node_id = NodeId::new("node.events_gestures.root");
    let action_id = ActionId::new("action.events_gestures.tap");
    DslSurfacePayload::EventsGestures {
        bindings: vec![
            EventBinding {
                node_id,
                kind: EventKind::Tap,
                action_id,
            },
            EventBinding {
                node_id,
                kind: EventKind::Press,
                action_id,
            },
            EventBinding {
                node_id,
                kind: EventKind::LongPress,
                action_id,
            },
            EventBinding {
                node_id,
                kind: EventKind::Pan,
                action_id,
            },
            EventBinding {
                node_id: NodeId::new("node.events_gestures.scroll"),
                kind: EventKind::Scroll,
                action_id,
            },
            EventBinding {
                node_id: NodeId::new("node.events_gestures.input"),
                kind: EventKind::Focus,
                action_id,
            },
            EventBinding {
                node_id: NodeId::new("node.events_gestures.input"),
                kind: EventKind::Blur,
                action_id,
            },
            EventBinding {
                node_id: NodeId::new("node.events_gestures.input"),
                kind: EventKind::Input,
                action_id,
            },
            EventBinding {
                node_id: NodeId::new("node.events_gestures.input"),
                kind: EventKind::KeyboardSubmit,
                action_id,
            },
            EventBinding {
                node_id,
                kind: EventKind::Layout,
                action_id,
            },
            EventBinding {
                node_id: NodeId::new("node.events_gestures.draw"),
                kind: EventKind::Draw,
                action_id,
            },
            EventBinding {
                node_id,
                kind: EventKind::Visibility,
                action_id,
            },
            EventBinding {
                node_id,
                kind: EventKind::FrameObserver,
                action_id,
            },
            EventBinding {
                node_id: NodeId::new("node.events_gestures.native"),
                kind: EventKind::CustomNative(CustomNativeEventSpec {
                    event_name: "camera.ready",
                }),
                action_id,
            },
        ],
        scroll: ScrollEventPayload {
            offset_x: 1.0,
            offset_y: 2.0,
        },
        input: InputEventPayload { value: "Ada" },
        keyboard: KeyboardSubmitPayload { value: "Ada" },
        layout: LayoutEventPayload {
            width: 320.0,
            height: 180.0,
        },
        draw: DrawEventPayload { frame_number: 1 },
        visibility: VisibilityEventPayload { visible: true },
        frame: FrameObserverPayload { frame_number: 2 },
    }
}

fn actions_payload() -> DslSurfacePayload {
    let state_id = StateId::new("state.actions_state.form");
    DslSurfacePayload::ActionsState {
        action: ActionDefinition {
            id: ActionId::new("action.actions_state.save"),
            kind: ActionKind::Async,
            state_scope: state_id,
        },
        result: ActionResult::Failed(ActionError {
            code: "validation.failed",
        }),
        cancellation: CancellationPolicy::CancelByActionId,
        invalidation: InvalidationTarget { state_id },
        scheduling: SchedulingPolicy::Deferred,
        coalescing_key: CoalescingKey { value: "save.form" },
        state_slot: StateSlot {
            id: state_id,
            name: "form",
        },
    }
}

fn bindings_payload() -> DslSurfacePayload {
    DslSurfacePayload::BindingsExpressions {
        expressions: vec![
            bindings::field("app.user.name"),
            bindings::literal(LiteralValue::Text("Ada")),
            bindings::literal(LiteralValue::Null),
            bindings::logic(LogicOp::And),
            bindings::comparison(ComparisonOp::Equal),
            bindings::list_projection("app.items"),
            bindings::computed("count"),
            bindings::platform_constant("platform.os"),
        ],
        source_span: bindings::source_span(SourceSpanId::new("binding.dsl:1:1")),
    }
}

fn animations_payload() -> DslSurfacePayload {
    let timing = Timing {
        duration_ms: 250,
        delay_ms: 10,
        easing: Easing::EaseOut,
    };
    DslSurfacePayload::Animations {
        animation: Animation {
            node_id: NodeId::new("node.animations.box"),
            kind: AnimationKind::Property,
            timing,
        },
        lifecycle: AnimationLifecycle::Start,
        layout_animation: LayoutAnimation {
            property: AnimatedProperty::Layout,
            timing,
        },
        transaction_group: TransactionGroup {
            name: "animation.batch",
        },
    }
}

fn native_modules_payload() -> DslSurfacePayload {
    DslSurfacePayload::NativeModules {
        contract: modules::module_contract(
            ModuleId::new("module.storage"),
            DispatchTarget::RustHost,
        ),
        type_matrix_entry: modules::type_matrix_entry(
            ContractType::Result,
            ModuleFactoryTarget::RustHost,
        ),
    }
}

fn native_views_payload() -> DslSurfacePayload {
    DslSurfacePayload::NativeViews {
        contract: native_views::native_view_contract(
            NativeViewId::new("native.camera"),
            LifecycleHook::Create,
        ),
        attribute: native_views::native_view_attribute("session", "CameraSession"),
        event: native_views::native_view_event("ready", "CameraReady"),
        measurement: MeasurementContract {
            supports_intrinsic_size: true,
        },
        reuse_policy: ReusePolicy::ReuseByContract,
        accessibility: NativeViewAccessibility {
            role: AccessibilityRole::Image,
        },
        fallback: FallbackSpec {
            png_fallback: true,
            platform_extension: Some(platform_extensions::ios_safe_area(CapabilityId::new(
                "ios.camera.safe_area",
            ))),
        },
    }
}

fn accessibility_payload() -> DslSurfacePayload {
    DslSurfacePayload::Accessibility {
        node: AccessibilityNode {
            node_id: NodeId::new("node.accessibility.button"),
            role: AccessibilityRole::Button,
        },
        label: AccessibilityLabel("Save"),
        hint: valdi_rust_ir::accessibility::AccessibilityHint("Saves the form"),
        value: AccessibilityValue("Enabled"),
        state: AccessibilityState {
            disabled: false,
            selected: true,
            checked: false,
        },
        action: AccessibilityAction { name: "activate" },
        focus_order: FocusOrder(1),
        grouping: GroupingBehavior::GroupChildren,
        hidden: HiddenState(false),
        override_spec: AccessibilityOverride {
            platform: "ios",
            label: Some(AccessibilityLabel("Save form")),
        },
    }
}

fn hot_reload_payload() -> DslSurfacePayload {
    DslSurfacePayload::HotReload {
        patch: HotReloadPatch {
            identity: PatchIdentity {
                id: HotReloadIdentityId::new("hot_reload.patch.root"),
                source_span_id: Some(SourceSpanId::new("hot_reload.dsl:1:1")),
            },
            compatibility: CompatibilityClass::UiOnly,
        },
        asset: valdi_rust_ir::hot_reload::AssetPatchRef(AssetId::new("asset.patch.logo")),
        binding: valdi_rust_ir::hot_reload::BindingPatchRef(BindingId::new("binding.patch.title")),
        action: valdi_rust_ir::hot_reload::ActionPatchRef(ActionId::new("action.patch.save")),
        module: valdi_rust_ir::hot_reload::ModuleRefPatch(ModuleId::new("module.patch.storage")),
        native_view: valdi_rust_ir::hot_reload::NativeViewRefPatch(NativeViewId::new(
            "native.patch.camera",
        )),
    }
}

fn diagnostics_payload() -> DslSurfacePayload {
    DslSurfacePayload::Diagnostics {
        diagnostic: Diagnostic {
            id: DiagnosticId::new("diagnostic.unsupported_attr"),
            severity: DiagnosticSeverity::Error,
        },
        schema_path: SchemaPath("$.nodes[].attributes.text"),
        backend_path: BackendPath(BackendPathId::new("backend.attr.text")),
        capability_error: CapabilityError {
            capability_id: CapabilityId::new("backend.text"),
            reason: "unsupported text capability",
        },
        unsupported_surface: UnsupportedSurface {
            schema_path: SchemaPath("$.nodes[].events[]"),
            source_span_id: Some(SourceSpanId::new("diagnostics.dsl:1:1")),
        },
        fixture_id: FixtureId::new("contract.diagnostics.v1"),
        action_trace: valdi_rust_ir::diagnostics::ActionTraceId(ActionId::new(
            "action.diagnostics.trace",
        )),
        module_trace: valdi_rust_ir::diagnostics::ModuleTraceId(ModuleId::new(
            "module.diagnostics.trace",
        )),
    }
}

fn web_dom_payload() -> DslSurfacePayload {
    DslSurfacePayload::WebDom {
        dom: DomMapping {
            tag_name: "button",
            class_emission: ClassEmission::Generated,
        },
        css: CssStyleMapping {
            property: "display",
            value_token: "flex",
        },
        event: DomEventMapping {
            event_name: "click",
            event_kind: EventKind::Tap,
        },
        text_measurement: TextMeasurementSpec {
            measurement_id: "web.text.measure",
        },
        browser_snapshot: BrowserSnapshotSpec {
            snapshot_name: "web_dom_contract",
        },
    }
}

fn png_payload() -> DslSurfacePayload {
    let layout = LayoutSnapshot {
        width: 320,
        height: 180,
    };
    let display_list = DisplayList {
        commands: PNG_COMMANDS,
    };
    DslSurfacePayload::PngBackend {
        plan: PngRenderPlan {
            layout_snapshot: layout,
            display_list,
        },
        layout,
        display_list,
        native_fallback: NativeViewFallback {
            label: "camera fallback",
        },
        web_fallback: WebViewFallback {
            label: "webview fallback",
        },
        accessibility: AccessibilityDebugMetadata {
            role: AccessibilityRole::Image,
            label: "chart",
        },
    }
}

fn dynamic_ui_payload() -> DslSurfacePayload {
    DslSurfacePayload::DynamicUi {
        producer: DynamicUiProducer {
            name: "rust_dsl_home",
            validation_state: DynamicUiValidationState::Validated,
        },
        input: DynamicUiInput::InMemory(InMemoryProducerTag { name: "dsl.memory" }),
        generated_fixture: GeneratedFixtureRef {
            fixture_name: "contract.dynamic_ui.v1",
        },
        debug_json: DebugJsonLabel {
            label: "//valdi_rust/fixtures:dynamic_ui_debug_json",
        },
        binary_bytes: BinaryBytesLabel {
            label: "//valdi_rust/fixtures:dynamic_ui_binary",
        },
    }
}

fn ts_compatibility_payload() -> DslSurfacePayload {
    DslSurfacePayload::TsCompatibility {
        coverage: TsxCoverageMap {
            feature: "tsx_to_ir_equivalence",
            covered: true,
        },
        compatibility: DirectRendererCompatibility::EquivalentIr,
        equivalence: TsxToIrEquivalence {
            fixture_name: "contract.ts_compatibility.v1",
        },
        dependency_check: RustPathDependencyCheck {
            excludes_typescript_runtime: true,
        },
    }
}

fn build_graph_payload() -> DslSurfacePayload {
    let dsl = BazelTargetRef::new("//valdi_rust/dsl:dsl");
    DslSurfacePayload::BuildGraph {
        target: dsl,
        edge: CrateGraphEdge {
            from: dsl,
            to: BazelTargetRef::new("//valdi_rust/ir:ir"),
        },
        generated_glue: GeneratedGlueLabel {
            label: BazelTargetRef::new("//valdi_rust/tests:rust_host_generated_glue_placeholder"),
        },
        platform_app: PlatformAppTarget {
            platform: "rust_host",
            label: dsl,
        },
        compatibility_label: CompatibilityLabel {
            label: BazelTargetRef::new("//valdi_rust/dsl:dsl_tests"),
        },
        forbidden_dependency: ForbiddenDependencyCheck {
            dependency_name: "typescript_runtime",
            forbidden: true,
        },
    }
}

fn sample_layout() -> LayoutStyle {
    LayoutStyle {
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
        measure: MeasurePolicy::Delegate("dsl.measure"),
        safe_area: SafeAreaEdges {
            top: true,
            right: false,
            bottom: true,
            left: false,
        },
        scroll_sizing: Some(ScrollSizing {
            horizontal: Length::Auto,
            vertical: Length::Points(480.0),
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
    }
}

fn sample_style() -> Style {
    let extension = PlatformExtension::new(
        ExtensionNamespace::Web,
        ExtensionVersion::new(1, 0),
        CapabilityId::new("web.dsl.style"),
        PlatformExtensionPayload::Web(WebPlatformExtension::CssCustomProperty("--dsl-accent")),
    );
    Style {
        background: Background::Solid(Color::rgba(10, 40, 90, 255)),
        opacity: 0.95,
        border: Some(Border {
            color: Color::rgba(0, 0, 0, 255),
            width: 1.0,
        }),
        radius: Some(Radius {
            top_left: 4.0,
            top_right: 4.0,
            bottom_right: 4.0,
            bottom_left: 4.0,
        }),
        shadow: Some(Shadow {
            color: Color::rgba(0, 0, 0, 64),
            offset_x: 0.0,
            offset_y: 2.0,
            blur: 8.0,
        }),
        overflow: Overflow::Hidden,
        visibility: Visibility::Visible,
        display: Display::Flex,
        class_metadata: Some(ClassMetadata {
            class_name: "dsl-card",
        }),
        platform_extension: Some(StylePlatformExtension { extension }),
    }
}

static RICH_TEXT_SPANS: &[TextSpan] = &[
    TextSpan {
        value: "Rust",
        font: FontStyle {
            family: "System",
            size_points: 14,
            bold: true,
            italic: false,
        },
        link: None,
    },
    TextSpan {
        value: "DSL",
        font: FontStyle {
            family: "System",
            size_points: 14,
            bold: false,
            italic: false,
        },
        link: Some(LinkTarget {
            href: "https://example.invalid/dsl",
            source_span_id: SourceSpanId::new("text.dsl:1:5"),
        }),
    },
];

static PNG_COMMANDS: &[DisplayCommand] = &[
    DisplayCommand::Save,
    DisplayCommand::DrawText("title"),
    DisplayCommand::DrawImage("logo"),
    DisplayCommand::Restore,
];

#[allow(dead_code)]
const _GESTURE_COVERAGE: &[GestureKind] = &[
    GestureKind::Tap,
    GestureKind::Press,
    GestureKind::LongPress,
    GestureKind::Pan,
];
