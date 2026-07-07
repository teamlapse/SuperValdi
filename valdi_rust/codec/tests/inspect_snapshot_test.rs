use valdi_rust_codec::{inspect, json_debug};
use valdi_rust_fixtures::corpus::SERIALIZED_FIXTURES;

#[test]
fn ir_inspect_output_is_stable_for_fixture_payloads() {
    let fixture = json_debug::decode_fixture(SERIALIZED_FIXTURES[0].contents).expect("fixture parses");
    let output = inspect::inspect_fixture(&fixture, Some("ios"));
    let expected = [
        "schema_path: $.fixtures[contract.schema_versioning.v1]",
        "node_id: node.schema_versioning.root",
        "source_span: <absent>",
        "owner_pr: PR01",
        "fixture_id: contract.schema_versioning.v1",
        "backend_tag: ios",
        "surface: Schema and versioning",
    ]
    .join("\n");
    assert_eq!(output, expected);
}
