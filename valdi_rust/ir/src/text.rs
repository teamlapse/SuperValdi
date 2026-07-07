//! Text schema.
//!
//! ```
//! use valdi_rust_ir::text::{TextNode, Wrapping};
//!
//! let text = TextNode { value: "hello", wrapping: Wrapping::Word };
//! assert_eq!(text.value, "hello");
//! ```

use crate::ids::{BindingId, SourceSpanId};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TextNode {
    pub value: &'static str,
    pub wrapping: Wrapping,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RichText {
    pub spans: &'static [TextSpan],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TextSpan {
    pub value: &'static str,
    pub font: FontStyle,
    pub link: Option<LinkTarget>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FontStyle {
    pub family: &'static str,
    pub size_points: u16,
    pub bold: bool,
    pub italic: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Wrapping {
    None,
    Word,
    Character,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Truncation {
    None,
    Head,
    Middle,
    Tail,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LinkTarget {
    pub href: &'static str,
    pub source_span_id: SourceSpanId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TextMeasurementHook {
    pub hook_name: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TextInputState {
    pub value_binding: BindingId,
    pub selection: Option<SelectionRange>,
    pub composition: Option<CompositionRange>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SelectionRange {
    pub start: u32,
    pub end: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CompositionRange {
    pub start: u32,
    pub end: u32,
}
