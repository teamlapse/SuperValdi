#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WatchedEditKind {
    DeclarativeSource,
    Asset,
    ModuleContract,
    NativeViewContract,
}

impl WatchedEditKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DeclarativeSource => "declarative_source",
            Self::Asset => "asset",
            Self::ModuleContract => "module_contract",
            Self::NativeViewContract => "native_view_contract",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WatchedEdit {
    pub path: &'static str,
    pub kind: WatchedEditKind,
    pub sequence: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SimulatedFileWatcher {
    edits: Vec<WatchedEdit>,
}

impl SimulatedFileWatcher {
    pub fn new(edits: Vec<WatchedEdit>) -> Self {
        Self { edits }
    }

    pub fn drain(&mut self) -> Vec<WatchedEdit> {
        core::mem::take(&mut self.edits)
    }
}
