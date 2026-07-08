use valdi_rust_ir::{
    accessibility::{
        AccessibilityHint, AccessibilityLabel, AccessibilityNode, AccessibilityRole,
        AccessibilityState, AccessibilityValue,
    },
    ids::NodeId,
};

pub fn accessibility_node(node_id: NodeId, role: AccessibilityRole) -> AccessibilityNode {
    AccessibilityNode { node_id, role }
}

pub fn accessibility_text(
    label: &'static str,
    hint: &'static str,
    value: &'static str,
) -> (AccessibilityLabel, AccessibilityHint, AccessibilityValue) {
    (
        AccessibilityLabel(label),
        AccessibilityHint(hint),
        AccessibilityValue(value),
    )
}

pub const fn accessibility_state(
    disabled: bool,
    selected: bool,
    checked: bool,
) -> AccessibilityState {
    AccessibilityState {
        disabled,
        selected,
        checked,
    }
}
