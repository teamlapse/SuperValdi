use valdi_rust_ir::{
    bindings::{
        ComparisonOp, ComputedProjection, FieldPath, ListProjection, LiteralValue, LogicOp,
        PlatformConstant,
    },
    ids::SourceSpanId,
};

use crate::{
    diagnostics::{RuntimeDiagnostic, RuntimeResult},
    state::{StateStore, StateValue, StateValueKind, RUNTIME_BINDING_FIELD_MISSING},
};

pub const RUNTIME_BINDING_TYPE_MISMATCH: &str = "RUNTIME_BINDING_TYPE_MISMATCH";
pub const RUNTIME_PLATFORM_CONSTANT_MISSING: &str = "RUNTIME_PLATFORM_CONSTANT_MISSING";
pub const RUNTIME_COMPUTED_PROJECTION_MISSING: &str = "RUNTIME_COMPUTED_PROJECTION_MISSING";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuntimeExpression {
    Field(RuntimeFieldExpression),
    Literal(RuntimeLiteralExpression),
    Nullable(RuntimeNullableExpression),
    Logic(RuntimeLogicExpression),
    Comparison(RuntimeComparisonExpression),
    ListProjection(RuntimeListProjectionExpression),
    ComputedProjection(RuntimeComputedProjectionExpression),
    PlatformConstant(RuntimePlatformConstantExpression),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeFieldExpression {
    pub path: FieldPath,
    pub source_span_id: Option<SourceSpanId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeLiteralExpression {
    pub value: StateValue,
    pub source_span_id: Option<SourceSpanId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeNullableExpression {
    pub expression: Box<RuntimeExpression>,
    pub source_span_id: Option<SourceSpanId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeLogicExpression {
    pub op: LogicOp,
    pub left: Box<RuntimeExpression>,
    pub right: Option<Box<RuntimeExpression>>,
    pub source_span_id: Option<SourceSpanId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeComparisonExpression {
    pub op: ComparisonOp,
    pub left: Box<RuntimeExpression>,
    pub right: Box<RuntimeExpression>,
    pub source_span_id: Option<SourceSpanId>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeListProjectionExpression {
    pub projection: ListProjection,
    pub field_name: &'static str,
    pub source_span_id: Option<SourceSpanId>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeComputedProjectionExpression {
    pub projection: ComputedProjection,
    pub source: FieldPath,
    pub source_span_id: Option<SourceSpanId>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimePlatformConstantExpression {
    pub constant: PlatformConstant,
    pub source_span_id: Option<SourceSpanId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimePlatformConstants {
    entries: Vec<RuntimePlatformConstantEntry>,
}

impl RuntimePlatformConstants {
    pub fn new(entries: Vec<RuntimePlatformConstantEntry>) -> Self {
        Self { entries }
    }

    pub fn get(
        &self,
        name: &'static str,
        source_span_id: Option<SourceSpanId>,
    ) -> RuntimeResult<StateValue> {
        self.entries
            .iter()
            .find(|entry| entry.name == name)
            .map(|entry| entry.value.clone())
            .ok_or_else(|| {
                RuntimeDiagnostic::error(
                    RUNTIME_PLATFORM_CONSTANT_MISSING,
                    "$.binding.platform_constant",
                    format!("platform constant {name} is missing"),
                    source_span_id,
                )
            })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimePlatformConstantEntry {
    pub name: &'static str,
    pub value: StateValue,
}

pub struct ExpressionContext<'a> {
    pub state: &'a StateStore,
    pub platform_constants: &'a RuntimePlatformConstants,
}

pub fn evaluate_expression(
    context: &ExpressionContext<'_>,
    expression: &RuntimeExpression,
) -> RuntimeResult<StateValue> {
    match expression {
        RuntimeExpression::Field(field) => context
            .state
            .resolve_field_path(field.path.path, field.source_span_id),
        RuntimeExpression::Literal(literal) => Ok(literal.value.clone()),
        RuntimeExpression::Nullable(nullable) => {
            let value = evaluate_expression(context, &nullable.expression)?;
            Ok(StateValue::Bool(value == StateValue::Null))
        }
        RuntimeExpression::Logic(logic) => evaluate_logic(context, logic),
        RuntimeExpression::Comparison(comparison) => evaluate_comparison(context, comparison),
        RuntimeExpression::ListProjection(projection) => {
            evaluate_list_projection(context, projection)
        }
        RuntimeExpression::ComputedProjection(projection) => {
            evaluate_computed_projection(context, projection)
        }
        RuntimeExpression::PlatformConstant(platform) => context
            .platform_constants
            .get(platform.constant.name, platform.source_span_id),
    }
}

pub fn literal_value(value: LiteralValue) -> StateValue {
    match value {
        LiteralValue::Null => StateValue::Null,
        LiteralValue::Bool(value) => StateValue::Bool(value),
        LiteralValue::I64(value) => StateValue::I64(value),
        LiteralValue::Text(value) => StateValue::Text(value),
    }
}

fn evaluate_logic(
    context: &ExpressionContext<'_>,
    logic: &RuntimeLogicExpression,
) -> RuntimeResult<StateValue> {
    let left = expect_bool(
        evaluate_expression(context, &logic.left)?,
        logic.source_span_id,
    )?;
    let value = match logic.op {
        LogicOp::Not => !left,
        LogicOp::And => {
            let right = expect_bool(
                evaluate_expression(
                    context,
                    logic
                        .right
                        .as_ref()
                        .ok_or_else(|| missing_operand(logic.source_span_id))?,
                )?,
                logic.source_span_id,
            )?;
            left && right
        }
        LogicOp::Or => {
            let right = expect_bool(
                evaluate_expression(
                    context,
                    logic
                        .right
                        .as_ref()
                        .ok_or_else(|| missing_operand(logic.source_span_id))?,
                )?,
                logic.source_span_id,
            )?;
            left || right
        }
    };
    Ok(StateValue::Bool(value))
}

fn evaluate_comparison(
    context: &ExpressionContext<'_>,
    comparison: &RuntimeComparisonExpression,
) -> RuntimeResult<StateValue> {
    let left = evaluate_expression(context, &comparison.left)?;
    let right = evaluate_expression(context, &comparison.right)?;
    let value = match comparison.op {
        ComparisonOp::Equal => left == right,
        ComparisonOp::NotEqual => left != right,
        ComparisonOp::GreaterThan => {
            expect_i64(left, comparison.source_span_id)?
                > expect_i64(right, comparison.source_span_id)?
        }
        ComparisonOp::LessThan => {
            expect_i64(left, comparison.source_span_id)?
                < expect_i64(right, comparison.source_span_id)?
        }
    };
    Ok(StateValue::Bool(value))
}

fn evaluate_list_projection(
    context: &ExpressionContext<'_>,
    projection: &RuntimeListProjectionExpression,
) -> RuntimeResult<StateValue> {
    let source = context
        .state
        .resolve_field_path(projection.projection.source.path, projection.source_span_id)?;
    let StateValue::List(items) = source else {
        return Err(type_mismatch(
            StateValueKind::List,
            source.kind(),
            projection.source_span_id,
        ));
    };

    let mut projected = Vec::new();
    for item in items {
        let value = item.field(projection.field_name).ok_or_else(|| {
            RuntimeDiagnostic::error(
                RUNTIME_BINDING_FIELD_MISSING,
                "$.binding.field_path",
                format!("list projection field {} is missing", projection.field_name),
                projection.source_span_id,
            )
        })?;
        projected.push(value.clone());
    }
    Ok(StateValue::List(projected))
}

fn evaluate_computed_projection(
    context: &ExpressionContext<'_>,
    projection: &RuntimeComputedProjectionExpression,
) -> RuntimeResult<StateValue> {
    let source = context
        .state
        .resolve_field_path(projection.source.path, projection.source_span_id)?;
    match (projection.projection.name, source) {
        ("count", StateValue::List(items)) => Ok(StateValue::I64(items.len() as i64)),
        ("is_empty", StateValue::List(items)) => Ok(StateValue::Bool(items.is_empty())),
        (name, _) => Err(RuntimeDiagnostic::error(
            RUNTIME_COMPUTED_PROJECTION_MISSING,
            "$.binding.computed_projection",
            format!("computed projection {name} is unsupported for source"),
            projection.source_span_id,
        )),
    }
}

fn expect_bool(value: StateValue, source_span_id: Option<SourceSpanId>) -> RuntimeResult<bool> {
    let actual = value.kind();
    value
        .as_bool()
        .ok_or_else(|| type_mismatch(StateValueKind::Bool, actual, source_span_id))
}

fn expect_i64(value: StateValue, source_span_id: Option<SourceSpanId>) -> RuntimeResult<i64> {
    let actual = value.kind();
    value
        .as_i64()
        .ok_or_else(|| type_mismatch(StateValueKind::I64, actual, source_span_id))
}

fn type_mismatch(
    expected: StateValueKind,
    actual: StateValueKind,
    source_span_id: Option<SourceSpanId>,
) -> RuntimeDiagnostic {
    RuntimeDiagnostic::error(
        RUNTIME_BINDING_TYPE_MISMATCH,
        "$.binding.value",
        format!(
            "binding value has type {}, expected {}",
            actual.as_str(),
            expected.as_str()
        ),
        source_span_id,
    )
}

fn missing_operand(source_span_id: Option<SourceSpanId>) -> RuntimeDiagnostic {
    RuntimeDiagnostic::error(
        RUNTIME_BINDING_TYPE_MISMATCH,
        "$.binding.value",
        "binding expression operand is missing",
        source_span_id,
    )
}
