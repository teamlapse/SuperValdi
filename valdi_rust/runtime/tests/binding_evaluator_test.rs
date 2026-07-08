use valdi_rust_ir::{
    bindings::{BindingExpression, FieldPath, LiteralValue},
    ids::{BindingId, SourceSpanId},
};
use valdi_rust_runtime::{
    evaluate_expression, literal_value, sample_binding_resolver, sample_ir_binding_expression,
    sample_list_projection_expression, sample_logic_expression, sample_null_literal_expression,
    sample_state_store, ExpressionContext, RuntimeExpression, RuntimeFieldExpression,
    RuntimeNullableExpression, StateValue, RUNTIME_BINDING_FIELD_MISSING,
};

#[test]
fn binding_evaluator_resolves_required_expression_shapes() {
    let state = sample_state_store().expect("state fixture loads");
    let resolver = sample_binding_resolver();
    let context = ExpressionContext {
        state: &state,
        platform_constants: &resolver.platform_constants,
    };

    let user_name = resolver
        .resolve(BindingId::new("binding.user_name"), &state)
        .expect("user name binding");
    assert_eq!(user_name.value, StateValue::Text("Ada"));

    let null_literal = evaluate_expression(&context, &sample_null_literal_expression())
        .expect("null literal evaluates");
    assert_eq!(null_literal, StateValue::Null);
    assert_eq!(
        literal_value(LiteralValue::Bool(true)),
        StateValue::Bool(true)
    );
    assert_eq!(
        sample_ir_binding_expression(),
        BindingExpression::Literal(LiteralValue::Bool(true))
    );

    let nullable = RuntimeExpression::Nullable(RuntimeNullableExpression {
        expression: Box::new(sample_null_literal_expression()),
        source_span_id: Some(SourceSpanId::new("binding_test:1:1")),
    });
    assert_eq!(
        evaluate_expression(&context, &nullable).expect("nullable evaluates"),
        StateValue::Bool(true)
    );

    assert_eq!(
        evaluate_expression(&context, &sample_logic_expression()).expect("logic evaluates"),
        StateValue::Bool(true)
    );

    assert_eq!(
        evaluate_expression(&context, &sample_list_projection_expression())
            .expect("list projection evaluates"),
        StateValue::List(vec![StateValue::Text("tea"), StateValue::Text("cake")])
    );

    assert_eq!(
        resolver
            .resolve(BindingId::new("binding.cart_count"), &state)
            .expect("computed binding")
            .value,
        StateValue::I64(2)
    );
    assert_eq!(
        resolver
            .resolve(BindingId::new("binding.platform_os"), &state)
            .expect("platform constant binding")
            .value,
        StateValue::Text("ios")
    );
}

#[test]
fn binding_evaluator_reports_missing_field_with_source_span() {
    let state = sample_state_store().expect("state fixture loads");
    let resolver = sample_binding_resolver();
    let context = ExpressionContext {
        state: &state,
        platform_constants: &resolver.platform_constants,
    };
    let expression = RuntimeExpression::Field(RuntimeFieldExpression {
        path: FieldPath::new("app.user.missing"),
        source_span_id: Some(SourceSpanId::new("binding_test:9:3")),
    });

    let diagnostic =
        evaluate_expression(&context, &expression).expect_err("missing field must fail");
    assert_eq!(diagnostic.code, RUNTIME_BINDING_FIELD_MISSING);
    assert_eq!(diagnostic.path, "$.binding.field_path");
    assert_eq!(diagnostic.severity.as_str(), "error");
    assert_eq!(
        diagnostic.source_span_id.expect("source span").as_str(),
        "binding_test:9:3"
    );
}
