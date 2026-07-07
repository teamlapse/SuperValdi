//! Events and gestures schema.
//!
//! ```
//! use valdi_rust_ir::{events::{EventBinding, EventKind}, ids::{ActionId, NodeId}};
//!
//! let binding = EventBinding { node_id: NodeId::new("button"), kind: EventKind::Tap, action_id: ActionId::new("tap") };
//! assert_eq!(binding.kind, EventKind::Tap);
//! ```

use crate::ids::{ActionId, NodeId};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EventBinding {
    pub node_id: NodeId,
    pub kind: EventKind,
    pub action_id: ActionId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventKind {
    Tap,
    Press,
    LongPress,
    Pan,
    Scroll,
    Focus,
    Blur,
    Input,
    KeyboardSubmit,
    Layout,
    Draw,
    Visibility,
    FrameObserver,
    CustomNative(CustomNativeEventSpec),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GestureKind {
    Tap,
    Press,
    LongPress,
    Pan,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScrollEventPayload {
    pub offset_x: f32,
    pub offset_y: f32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InputEventPayload {
    pub value: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KeyboardSubmitPayload {
    pub value: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LayoutEventPayload {
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DrawEventPayload {
    pub frame_number: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VisibilityEventPayload {
    pub visible: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FrameObserverPayload {
    pub frame_number: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CustomNativeEventSpec {
    pub event_name: &'static str,
}
