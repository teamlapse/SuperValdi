use valdi_rust_ir::ids::{ComponentId, KeyId, NodeId, SourceSpanId, StateId};

use crate::{document::RuntimeDocument, tree::RuntimeNodeRole};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ComponentIdentity {
    pub component_id: ComponentId,
    pub source_span_id: Option<SourceSpanId>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NodeIdentity {
    pub component_id: ComponentId,
    pub node_id: NodeId,
    pub role: RuntimeNodeRole,
    pub state_id: Option<StateId>,
    pub source_span_id: Option<SourceSpanId>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KeyIdentity {
    pub key_id: KeyId,
    pub parent_id: NodeId,
    pub child_id: NodeId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateSlot {
    pub component_id: ComponentId,
    pub node_id: NodeId,
    pub state_id: StateId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityTables {
    pub components: Vec<ComponentIdentity>,
    pub nodes: Vec<NodeIdentity>,
    pub keys: Vec<KeyIdentity>,
    pub state_slots: Vec<StateSlot>,
}

impl IdentityTables {
    pub fn from_document(document: &RuntimeDocument) -> Self {
        let components = vec![ComponentIdentity {
            component_id: document.component_id,
            source_span_id: document.source_span_id,
        }];
        let nodes = document
            .nodes
            .iter()
            .map(|node| NodeIdentity {
                component_id: document.component_id,
                node_id: node.node_id(),
                role: node.role,
                state_id: node.state_id(),
                source_span_id: node.source_span_id,
            })
            .collect();
        let keys = document
            .children
            .iter()
            .filter_map(|child| {
                child.tree.key.map(|key_id| KeyIdentity {
                    key_id,
                    parent_id: child.tree.parent_id,
                    child_id: child.tree.child_id,
                })
            })
            .collect();
        let state_slots = document
            .nodes
            .iter()
            .filter_map(|node| {
                node.state_id().map(|state_id| StateSlot {
                    component_id: document.component_id,
                    node_id: node.node_id(),
                    state_id,
                })
            })
            .collect();

        Self {
            components,
            nodes,
            keys,
            state_slots,
        }
    }

    pub fn state_slot(&self, state_id: StateId) -> Option<StateSlot> {
        self.state_slots
            .iter()
            .copied()
            .find(|slot| slot.state_id == state_id)
    }
}
