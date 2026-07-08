use valdi_rust_ir::{
    elements::ElementKind,
    events::{EventBinding, EventKind},
    ids::SourceSpanId,
};

use crate::diagnostics::{
    DslDiagnostic, DslResult, DSL_EVENT_ELEMENT_MISMATCH, DSL_SOURCE_SPAN_REQUIRED,
};

pub const REQUIRED_EVENT_TOKENS: &[&str] = &[
    "tap",
    "press",
    "long_press",
    "pan",
    "scroll",
    "focus",
    "blur",
    "input",
    "keyboard_submit",
    "layout",
    "draw",
    "visibility",
    "frame_observer",
    "custom_native",
];

pub fn event_token(kind: EventKind) -> &'static str {
    match kind {
        EventKind::Tap => "tap",
        EventKind::Press => "press",
        EventKind::LongPress => "long_press",
        EventKind::Pan => "pan",
        EventKind::Scroll => "scroll",
        EventKind::Focus => "focus",
        EventKind::Blur => "blur",
        EventKind::Input => "input",
        EventKind::KeyboardSubmit => "keyboard_submit",
        EventKind::Layout => "layout",
        EventKind::Draw => "draw",
        EventKind::Visibility => "visibility",
        EventKind::FrameObserver => "frame_observer",
        EventKind::CustomNative(_) => "custom_native",
    }
}

pub fn validate_event_for_element(
    element_kind: ElementKind,
    binding: EventBinding,
    source_span_id: Option<SourceSpanId>,
) -> DslResult<EventBinding> {
    if source_span_id.is_none() {
        return Err(DslDiagnostic::error(
            DSL_SOURCE_SPAN_REQUIRED,
            "$.nodes[].events[]",
            format!(
                "event {} requires a DSL source span",
                event_token(binding.kind)
            ),
            None,
        ));
    }

    let allowed = match binding.kind {
        EventKind::Tap | EventKind::Press | EventKind::LongPress | EventKind::Pan => {
            matches!(
                element_kind,
                ElementKind::View
                    | ElementKind::Control
                    | ElementKind::Image
                    | ElementKind::Text
                    | ElementKind::NativeView
            )
        }
        EventKind::Scroll => matches!(element_kind, ElementKind::Scroll | ElementKind::List),
        EventKind::Focus | EventKind::Blur | EventKind::Input | EventKind::KeyboardSubmit => {
            matches!(element_kind, ElementKind::TextInput)
        }
        EventKind::Layout | EventKind::Visibility | EventKind::FrameObserver => true,
        EventKind::Draw => matches!(element_kind, ElementKind::DrawingHost),
        EventKind::CustomNative(_) => {
            matches!(element_kind, ElementKind::NativeView | ElementKind::WebView)
        }
    };

    if allowed {
        return Ok(binding);
    }

    Err(DslDiagnostic::error(
        DSL_EVENT_ELEMENT_MISMATCH,
        "$.nodes[].events[]",
        format!(
            "event {} is not valid for element {}",
            event_token(binding.kind),
            element_kind.contract_token()
        ),
        source_span_id,
    ))
}
