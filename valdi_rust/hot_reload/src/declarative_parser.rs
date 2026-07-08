use valdi_rust_ir::{
    accessibility::{AccessibilityLabel, AccessibilityNode, AccessibilityRole},
    assets::{AssetRef, AssetResizeMode},
    bindings::FieldPath,
    elements::ElementKind,
    events::{EventBinding, EventKind},
    ids::{
        ActionId, AssetId, BindingId, HotReloadIdentityId, KeyId, ModuleId, NativeViewId, NodeId,
        SourceSpanId,
    },
    layout::{
        ClipBehavior, FlexDirection, FlexLayout, LayoutStyle, Length, MeasurePolicy, RtlMode,
        SafeAreaEdges, Transform, ZOrder,
    },
    styling::{Color, Style},
    text::{TextNode, Wrapping},
    tree::ChildOrder,
};
use valdi_rust_runtime::{
    RuntimeBinding, RuntimeExpression, RuntimeFieldExpression, RuntimeLiteralExpression,
};

use crate::{
    diagnostics::{HotReloadResult, HOT_RELOAD_PARSE_ERROR},
    patch::{
        AccessibilityPatch, ActionBodyPatch, AssetPatch, BindingPatch, EventPatch, HotReloadEdit,
        HotReloadPatchIntent, LayoutPatch, ModuleRefPatchIntent, NativeViewRefPatchIntent,
        StylePatch, TextPatch, UiTreePatch,
    },
    HotReloadDiagnostic,
};

pub fn parse_declarative_patch(input: &'static str) -> HotReloadResult<HotReloadPatchIntent> {
    let mut parser = FieldParser::new(input);
    let kind = parser.next_kind()?;
    let source_span_id = parser.source_span_id();
    let patch_id = HotReloadIdentityId::new(parser.value_or("patch_id", "patch.hot_reload"));
    let edit = match kind {
        "tree" => HotReloadEdit::UiTree(UiTreePatch {
            parent_id: NodeId::new(parser.required("parent")?),
            node_id: NodeId::new(parser.required("node")?),
            element_kind: parse_element_kind(parser.required("kind")?)?,
            order: ChildOrder(parse_u32(
                parser.value_or("order", "0"),
                "$.patch.ui_tree.order",
            )?),
            key: parser.optional("key").map(KeyId::new),
        }),
        "style" => HotReloadEdit::Style(StylePatch {
            node_id: NodeId::new(parser.required("node")?),
            style: Style::minimal(parse_color(parser.required("color")?)?),
        }),
        "layout" => HotReloadEdit::Layout(LayoutPatch {
            node_id: NodeId::new(parser.required("node")?),
            layout: layout_style(parser.value_or("direction", "column"))?,
        }),
        "text" => HotReloadEdit::Text(TextPatch {
            node_id: NodeId::new(parser.required("node")?),
            text: TextNode {
                value: parser.required("value")?,
                wrapping: Wrapping::Word,
            },
        }),
        "asset" => HotReloadEdit::Asset(AssetPatch {
            node_id: NodeId::new(parser.required("node")?),
            asset: AssetRef {
                id: AssetId::new(parser.required("asset")?),
                source: parser.required("source")?,
                resize_mode: AssetResizeMode::Contain,
            },
        }),
        "binding" => {
            let binding_id = BindingId::new(parser.required("binding")?);
            let expression = match parser.optional("literal") {
                Some(value) => RuntimeExpression::Literal(RuntimeLiteralExpression {
                    value: valdi_rust_runtime::literal_value(
                        valdi_rust_ir::bindings::LiteralValue::Text(value),
                    ),
                    source_span_id,
                }),
                None => RuntimeExpression::Field(RuntimeFieldExpression {
                    path: FieldPath::new(parser.required("path")?),
                    source_span_id,
                }),
            };
            HotReloadEdit::Binding(BindingPatch {
                binding: RuntimeBinding {
                    binding_id,
                    expression,
                    source_span_id,
                },
            })
        }
        "event" => HotReloadEdit::Event(EventPatch {
            binding: EventBinding {
                node_id: NodeId::new(parser.required("node")?),
                kind: parse_event_kind(parser.required("event")?)?,
                action_id: ActionId::new(parser.required("action")?),
            },
        }),
        "accessibility" => HotReloadEdit::Accessibility(AccessibilityPatch {
            node: AccessibilityNode {
                node_id: NodeId::new(parser.required("node")?),
                role: parse_accessibility_role(parser.value_or("role", "button"))?,
            },
            label: AccessibilityLabel(parser.required("label")?),
        }),
        "module_ref" => HotReloadEdit::ModuleRef(ModuleRefPatchIntent {
            module_id: ModuleId::new(parser.required("module")?),
            expected_shape: parser.required("shape")?,
        }),
        "native_view_ref" => HotReloadEdit::NativeViewRef(NativeViewRefPatchIntent {
            native_view_id: NativeViewId::new(parser.required("native_view")?),
            expected_shape: parser.required("shape")?,
        }),
        "action_body" => HotReloadEdit::ActionBody(ActionBodyPatch {
            action_id: ActionId::new(parser.required("action")?),
            edit_token: parser.required("edit")?,
        }),
        _ => return Err(parse_error("$.patch.kind", "unknown patch kind", None)),
    };
    Ok(HotReloadPatchIntent::new(patch_id, edit, source_span_id))
}

struct FieldParser {
    input: &'static str,
}

