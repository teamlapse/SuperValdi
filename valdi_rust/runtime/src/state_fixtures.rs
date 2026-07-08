use valdi_rust_ir::{
    actions::{
        ActionDefinition, ActionKind, CancellationPolicy, CoalescingKey, InvalidationTarget,
        SchedulingPolicy,
    },
    bindings::{
        BindingExpression, ComparisonOp, ComputedProjection, FieldPath, ListProjection,
        LiteralValue, PlatformConstant,
    },
    events::{EventBinding, EventKind},
    ids::{ActionId, BindingId, NodeId, SourceSpanId, StateId},
};

use crate::{
    actions::{ActionDispatchRecord, RuntimeActionBehavior, RuntimeActionDefinition},
    bindings::{BindingResolution, BindingResolver, RuntimeBinding},
    diagnostics::{RuntimeDiagnostic, RuntimeResult},
    event_bindings::EventActionBindings,
    expressions::{
        literal_value, RuntimeComparisonExpression, RuntimeComputedProjectionExpression,
        RuntimeExpression, RuntimeFieldExpression, RuntimeListProjectionExpression,
        RuntimeLiteralExpression, RuntimeLogicExpression, RuntimePlatformConstantEntry,
        RuntimePlatformConstantExpression, RuntimePlatformConstants,
    },
    scheduler::{typed_action_error, ActionScheduler},
    state::{StateEntry, StateField, StateStore, StateValue},
};

pub const STATE_BINDING_ACTION_TRACE_FIXTURE_ID: &str = "runtime.state_bindings_actions.v1";
pub const RUNTIME_STATE_ACTION_TRACE_DRIFT: &str = "RUNTIME_STATE_ACTION_TRACE_DRIFT";

pub fn sample_state_store() -> RuntimeResult<StateStore> {
    StateStore::new(vec![StateEntry::new(
        StateId::new("app"),
        StateValue::Record(vec![
            StateField {
                name: "user",
                value: StateValue::Record(vec![
                    StateField {
                        name: "name",
                        value: StateValue::Text("Ada"),
                    },
                    StateField {
                        name: "age",
                        value: StateValue::I64(37),
                    },
                    StateField {
                        name: "active",
                        value: StateValue::Bool(true),
                    },
                    StateField {
                        name: "nickname",
                        value: StateValue::Null,
                    },
                ]),
            },
            StateField {
                name: "cart",
                value: StateValue::Record(vec![StateField {
                    name: "items",
                    value: StateValue::List(vec![
                        StateValue::Record(vec![StateField {
                            name: "sku",
                            value: StateValue::Text("tea"),
                        }]),
                        StateValue::Record(vec![StateField {
                            name: "sku",
                            value: StateValue::Text("cake"),
                        }]),
                    ]),
                }]),
            },
        ]),
        Some(SourceSpanId::new("state_fixture:1:1")),
    )])
}

pub fn sample_binding_resolver() -> BindingResolver {
    BindingResolver {
        platform_constants: RuntimePlatformConstants::new(vec![RuntimePlatformConstantEntry {
            name: "platform.os",
            value: StateValue::Text("ios"),
        }]),
        bindings: vec![
            RuntimeBinding {
                binding_id: BindingId::new("binding.user_name"),
                expression: RuntimeExpression::Field(RuntimeFieldExpression {
                    path: FieldPath::new("app.user.name"),
                    source_span_id: Some(SourceSpanId::new("binding_fixture:1:5")),
                }),
                source_span_id: Some(SourceSpanId::new("binding_fixture:1:1")),
            },
            RuntimeBinding {
                binding_id: BindingId::new("binding.cart_count"),
                expression: RuntimeExpression::ComputedProjection(
                    RuntimeComputedProjectionExpression {
                        projection: ComputedProjection { name: "count" },
                        source: FieldPath::new("app.cart.items"),
                        source_span_id: Some(SourceSpanId::new("binding_fixture:2:5")),
                    },
                ),
                source_span_id: Some(SourceSpanId::new("binding_fixture:2:1")),
            },
            RuntimeBinding {
                binding_id: BindingId::new("binding.platform_os"),
                expression: RuntimeExpression::PlatformConstant(
                    RuntimePlatformConstantExpression {
                        constant: PlatformConstant {
                            name: "platform.os",
                        },
                        source_span_id: Some(SourceSpanId::new("binding_fixture:3:5")),
                    },
                ),
                source_span_id: Some(SourceSpanId::new("binding_fixture:3:1")),
            },
        ],
    }
}

pub fn sample_logic_expression() -> RuntimeExpression {
    RuntimeExpression::Logic(RuntimeLogicExpression {
        op: valdi_rust_ir::bindings::LogicOp::And,
        left: Box::new(RuntimeExpression::Field(RuntimeFieldExpression {
            path: FieldPath::new("app.user.active"),
            source_span_id: Some(SourceSpanId::new("binding_fixture:4:5")),
        })),
        right: Some(Box::new(RuntimeExpression::Comparison(
            RuntimeComparisonExpression {
                op: ComparisonOp::GreaterThan,
                left: Box::new(RuntimeExpression::Field(RuntimeFieldExpression {
                    path: FieldPath::new("app.user.age"),
                    source_span_id: Some(SourceSpanId::new("binding_fixture:4:20")),
                })),
                right: Box::new(RuntimeExpression::Literal(RuntimeLiteralExpression {
                    value: literal_value(LiteralValue::I64(30)),
                    source_span_id: Some(SourceSpanId::new("binding_fixture:4:35")),
                })),
                source_span_id: Some(SourceSpanId::new("binding_fixture:4:15")),
            },
        ))),
        source_span_id: Some(SourceSpanId::new("binding_fixture:4:1")),
    })
}

