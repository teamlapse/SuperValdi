use valdi_rust_dsl::{
    dependency_ids, fixtures::payload_for_contract_row, CRATE_ID, PUBLIC_API_BOUNDARY,
};
use valdi_rust_ir::ts_compatibility::RustPathDependencyCheck;

#[test]
fn dsl_crate_depends_only_on_the_rust_ir_schema_crate() {
    assert_eq!(CRATE_ID, "valdi_rust_dsl");
    assert_eq!(dependency_ids(), [valdi_rust_ir::CRATE_ID]);
    assert!(PUBLIC_API_BOUNDARY.starts_with("rust_"));
}

#[test]
fn dsl_source_excludes_forbidden_renderer_and_compiler_terms() {
    let source_files = [
        include_str!("../src/lib.rs"),
        include_str!("../src/accessibility.rs"),
        include_str!("../src/attributes.rs"),
        include_str!("../src/bindings.rs"),
        include_str!("../src/diagnostics.rs"),
        include_str!("../src/elements.rs"),
        include_str!("../src/events.rs"),
        include_str!("../src/fixtures.rs"),
        include_str!("../src/modules.rs"),
        include_str!("../src/native_views.rs"),
        include_str!("../src/platform_extensions.rs"),
        include_str!("../src/source.rs"),
        include_str!("../src/tree.rs"),
    ];
    let forbidden_terms = [
        "extern \"C\"",
        "cxx::",
        "bindgen",
        "cbindgen",
        "std::ffi::",
        "libc::",
        "RenderRequest",
        "ViewNodeRenderer",
        "ViewNodeTree",
        "JS direct renderer",
        "TypeScript compiler/runtime",
        "TSN C emitter/runtime",
    ];

    for source in source_files {
        for term in forbidden_terms {
            assert!(
                !source.contains(term),
                "DSL source exposes forbidden dependency term {term}"
            );
        }
    }
}

#[test]
fn ts_compatibility_metadata_remains_a_dependency_exclusion_check() {
    let payload = payload_for_contract_row("ts_compatibility").expect("TS compatibility payload");
    let valdi_rust_dsl::DslSurfacePayload::TsCompatibility {
        dependency_check, ..
    } = payload
    else {
        panic!("unexpected payload");
    };

    assert_eq!(
        dependency_check,
        RustPathDependencyCheck {
            excludes_typescript_runtime: true
        }
    );
}
