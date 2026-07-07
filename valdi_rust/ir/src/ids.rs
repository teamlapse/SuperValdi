//! Stable identity newtypes shared by all IR surfaces.
//!
//! ```
//! use valdi_rust_ir::ids::{ActionId, NodeId, SourceSpanId};
//!
//! assert_eq!(NodeId::new("n1").as_str(), "n1");
//! assert_eq!(ActionId::new("save").as_str(), "save");
//! assert_eq!(SourceSpanId::new("src:1:1").as_str(), "src:1:1");
//! ```

macro_rules! id_type {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct $name(pub &'static str);

        impl $name {
            pub const fn new(value: &'static str) -> Self {
                Self(value)
            }

            pub const fn as_str(self) -> &'static str {
                self.0
            }
        }
    };
}

id_type!(SchemaVersionId);
id_type!(CapabilityId);
id_type!(ComponentId);
id_type!(NodeId);
id_type!(KeyId);
id_type!(StateId);
id_type!(ActionId);
id_type!(BindingId);
id_type!(ModuleId);
id_type!(NativeViewId);
id_type!(AssetId);
id_type!(SourceSpanId);
id_type!(DiagnosticId);
id_type!(FixtureId);
id_type!(BackendPathId);
id_type!(HotReloadIdentityId);
id_type!(DynamicIdentityId);
