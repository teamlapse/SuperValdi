//! Web DOM schema.
//!
//! ```
//! use valdi_rust_ir::web_dom::{ClassEmission, DomMapping};
//!
//! let mapping = DomMapping { tag_name: "div", class_emission: ClassEmission::Static("root") };
//! assert_eq!(mapping.tag_name, "div");
//! ```

use crate::events::EventKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DomMapping {
    pub tag_name: &'static str,
    pub class_emission: ClassEmission,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CssStyleMapping {
    pub property: &'static str,
    pub value_token: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DomEventMapping {
    pub event_name: &'static str,
    pub event_kind: EventKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TextMeasurementSpec {
    pub measurement_id: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClassEmission {
    Static(&'static str),
    Generated,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BrowserSnapshotSpec {
    pub snapshot_name: &'static str,
}
