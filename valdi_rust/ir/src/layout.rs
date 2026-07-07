//! Layout schema.
//!
//! ```
//! use valdi_rust_ir::layout::{FlexDirection, FlexLayout, Length};
//!
//! let layout = FlexLayout { direction: FlexDirection::Column, grow: 1.0, shrink: 1.0, basis: Length::Auto };
//! assert_eq!(layout.direction, FlexDirection::Column);
//! ```

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Length {
    Auto,
    Points(f32),
    Percent(f32),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlexLayout {
    pub direction: FlexDirection,
    pub grow: f32,
    pub shrink: f32,
    pub basis: Length,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FlexDirection {
    Row,
    Column,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AbsoluteLayout {
    pub top: Length,
    pub right: Length,
    pub bottom: Length,
    pub left: Length,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MeasurePolicy {
    Intrinsic,
    Delegate(&'static str),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SafeAreaEdges {
    pub top: bool,
    pub right: bool,
    pub bottom: bool,
    pub left: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScrollSizing {
    pub horizontal: Length,
    pub vertical: Length,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ZOrder(pub i32);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClipBehavior {
    Visible,
    Hidden,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    pub translate_x: f32,
    pub translate_y: f32,
    pub scale: f32,
    pub rotate_degrees: f32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RtlMode {
    Inherit,
    LeftToRight,
    RightToLeft,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LayoutStyle {
    pub flex: FlexLayout,
    pub absolute: Option<AbsoluteLayout>,
    pub measure: MeasurePolicy,
    pub safe_area: SafeAreaEdges,
    pub scroll_sizing: Option<ScrollSizing>,
    pub z_order: ZOrder,
    pub clipping: ClipBehavior,
    pub transform: Transform,
    pub rtl: RtlMode,
}
