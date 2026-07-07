//! PNG backend schema.
//!
//! ```
//! use valdi_rust_ir::png::{DisplayCommand, DisplayList};
//!
//! let list = DisplayList { commands: &[DisplayCommand::Save] };
//! assert_eq!(list.commands.len(), 1);
//! ```

use crate::accessibility::AccessibilityRole;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PngRenderPlan {
    pub layout_snapshot: LayoutSnapshot,
    pub display_list: DisplayList,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LayoutSnapshot {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DisplayList {
    pub commands: &'static [DisplayCommand],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DisplayCommand {
    Save,
    Restore,
    DrawText(&'static str),
    DrawImage(&'static str),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeViewFallback {
    pub label: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WebViewFallback {
    pub label: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AccessibilityDebugMetadata {
    pub role: AccessibilityRole,
    pub label: &'static str,
}
