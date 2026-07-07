use crate::json_debug::FixtureEnvelope;

pub fn inspect_fixture(fixture: &FixtureEnvelope, backend_tag: Option<&str>) -> String {
    let selected_tag = backend_tag.unwrap_or_else(|| fixture.fixture_tags.first().map_or("unknown", String::as_str));
    let owner_pr = fixture.metadata.owner_prs.first().map_or("unknown", String::as_str);
    let source_span = fixture.ir_debug.source_span.as_deref().unwrap_or("<absent>");
    [
        format!("schema_path: $.fixtures[{}]", fixture.fixture_id),
        format!("node_id: {}", fixture.ir_debug.root_node_id),
        format!("source_span: {source_span}"),
        format!("owner_pr: {owner_pr}"),
        format!("fixture_id: {}", fixture.fixture_id),
        format!("backend_tag: {selected_tag}"),
        format!("surface: {}", fixture.surface),
    ]
    .join("\n")
}
