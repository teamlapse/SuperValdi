use valdi_rust_ir::{
    elements::{ElementIdentity, ElementKind},
    ids::{ComponentId, NodeId, SourceSpanId, StateId},
    tree::{
        ContextScope, DestructionPolicy, FragmentNode, PortalNode, SlotNode, TreeChild, UiNode,
    },
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeNodeRole {
    Element(UiNode),
    Fragment(FragmentNode),
    Slot(SlotNode),
    Portal(PortalNode),
    Context(ContextScope),
}

impl RuntimeNodeRole {
    pub const fn node_id(self) -> NodeId {
        match self {
            Self::Element(node) => node.node_id,
            Self::Fragment(node) => node.node_id,
            Self::Slot(node) => node.node_id,
            Self::Portal(node) => node.node_id,
            Self::Context(scope) => scope.node_id,
        }
    }

    pub const fn element_kind(self) -> ElementKind {
        match self {
            Self::Element(node) => node.kind,
            Self::Fragment(_) | Self::Slot(_) | Self::Portal(_) | Self::Context(_) => {
                ElementKind::View
            }
        }
    }

    pub const fn state_id(self) -> Option<StateId> {
        match self {
            Self::Element(node) => node.state_id,
            Self::Context(scope) => Some(scope.state_id),
            Self::Fragment(_) | Self::Slot(_) | Self::Portal(_) => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeNodeDeclaration {
    pub role: RuntimeNodeRole,
    pub destruction_policy: DestructionPolicy,
    pub source_span_id: Option<SourceSpanId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeNode {
    pub role: RuntimeNodeRole,
    pub destruction_policy: DestructionPolicy,
    pub source_span_id: Option<SourceSpanId>,
}

impl RuntimeNode {
    pub const fn from_declaration(declaration: RuntimeNodeDeclaration) -> Self {
        Self {
            role: declaration.role,
            destruction_policy: declaration.destruction_policy,
            source_span_id: declaration.source_span_id,
        }
    }

    pub const fn node_id(&self) -> NodeId {
        self.role.node_id()
    }

    pub const fn state_id(&self) -> Option<StateId> {
        self.role.state_id()
    }

    pub const fn identity(&self, component_id: ComponentId) -> ElementIdentity {
        ElementIdentity {
            component_id,
            node_id: self.node_id(),
            kind: self.role.element_kind(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeChild {
    pub tree: TreeChild,
    pub source_span_id: Option<SourceSpanId>,
}
