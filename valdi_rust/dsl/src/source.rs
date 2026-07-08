use valdi_rust_ir::ids::SourceSpanId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DslSourceSpan {
    pub id: SourceSpanId,
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

impl DslSourceSpan {
    pub const fn new(id: SourceSpanId, file: &'static str, line: u32, column: u32) -> Self {
        Self {
            id,
            file,
            line,
            column,
        }
    }

    pub fn token(self) -> String {
        format!("{}:{}:{}", self.file, self.line, self.column)
    }
}

pub const fn source_span(
    id: &'static str,
    file: &'static str,
    line: u32,
    column: u32,
) -> DslSourceSpan {
    DslSourceSpan::new(SourceSpanId::new(id), file, line, column)
}
