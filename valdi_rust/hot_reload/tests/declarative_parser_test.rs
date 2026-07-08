use valdi_rust_hot_reload::{
    parse_declarative_patch, HotReloadEdit, HotReloadEditFamily, HOT_RELOAD_PARSE_ERROR,
    SUPPORTED_PATCH_INPUTS,
};
use valdi_rust_ir::elements::ElementKind;

#[test]
fn parser_covers_every_supported_patch_family() {
    let families = SUPPORTED_PATCH_INPUTS
        .iter()
        .map(|input| parse_declarative_patch(input).unwrap().family())
        .collect::<Vec<_>>();

    assert_eq!(
        families,
        vec![
            HotReloadEditFamily::UiTree,
            HotReloadEditFamily::Style,
            HotReloadEditFamily::Layout,
            HotReloadEditFamily::Text,
            HotReloadEditFamily::Asset,
            HotReloadEditFamily::Binding,
            HotReloadEditFamily::Event,
            HotReloadEditFamily::Accessibility,
            HotReloadEditFamily::ModuleRef,
            HotReloadEditFamily::NativeViewRef,
        ]
    );
}

#[test]
fn parser_builds_typed_tree_intent() {
    let intent = parse_declarative_patch(
        "tree patch_id=patch.test parent=node.root node=node.child kind=text key=key.child order=4 span=test:1:1",
    )
    .unwrap();

    match intent.edit {
        HotReloadEdit::UiTree(tree) => {
            assert_eq!(tree.node_id.as_str(), "node.child");
            assert_eq!(tree.parent_id.as_str(), "node.root");
            assert_eq!(tree.element_kind, ElementKind::Text);
            assert_eq!(tree.order.0, 4);
            assert_eq!(tree.key.unwrap().as_str(), "key.child");
        }
        other => panic!("unexpected edit {other:?}"),
    }
}

#[test]
fn parser_rejects_unknown_kind_with_exact_diagnostic() {
    let diagnostic = parse_declarative_patch("unknown patch_id=patch.bad").unwrap_err();

    assert_eq!(diagnostic.code, HOT_RELOAD_PARSE_ERROR);
    assert_eq!(diagnostic.path, "$.patch.kind");
    assert_eq!(diagnostic.reason, "unknown patch kind");
    assert_eq!(diagnostic.severity.as_str(), "error");
}