impl FieldParser {
    const fn new(input: &'static str) -> Self {
        Self { input }
    }

    fn next_kind(&mut self) -> HotReloadResult<&'static str> {
        self.input
            .split_whitespace()
            .next()
            .ok_or_else(|| parse_error("$.patch.kind", "missing patch kind", None))
    }

    fn required(&self, key: &'static str) -> HotReloadResult<&'static str> {
        self.optional(key)
            .ok_or_else(|| parse_error("$.patch.fields", "missing required field", None))
    }

    fn optional(&self, key: &'static str) -> Option<&'static str> {
        self.input.split_whitespace().skip(1).find_map(|token| {
            let (candidate, value) = token.split_once('=')?;
            (candidate == key).then_some(value)
        })
    }

    fn value_or(&self, key: &'static str, fallback: &'static str) -> &'static str {
        self.optional(key).unwrap_or(fallback)
    }

    fn source_span_id(&self) -> Option<SourceSpanId> {
        self.optional("span").map(SourceSpanId::new)
    }
}

fn parse_element_kind(value: &'static str) -> HotReloadResult<ElementKind> {
    match value {
        "view" => Ok(ElementKind::View),
        "layout" => Ok(ElementKind::Layout),
        "scroll" => Ok(ElementKind::Scroll),
        "image" => Ok(ElementKind::Image),
        "text" => Ok(ElementKind::Text),
        "rich_text" => Ok(ElementKind::RichText),
        "text_input" => Ok(ElementKind::TextInput),
        "control" => Ok(ElementKind::Control),
        "list" => Ok(ElementKind::List),
        "webview" => Ok(ElementKind::WebView),
        "native_view" => Ok(ElementKind::NativeView),
        "drawing_host" => Ok(ElementKind::DrawingHost),
        _ => Err(parse_error(
            "$.patch.ui_tree.kind",
            "unknown element kind",
            None,
        )),
    }
}

fn parse_event_kind(value: &'static str) -> HotReloadResult<EventKind> {
    match value {
        "tap" => Ok(EventKind::Tap),
        "press" => Ok(EventKind::Press),
        "input" => Ok(EventKind::Input),
        "layout" => Ok(EventKind::Layout),
        "draw" => Ok(EventKind::Draw),
        "visibility" => Ok(EventKind::Visibility),
        "frame_observer" => Ok(EventKind::FrameObserver),
        _ => Err(parse_error(
            "$.patch.event.kind",
            "unknown event kind",
            None,
        )),
    }
}

fn parse_accessibility_role(value: &'static str) -> HotReloadResult<AccessibilityRole> {
    match value {
        "button" => Ok(AccessibilityRole::Button),
        "image" => Ok(AccessibilityRole::Image),
        "text" => Ok(AccessibilityRole::Text),
        "header" => Ok(AccessibilityRole::Header),
        "input" => Ok(AccessibilityRole::Input),
        "none" => Ok(AccessibilityRole::None),
        _ => Err(parse_error(
            "$.patch.accessibility.role",
            "unknown accessibility role",
            None,
        )),
    }
}

fn parse_color(value: &'static str) -> HotReloadResult<Color> {
    let mut parts = value.split(',');
    let red = parse_u8(parts.next().unwrap_or(""), "$.patch.style.color.red")?;
    let green = parse_u8(parts.next().unwrap_or(""), "$.patch.style.color.green")?;
    let blue = parse_u8(parts.next().unwrap_or(""), "$.patch.style.color.blue")?;
    let alpha = parse_u8(parts.next().unwrap_or("255"), "$.patch.style.color.alpha")?;
    if parts.next().is_some() {
        return Err(parse_error(
            "$.patch.style.color",
            "color must be red,green,blue,alpha",
            None,
        ));
    }
    Ok(Color::rgba(red, green, blue, alpha))
}

fn parse_u8(value: &str, path: &'static str) -> HotReloadResult<u8> {
    value
        .parse::<u8>()
        .map_err(|_| parse_error(path, "expected u8", None))
}

fn parse_u32(value: &str, path: &'static str) -> HotReloadResult<u32> {
    value
        .parse::<u32>()
        .map_err(|_| parse_error(path, "expected u32", None))
}

fn layout_style(direction: &'static str) -> HotReloadResult<LayoutStyle> {
    let direction = match direction {
        "row" => FlexDirection::Row,
        "column" => FlexDirection::Column,
        _ => {
            return Err(parse_error(
                "$.patch.layout.direction",
                "unknown flex direction",
                None,
            ))
        }
    };
    Ok(LayoutStyle {
        flex: FlexLayout {
            direction,
            grow: 1.0,
            shrink: 1.0,
            basis: Length::Auto,
        },
        absolute: None,
        measure: MeasurePolicy::Intrinsic,
        safe_area: SafeAreaEdges {
            top: false,
            right: false,
            bottom: false,
            left: false,
        },
        scroll_sizing: None,
        z_order: ZOrder(0),
        clipping: ClipBehavior::Visible,
        transform: Transform {
            translate_x: 0.0,
            translate_y: 0.0,
            scale: 1.0,
            rotate_degrees: 0.0,
        },
        rtl: RtlMode::Inherit,
    })
}

fn parse_error(
    path: &'static str,
    reason: &'static str,
    source_span_id: Option<SourceSpanId>,
) -> HotReloadDiagnostic {
    HotReloadDiagnostic::error(HOT_RELOAD_PARSE_ERROR, path, reason, reason, source_span_id)
}
