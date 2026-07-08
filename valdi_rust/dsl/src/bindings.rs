use valdi_rust_ir::{
    bindings::{
        BindingExpression, ComparisonOp, ComputedProjection, FieldPath, ListProjection,
        LiteralValue, LogicOp, PlatformConstant,
    },
    ids::SourceSpanId,
};

pub fn field(path: &'static str) -> BindingExpression {
    BindingExpression::Field(FieldPath::new(path))
}

pub const fn literal(value: LiteralValue) -> BindingExpression {
    BindingExpression::Literal(value)
}

pub const fn logic(op: LogicOp) -> BindingExpression {
    BindingExpression::Logic(op)
}

pub const fn comparison(op: ComparisonOp) -> BindingExpression {
    BindingExpression::Comparison(op)
}

pub fn list_projection(source: &'static str) -> BindingExpression {
    BindingExpression::List(ListProjection {
        source: FieldPath::new(source),
    })
}

pub const fn computed(name: &'static str) -> BindingExpression {
    BindingExpression::Computed(ComputedProjection { name })
}

pub const fn platform_constant(name: &'static str) -> BindingExpression {
    BindingExpression::PlatformConstant(PlatformConstant { name })
}

pub const fn source_span(id: SourceSpanId) -> valdi_rust_ir::bindings::BindingSourceSpan {
    valdi_rust_ir::bindings::BindingSourceSpan { source_span_id: id }
}
