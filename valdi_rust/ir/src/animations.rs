//! Animation schema.
//!
//! ```
//! use valdi_rust_ir::{animations::{Animation, AnimationKind, Timing}, ids::NodeId};
//!
//! let animation = Animation { node_id: NodeId::new("box"), kind: AnimationKind::Property, timing: Timing::milliseconds(250) };
//! assert_eq!(animation.timing.duration_ms, 250);
//! ```

use crate::ids::NodeId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Animation {
    pub node_id: NodeId,
    pub kind: AnimationKind,
    pub timing: Timing,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnimationKind {
    Property,
    Layout,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnimationLifecycle {
    Start,
    End,
    Cancel,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Timing {
    pub duration_ms: u32,
    pub delay_ms: u32,
    pub easing: Easing,
}

impl Timing {
    pub const fn milliseconds(duration_ms: u32) -> Self {
        Self { duration_ms, delay_ms: 0, easing: Easing::Linear }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Easing {
    Linear,
    EaseIn,
    EaseOut,
    CubicBezier(u16, u16, u16, u16),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RepeatMode {
    Once,
    Count(u16),
    Forever,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FillMode {
    None,
    Forwards,
    Backwards,
    Both,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnimatedProperty {
    Opacity,
    Transform,
    Layout,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LayoutAnimation {
    pub property: AnimatedProperty,
    pub timing: Timing,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TransactionGroup {
    pub name: &'static str,
}
