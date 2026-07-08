use valdi_rust_ir::{
    accessibility::{AccessibilityLabel, AccessibilityNode},
    assets::AssetRef,
    elements::ElementKind,
    ids::SourceSpanId,
    layout::LayoutStyle,
    native_views::NativeViewAttribute,
    styling::Style,
    text::{TextInputState, TextNode},
    web_dom::DomMapping,
};

use crate::diagnostics::{
    DslDiagnostic, DslResult, DSL_ATTRIBUTE_ELEMENT_MISMATCH, DSL_SOURCE_SPAN_REQUIRED,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DslAttribute {
    Layout(LayoutStyle),
    Style(Style),
    Text(TextNode),
    TextInput(TextInputState),
    Asset(AssetRef),
    Accessibility(AccessibilityNode),
    AccessibilityLabel(AccessibilityLabel),
    NativeView(NativeViewAttribute),
    WebDom(DomMapping),
    TestIdentifier(&'static str),
}

impl DslAttribute {
    pub const fn path(self) -> &'static str {
        match self {
            Self::Layout(_) => "$.nodes[].attributes.layout",
            Self::Style(_) => "$.nodes[].attributes.style",
            Self::Text(_) => "$.nodes[].attributes.text",
            Self::TextInput(_) => "$.nodes[].attributes.text_input",
            Self::Asset(_) => "$.nodes[].attributes.asset",
            Self::Accessibility(_) => "$.nodes[].attributes.accessibility",
            Self::AccessibilityLabel(_) => "$.nodes[].attributes.accessibility_label",
            Self::NativeView(_) => "$.nodes[].attributes.native_view",
            Self::WebDom(_) => "$.nodes[].attributes.web_dom",
            Self::TestIdentifier(_) => "$.nodes[].attributes.test_identifier",
        }
    }

    pub const fn token(self) -> &'static str {
        match self {
            Self::Layout(_) => "layout",
            Self::Style(_) => "style",
            Self::Text(_) => "text",
            Self::TextInput(_) => "text_input",
            Self::Asset(_) => "asset",
            Self::Accessibility(_) => "accessibility",
            Self::AccessibilityLabel(_) => "accessibility_label",
            Self::NativeView(_) => "native_view",
            Self::WebDom(_) => "web_dom",
            Self::TestIdentifier(_) => "test_identifier",
        }
    }
}

pub const REQUIRED_ATTRIBUTE_TOKENS: &[&str] = &[
    "layout",
    "style",
    "text",
    "text_input",
    "asset",
    "accessibility",
    "accessibility_label",
    "native_view",
    "web_dom",
    "test_identifier",
];

pub fn validate_attribute_for_element(
    kind: ElementKind,
    attribute: DslAttribute,
    source_span_id: Option<SourceSpanId>,
) -> DslResult<()> {
    if source_span_id.is_none() {
        return Err(DslDiagnostic::error(
            DSL_SOURCE_SPAN_REQUIRED,
            attribute.path(),
            format!("attribute {} requires a DSL source span", attribute.token()),
            None,
        ));
    }

    let allowed = match attribute {
        DslAttribute::Layout(_)
        | DslAttribute::Style(_)
        | DslAttribute::Accessibility(_)
        | DslAttribute::AccessibilityLabel(_)
        | DslAttribute::TestIdentifier(_) => true,
        DslAttribute::Text(_) => matches!(kind, ElementKind::Text | ElementKind::RichText),
        DslAttribute::TextInput(_) => matches!(kind, ElementKind::TextInput),
        DslAttribute::Asset(_) => matches!(kind, ElementKind::Image),
        DslAttribute::NativeView(_) => matches!(kind, ElementKind::NativeView),
        DslAttribute::WebDom(_) => matches!(kind, ElementKind::WebView | ElementKind::View),
    };

    if allowed {
        return Ok(());
    }

    Err(DslDiagnostic::error(
        DSL_ATTRIBUTE_ELEMENT_MISMATCH,
        attribute.path(),
        format!(
            "attribute {} is not valid for element {}",
            attribute.token(),
            kind.contract_token()
        ),
        source_span_id,
    ))
}
