//! Bindings and expressions schema.
//!
//! ```
//! use valdi_rust_ir::bindings::{BindingExpression, FieldPath, LiteralValue};
//!
//! let expression = BindingExpression::Literal(LiteralValue::Bool(true));
//! assert_eq!(expression, BindingExpression::Literal(LiteralValue::Bool(true)));
//! assert_eq!(FieldPath::new("user.name").path, "user.name");
//! ```

use crate::ids::SourceSpanId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BindingExpression {
    Field(FieldPath),
    Literal(LiteralValue),
    Logic(LogicOp),
    Comparison(ComparisonOp),
    List(ListProjection),
    Computed(ComputedProjection),
    PlatformConstant(PlatformConstant),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FieldPath {
    pub path: &'static str,
}

impl FieldPath {
    pub const fn new(path: &'static str) -> Self {
        Self { path }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiteralValue {
    Null,
    Bool(bool),
    I64(i64),
    Text(&'static str),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComparisonOp {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LogicOp {
    And,
    Or,
    Not,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ListProjection {
    pub source: FieldPath,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ComputedProjection {
    pub name: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlatformConstant {
    pub name: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BindingSourceSpan {
    pub source_span_id: SourceSpanId,
}
