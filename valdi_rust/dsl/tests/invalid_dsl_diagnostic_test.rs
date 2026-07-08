use valdi_rust_dsl::{
    attributes::{validate_attribute_for_element, DslAttribute},
    events::validate_event_for_element,
    fixtures::{contract_fixture, DslFixtureInput},
    source::source_span,
    DSL_ATTRIBUTE_ELEMENT_MISMATCH, DSL_CONTRACT_ROW_UNSUPPORTED, DSL_EVENT_ELEMENT_MISMATCH,
    DSL_FIXTURE_INPUT_INVALID, DSL_SOURCE_SPAN_REQUIRED,
};
use valdi_rust_ir::{
    elements::ElementKind,
    events::{EventBinding, EventKind},
    ids::{ActionId, NodeId},
    text::{TextNode, Wrapping},
};

#[test]
fn invalid_attribute_element_pair_returns_exact_diagnostic() {
    let source = source_span("dsl.invalid:1:1", "invalid.rs", 1, 1);
    let diagnostic = validate_attribute_for_element(
        ElementKind::Image,
        DslAttribute::Text(TextNode {
            value: "not image content",
            wrapping: Wrapping::Word,
        }),
        Some(source.id),
    )
    .expect_err("text attribute on image must fail");

    assert_eq!(diagnostic.code, DSL_ATTRIBUTE_ELEMENT_MISMATCH);
    assert_eq!(diagnostic.path, "$.nodes[].attributes.text");
    assert_eq!(diagnostic.severity.as_str(), "error");
    assert_eq!(diagnostic.source_span_id, Some(source.id));
}

#[test]
fn invalid_event_element_pair_returns_exact_diagnostic() {
    let source = source_span("dsl.invalid:2:1", "invalid.rs", 2, 1);
    let binding = EventBinding {
        node_id: NodeId::new("node.invalid.root"),
        kind: EventKind::Input,
        action_id: ActionId::new("action.invalid.input"),
    };
    let diagnostic = validate_event_for_element(ElementKind::View, binding, Some(source.id))
        .expect_err("input event on view must fail");

    assert_eq!(diagnostic.code, DSL_EVENT_ELEMENT_MISMATCH);
    assert_eq!(diagnostic.path, "$.nodes[].events[]");
    assert_eq!(diagnostic.severity.as_str(), "error");
    assert_eq!(diagnostic.source_span_id, Some(source.id));
}

#[test]
fn missing_source_span_returns_exact_diagnostic() {
    let diagnostic = validate_attribute_for_element(
        ElementKind::Text,
        DslAttribute::Text(TextNode {
            value: "missing span",
            wrapping: Wrapping::Word,
        }),
        None,
    )
    .expect_err("missing source span must fail");

    assert_eq!(diagnostic.code, DSL_SOURCE_SPAN_REQUIRED);
    assert_eq!(diagnostic.path, "$.nodes[].attributes.text");
    assert_eq!(diagnostic.severity.as_str(), "error");
    assert_eq!(diagnostic.source_span_id, None);
}

#[test]
fn unsupported_contract_row_returns_exact_diagnostic() {
    let diagnostic = contract_fixture(DslFixtureInput {
        fixture_id: "contract.unsupported.v1".to_string(),
        contract_row_id: "unsupported".to_string(),
        surface: "Unsupported".to_string(),
        fixture_tags: vec!["rust_backend".to_string()],
        platform_targets: vec!["rust_host".to_string()],
        owner_prs: vec!["PR09".to_string()],
        proof_prs: vec!["PR09".to_string()],
        serialized_artifact_path: "valdi_rust/fixtures/serialized/unsupported.ir.json".to_string(),
        coverage_tokens: vec!["unsupported".to_string()],
    })
    .expect_err("unknown row must fail");

    assert_eq!(diagnostic.code, DSL_CONTRACT_ROW_UNSUPPORTED);
    assert_eq!(diagnostic.path, "$.contract_row_id");
    assert_eq!(diagnostic.severity.as_str(), "error");
}

#[test]
fn missing_fixture_input_returns_exact_diagnostic() {
    let diagnostic = contract_fixture(DslFixtureInput {
        fixture_id: String::new(),
        contract_row_id: "schema_versioning".to_string(),
        surface: "Schema and versioning".to_string(),
        fixture_tags: vec!["rust_backend".to_string()],
        platform_targets: vec!["rust_host".to_string()],
        owner_prs: vec!["PR09".to_string()],
        proof_prs: vec!["PR09".to_string()],
        serialized_artifact_path: "valdi_rust/fixtures/serialized/schema_versioning.ir.json"
            .to_string(),
        coverage_tokens: vec!["schema.version".to_string()],
    })
    .expect_err("empty fixture ID must fail");

    assert_eq!(diagnostic.code, DSL_FIXTURE_INPUT_INVALID);
    assert_eq!(diagnostic.path, "$.fixture_id");
    assert_eq!(diagnostic.severity.as_str(), "error");
}

#[test]
fn serialized_fixture_path_drift_returns_exact_diagnostic() {
    let diagnostic = contract_fixture(DslFixtureInput {
        fixture_id: "contract.schema_versioning.v1".to_string(),
        contract_row_id: "schema_versioning".to_string(),
        surface: "Schema and versioning".to_string(),
        fixture_tags: vec!["rust_backend".to_string()],
        platform_targets: vec!["rust_host".to_string()],
        owner_prs: vec!["PR09".to_string()],
        proof_prs: vec!["PR09".to_string()],
        serialized_artifact_path: "valdi_rust/fixtures/serialized/wrong.ir.json".to_string(),
        coverage_tokens: vec!["schema.version".to_string()],
    })
    .expect_err("serialized path drift must fail");

    assert_eq!(diagnostic.code, DSL_FIXTURE_INPUT_INVALID);
    assert_eq!(diagnostic.path, "$.metadata.serialized_artifact_path");
    assert_eq!(diagnostic.severity.as_str(), "error");
}