pub fn sample_list_projection_expression() -> RuntimeExpression {
    RuntimeExpression::ListProjection(RuntimeListProjectionExpression {
        projection: ListProjection {
            source: FieldPath::new("app.cart.items"),
        },
        field_name: "sku",
        source_span_id: Some(SourceSpanId::new("binding_fixture:5:1")),
    })
}

pub fn sample_null_literal_expression() -> RuntimeExpression {
    RuntimeExpression::Literal(RuntimeLiteralExpression {
        value: literal_value(LiteralValue::Null),
        source_span_id: Some(SourceSpanId::new("binding_fixture:6:1")),
    })
}

pub fn sample_ir_binding_expression() -> BindingExpression {
    BindingExpression::Literal(LiteralValue::Bool(true))
}

pub fn sample_action_scheduler() -> RuntimeResult<ActionScheduler> {
    ActionScheduler::new(vec![
        RuntimeActionDefinition {
            definition: ActionDefinition {
                id: ActionId::new("action.save"),
                kind: ActionKind::Sync,
                state_scope: StateId::new("app"),
            },
            cancellation_policy: CancellationPolicy::NotCancellable,
            scheduling_policy: SchedulingPolicy::Immediate,
            coalescing_key: None,
            invalidation_targets: vec![InvalidationTarget {
                state_id: StateId::new("app"),
            }],
            behavior: RuntimeActionBehavior::Complete,
            source_span_id: Some(SourceSpanId::new("action_fixture:1:1")),
        },
        RuntimeActionDefinition {
            definition: ActionDefinition {
                id: ActionId::new("action.load"),
                kind: ActionKind::Async,
                state_scope: StateId::new("app"),
            },
            cancellation_policy: CancellationPolicy::CancelByActionId,
            scheduling_policy: SchedulingPolicy::Deferred,
            coalescing_key: Some(CoalescingKey { value: "load.user" }),
            invalidation_targets: vec![InvalidationTarget {
                state_id: StateId::new("app"),
            }],
            behavior: RuntimeActionBehavior::DeterministicAsyncComplete,
            source_span_id: Some(SourceSpanId::new("action_fixture:2:1")),
        },
        RuntimeActionDefinition {
            definition: ActionDefinition {
                id: ActionId::new("action.fail"),
                kind: ActionKind::Sync,
                state_scope: StateId::new("app"),
            },
            cancellation_policy: CancellationPolicy::NotCancellable,
            scheduling_policy: SchedulingPolicy::Immediate,
            coalescing_key: None,
            invalidation_targets: Vec::new(),
            behavior: RuntimeActionBehavior::Fail(typed_action_error("network.unavailable")),
            source_span_id: Some(SourceSpanId::new("action_fixture:3:1")),
        },
    ])
}

pub fn sample_event_bindings() -> EventActionBindings {
    EventActionBindings::new(vec![EventBinding {
        node_id: NodeId::new("node.button"),
        kind: EventKind::Tap,
        action_id: ActionId::new("action.save"),
    }])
}

pub fn state_binding_action_trace_snapshot() -> RuntimeResult<String> {
    let mut state = sample_state_store()?;
    let resolver = sample_binding_resolver();
    let bindings = resolver.resolve_all(&state)?;
    let mut scheduler = sample_action_scheduler()?;
    let event_bindings = sample_event_bindings();
    let action_id =
        event_bindings.resolve(NodeId::new("node.button"), EventKind::Tap, &scheduler)?;
    let save = scheduler.dispatch(action_id, &mut state)?;
    let load = scheduler.dispatch(ActionId::new("action.load"), &mut state)?;
    let coalesced = scheduler.dispatch(ActionId::new("action.load"), &mut state)?;
    let load_complete = scheduler.complete_async(ActionId::new("action.load"), &mut state)?;

    let mut lines = vec![
        "state_binding_action_trace_v1".to_string(),
        format!("fixture={STATE_BINDING_ACTION_TRACE_FIXTURE_ID}"),
        "state app.user.name=Ada".to_string(),
    ];
    for binding in bindings {
        lines.push(format_binding(binding));
    }
    lines.push(format!("event node.button.tap -> {}", action_id.as_str()));
    lines.push(format_action("action save", save));
    lines.push(format_action("action load", load));
    lines.push(format_action("action load.coalesce", coalesced));
    lines.push(format_action("action load.complete", load_complete));
    Ok(format!("{}\n", lines.join("\n")))
}

pub fn validate_state_binding_action_trace_snapshot(expected: &str) -> RuntimeResult<()> {
    let actual = state_binding_action_trace_snapshot()?;
    if actual == expected {
        return Ok(());
    }
    Err(RuntimeDiagnostic::error(
        RUNTIME_STATE_ACTION_TRACE_DRIFT,
        "$.state_binding_action_trace",
        "state/binding/action trace snapshot drift",
        None,
    ))
}

fn format_binding(binding: BindingResolution) -> String {
    format!(
        "binding {}={}",
        binding.binding_id.as_str(),
        binding.value.display_token()
    )
}

fn format_action(label: &str, record: ActionDispatchRecord) -> String {
    let invalidations = if record.invalidations.is_empty() {
        "<none>".to_string()
    } else {
        record
            .invalidations
            .iter()
            .map(|invalidation| invalidation.state_id.as_str())
            .collect::<Vec<_>>()
            .join(",")
    };
    let key = record
        .coalescing_key
        .map(|key| key.value)
        .unwrap_or("<none>");
    format!(
        "{label} status={} schedule={:?} key={} invalidates={}",
        record.status.as_str(),
        record.scheduling_policy,
        key,
        invalidations
    )
}
