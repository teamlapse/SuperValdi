use valdi_rust_ir::dynamic_ui::{
    BinaryBytesLabel, DebugJsonLabel, GeneratedFixtureRef, InMemoryProducerTag,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DynamicUiSourceKind {
    JsonDebug(DebugJsonLabel),
    BinaryBytes(BinaryBytesLabel),
    GeneratedFixture(GeneratedFixtureRef),
    InMemory(InMemoryProducerTag),
}

impl DynamicUiSourceKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::JsonDebug(_) => "json_debug",
            Self::BinaryBytes(_) => "binary_bytes",
            Self::GeneratedFixture(_) => "generated_fixture",
            Self::InMemory(_) => "in_memory",
        }
    }

    pub const fn source_id(self) -> &'static str {
        match self {
            Self::JsonDebug(label) => label.label,
            Self::BinaryBytes(label) => label.label,
            Self::GeneratedFixture(reference) => reference.fixture_name,
            Self::InMemory(tag) => tag.name,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DynamicUiSourceMetadata {
    pub kind: DynamicUiSourceKind,
    pub schema_path: &'static str,
}

impl DynamicUiSourceMetadata {
    pub const fn json_debug(label: &'static str, schema_path: &'static str) -> Self {
        Self {
            kind: DynamicUiSourceKind::JsonDebug(DebugJsonLabel { label }),
            schema_path,
        }
    }

    pub const fn binary_bytes(label: &'static str, schema_path: &'static str) -> Self {
        Self {
            kind: DynamicUiSourceKind::BinaryBytes(BinaryBytesLabel { label }),
            schema_path,
        }
    }

    pub const fn generated_fixture(fixture_name: &'static str, schema_path: &'static str) -> Self {
        Self {
            kind: DynamicUiSourceKind::GeneratedFixture(GeneratedFixtureRef { fixture_name }),
            schema_path,
        }
    }

    pub const fn in_memory(name: &'static str, schema_path: &'static str) -> Self {
        Self {
            kind: DynamicUiSourceKind::InMemory(InMemoryProducerTag { name }),
            schema_path,
        }
    }

    pub const fn source_id(self) -> &'static str {
        self.kind.source_id()
    }
}

pub const VALIDATED_IR_SCHEMA_PATH: &str = "$.ir_debug";
