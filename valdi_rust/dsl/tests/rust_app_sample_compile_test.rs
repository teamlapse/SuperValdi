use valdi_rust_dsl::{
    accessibility::{accessibility_node, accessibility_text},
    attributes::DslAttribute,
    elements,
    fixtures::payload_for_contract_row,
    native_views::{native_view_attribute, native_view_event},
    platform_extensions,
    source::source_span,
    tree::{DslSurfacePayload, REQUIRED_ELEMENT_KINDS},
};
use valdi_rust_ir::{
    accessibility::AccessibilityRole,
    actions::ActionKind,
    assets::{AssetRef, AssetResizeMode},
    elements::ElementKind,
    events::{CustomNativeEventSpec, EventKind},
    ids::{ActionId, AssetId, CapabilityId, NativeViewId, NodeId, StateId},
    native_views::LifecycleHook,
    styling::{Color, Style},
    text::{TextInputState, TextNode, Wrapping},
};

#[test]
fn rust_app_developer_dsl_sample_compiles_without_compatibility_renderer_dependencies(
) -> Result<(), valdi_rust_dsl::DslDiagnostic> {
    let root_span = source_span("app:1:1", "app.rs", 1, 1);
    let title_span = source_span("app:2:5", "app.rs", 2, 5);
    let image_span = source_span("app:3:5", "app.rs", 3, 5);
    let input_span = source_span("app:4:5", "app.rs", 4, 5);
    let native_span = source_span("app:5:5", "app.rs", 5, 5);

    let action = elements::action(
        ActionId::new("action.save"),
        StateId::new("state.form"),
        ActionKind::Sync,
    );

    let root = elements::view(NodeId::new("node.root"), root_span)
        .attr(DslAttribute::Style(Style::minimal(Color::rgba(
            20, 40, 60, 255,
        ))))?
        .on(EventKind::Tap, action.id)?
        .build();
    let title = elements::text(NodeId::new("node.title"), title_span)
        .attr(DslAttribute::Text(TextNode {
            value: "Hello from Rust",
            wrapping: Wrapping::Word,
        }))?
        .attr(DslAttribute::Accessibility(accessibility_node(
            NodeId::new("node.title"),
            AccessibilityRole::Header,
        )))?
        .build();
    let image = elements::image(NodeId::new("node.logo"), image_span)
        .attr(DslAttribute::Asset(AssetRef {
            id: AssetId::new("asset.logo"),
            source: "logo.png",
            resize_mode: AssetResizeMode::Contain,
        }))?
        .build();
    let input = elements::text_input(NodeId::new("node.name_input"), input_span)
        .attr(DslAttribute::TextInput(TextInputState {
            value_binding: valdi_rust_ir::ids::BindingId::new("binding.name"),
            selection: None,
            composition: None,
        }))?
        .on(EventKind::Input, ActionId::new("action.name_changed"))?
        .build();
    let native = elements::native_view(NodeId::new("node.camera"), native_span)
        .attr(DslAttribute::NativeView(native_view_attribute(
            "session",
            "CameraSession",
        )))?
        .on(
            EventKind::CustomNative(CustomNativeEventSpec {
                event_name: "camera.ready",
            }),
            ActionId::new("action.camera_ready"),
        )?
        .build();

    let extension =
        platform_extensions::web_css_property(CapabilityId::new("web.app.accent"), "--app-accent");
    let (label, hint, value) = accessibility_text("Save", "Stores the form", "Enabled");
    let native_event = native_view_event("ready", "CameraReady");
    let native_contract = valdi_rust_dsl::native_views::native_view_contract(
        NativeViewId::new("native.camera"),
        LifecycleHook::Create,
    );

    assert_eq!(root.events.len(), 1);
    assert_eq!(title.attributes.len(), 2);
    assert_eq!(image.attributes.len(), 1);
    assert_eq!(input.events[0].kind, EventKind::Input);
    assert_eq!(native.events[0].action_id.as_str(), "action.camera_ready");
    assert_eq!(extension.version.major, 1);
    assert_eq!(label.0, "Save");
    assert_eq!(hint.0, "Stores the form");
    assert_eq!(value.0, "Enabled");
    assert_eq!(native_event.name, "ready");
    assert_eq!(native_contract.lifecycle, LifecycleHook::Create);

    match payload_for_contract_row("dynamic_ui").expect("dynamic UI payload exists") {
        DslSurfacePayload::DynamicUi { producer, .. } => assert_eq!(producer.name, "rust_dsl_home"),
        other => panic!("unexpected payload: {other:?}"),
    }

    Ok(())
}

#[test]
fn every_required_element_kind_has_a_named_app_facing_builder() {
    for kind in REQUIRED_ELEMENT_KINDS {
        let node = match kind {
            ElementKind::View => elements::view(
                NodeId::new("node.builder.view"),
                source_span("builder:view", "app.rs", 1, 1),
            )
            .build(),
            ElementKind::Layout => elements::layout(
                NodeId::new("node.builder.layout"),
                source_span("builder:layout", "app.rs", 2, 1),
            )
            .build(),
            ElementKind::Scroll => elements::scroll(
                NodeId::new("node.builder.scroll"),
                source_span("builder:scroll", "app.rs", 3, 1),
            )
            .build(),
            ElementKind::Image => elements::image(
                NodeId::new("node.builder.image"),
                source_span("builder:image", "app.rs", 4, 1),
            )
            .build(),
            ElementKind::Text => elements::text(
                NodeId::new("node.builder.text"),
                source_span("builder:text", "app.rs", 5, 1),
            )
            .build(),
            ElementKind::RichText => elements::rich_text(
                NodeId::new("node.builder.rich_text"),
                source_span("builder:rich_text", "app.rs", 6, 1),
            )
            .build(),
            ElementKind::TextInput => elements::text_input(
                NodeId::new("node.builder.text_input"),
                source_span("builder:text_input", "app.rs", 7, 1),
            )
            .build(),
            ElementKind::Control => elements::control(
                NodeId::new("node.builder.control"),
                source_span("builder:control", "app.rs", 8, 1),
            )
            .build(),
            ElementKind::List => elements::list(
                NodeId::new("node.builder.list"),
                source_span("builder:list", "app.rs", 9, 1),
            )
            .build(),
            ElementKind::WebView => elements::webview(
                NodeId::new("node.builder.webview"),
                source_span("builder:webview", "app.rs", 10, 1),
            )
            .build(),
            ElementKind::NativeView => elements::native_view(
                NodeId::new("node.builder.native_view"),
                source_span("builder:native_view", "app.rs", 11, 1),
            )
            .build(),
            ElementKind::DrawingHost => elements::drawing_host(
                NodeId::new("node.builder.drawing_host"),
                source_span("builder:drawing_host", "app.rs", 12, 1),
            )
            .build(),
        };
        assert_eq!(node.node.kind, *kind);
    }
}
