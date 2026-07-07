//! Element taxonomy schema.
//!
//! ```
//! use valdi_rust_ir::elements::ElementKind;
//!
//! assert_eq!(ElementKind::NativeView.contract_token(), "native_view");
//! ```

use crate::ids::{ComponentId, NativeViewId, NodeId};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ElementKind {
    View,
    Layout,
    Scroll,
    Image,
    Text,
    RichText,
    TextInput,
    Control,
    List,
    WebView,
    NativeView,
    DrawingHost,
}

impl ElementKind {
    pub const fn contract_token(self) -> &'static str {
        match self {
            Self::View => "view",
            Self::Layout => "layout",
            Self::Scroll => "scroll",
            Self::Image => "image",
            Self::Text => "text",
            Self::RichText => "rich_text",
            Self::TextInput => "text_input",
            Self::Control => "control",
            Self::List => "list",
            Self::WebView => "webview",
            Self::NativeView => "native_view",
            Self::DrawingHost => "drawing_host",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ElementIdentity {
    pub component_id: ComponentId,
    pub node_id: NodeId,
    pub kind: ElementKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeViewRef {
    pub native_view_id: NativeViewId,
    pub host_node_id: NodeId,
}
