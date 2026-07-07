//! Accessibility schema.
//!
//! ```
//! use valdi_rust_ir::{accessibility::{AccessibilityNode, AccessibilityRole}, ids::NodeId};
//!
//! let node = AccessibilityNode { node_id: NodeId::new("title"), role: AccessibilityRole::Header };
//! assert_eq!(node.role, AccessibilityRole::Header);
//! ```

use crate::ids::NodeId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AccessibilityNode {
    pub node_id: NodeId,
    pub role: AccessibilityRole,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccessibilityRole {
    None,
    Button,
    Image,
    Text,
    Header,
    Input,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AccessibilityLabel(pub &'static str);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AccessibilityHint(pub &'static str);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AccessibilityValue(pub &'static str);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AccessibilityState {
    pub disabled: bool,
    pub selected: bool,
    pub checked: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AccessibilityAction {
    pub name: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FocusOrder(pub u32);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GroupingBehavior {
    None,
    GroupChildren,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HiddenState(pub bool);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AccessibilityOverride {
    pub platform: &'static str,
    pub label: Option<AccessibilityLabel>,
}
