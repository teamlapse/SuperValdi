use valdi_rust_ir::{
    actions::ActionDefinition,
    elements::ElementKind,
    events::{EventBinding, EventKind},
    ids::{ActionId, NodeId, SourceSpanId, StateId},
    tree::UiNode,
};

use crate::{
    attributes::{validate_attribute_for_element, DslAttribute},
    diagnostics::DslResult,
    events::validate_event_for_element,
    source::DslSourceSpan,
    tree::DslNode,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ElementBuilder {
    node: UiNode,
    source_span_id: SourceSpanId,
    attributes: Vec<DslAttribute>,
    events: Vec<EventBinding>,
}

impl ElementBuilder {
    pub fn new(kind: ElementKind, node_id: NodeId, source: DslSourceSpan) -> Self {
        Self {
            node: UiNode {
                node_id,
                kind,
                state_id: None,
            },
            source_span_id: source.id,
            attributes: Vec::new(),
            events: Vec::new(),
        }
    }

    pub fn state(mut self, state_id: StateId) -> Self {
        self.node.state_id = Some(state_id);
        self
    }

    pub fn attr(mut self, attribute: DslAttribute) -> DslResult<Self> {
        validate_attribute_for_element(self.node.kind, attribute, Some(self.source_span_id))?;
        self.attributes.push(attribute);
        Ok(self)
    }

    pub fn on(mut self, kind: EventKind, action_id: ActionId) -> DslResult<Self> {
        let binding = EventBinding {
            node_id: self.node.node_id,
            kind,
            action_id,
        };
        self.events.push(validate_event_for_element(
            self.node.kind,
            binding,
            Some(self.source_span_id),
        )?);
        Ok(self)
    }

    pub fn build(self) -> DslNode {
        DslNode {
            node: self.node,
            source_span_id: self.source_span_id,
            attributes: self.attributes,
            events: self.events,
        }
    }
}

pub fn view(node_id: NodeId, source: DslSourceSpan) -> ElementBuilder {
    ElementBuilder::new(ElementKind::View, node_id, source)
}

pub fn layout(node_id: NodeId, source: DslSourceSpan) -> ElementBuilder {
    ElementBuilder::new(ElementKind::Layout, node_id, source)
}

pub fn scroll(node_id: NodeId, source: DslSourceSpan) -> ElementBuilder {
    ElementBuilder::new(ElementKind::Scroll, node_id, source)
}

pub fn text(node_id: NodeId, source: DslSourceSpan) -> ElementBuilder {
    ElementBuilder::new(ElementKind::Text, node_id, source)
}

pub fn rich_text(node_id: NodeId, source: DslSourceSpan) -> ElementBuilder {
    ElementBuilder::new(ElementKind::RichText, node_id, source)
}

pub fn text_input(node_id: NodeId, source: DslSourceSpan) -> ElementBuilder {
    ElementBuilder::new(ElementKind::TextInput, node_id, source)
}

pub fn image(node_id: NodeId, source: DslSourceSpan) -> ElementBuilder {
    ElementBuilder::new(ElementKind::Image, node_id, source)
}

pub fn control(node_id: NodeId, source: DslSourceSpan) -> ElementBuilder {
    ElementBuilder::new(ElementKind::Control, node_id, source)
}

pub fn list(node_id: NodeId, source: DslSourceSpan) -> ElementBuilder {
    ElementBuilder::new(ElementKind::List, node_id, source)
}

pub fn native_view(node_id: NodeId, source: DslSourceSpan) -> ElementBuilder {
    ElementBuilder::new(ElementKind::NativeView, node_id, source)
}

pub fn webview(node_id: NodeId, source: DslSourceSpan) -> ElementBuilder {
    ElementBuilder::new(ElementKind::WebView, node_id, source)
}

pub fn drawing_host(node_id: NodeId, source: DslSourceSpan) -> ElementBuilder {
    ElementBuilder::new(ElementKind::DrawingHost, node_id, source)
}

pub fn action(
    id: ActionId,
    state_scope: StateId,
    kind: valdi_rust_ir::actions::ActionKind,
) -> ActionDefinition {
    ActionDefinition {
        id,
        kind,
        state_scope,
    }
}
