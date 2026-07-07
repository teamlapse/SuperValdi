//! Tree structure schema.
//!
//! ```
//! use valdi_rust_ir::{ids::NodeId, tree::{DestructionPolicy, RootNode}};
//!
//! let root = RootNode { node_id: NodeId::new("root") };
//! assert_eq!(root.node_id.as_str(), "root");
//! assert_eq!(DestructionPolicy::DestroySubtree, DestructionPolicy::DestroySubtree);
//! ```

use crate::elements::ElementKind;
use crate::ids::{ComponentId, KeyId, NodeId, StateId};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiDocument {
    pub root: RootNode,
    pub component_id: ComponentId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RootNode {
    pub node_id: NodeId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiNode {
    pub node_id: NodeId,
    pub kind: ElementKind,
    pub state_id: Option<StateId>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TreeChild {
    pub parent_id: NodeId,
    pub child_id: NodeId,
    pub order: ChildOrder,
    pub key: Option<KeyId>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ChildOrder(pub u32);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FragmentNode {
    pub node_id: NodeId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SlotNode {
    pub node_id: NodeId,
    pub slot_name: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PortalNode {
    pub node_id: NodeId,
    pub target: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ContextScope {
    pub node_id: NodeId,
    pub state_id: StateId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DestructionPolicy {
    DestroySubtree,
    PreserveForPool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PoolingPolicy {
    NotReusable,
    ReusableByKind(ElementKind),
}
