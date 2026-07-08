#!/usr/bin/env python3

import json
import sys
from pathlib import Path


FORBIDDEN_CRATE_DEPENDENCY_NAMES = {
    "typescript",
    "ts-node",
    "ts-jest",
    "valdi_tsx",
    "jsx_direct_renderer",
    "tsn",
    "compiler_native_c_emitter",
    "cpp_renderer_adapter",
    "transition_adapter",
}

FORBIDDEN_RUST_IMPORT_OR_PATH_TERMS = {
    "compiler/companion",
    "src/valdi_modules/src/valdi/valdi_tsx",
    "JSXProcessor",
    "JSXRendererDelegate",
    "CompilerNativeCEmitter",
    "tsn/",
    "cpp_renderer_adapter",
    "transition_adapter",
}


def fail(message):
    raise AssertionError(message)


def flatten(value):
    if isinstance(value, list):
        return " ".join(flatten(item) for item in value)
    if isinstance(value, dict):
        return " ".join(flatten(item) for item in value.values())
    return str(value)


def main():
    graph = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
    source_paths = [Path(path) for path in sys.argv[2:]]

    compatibility = graph.get("tsx_to_ir_compatibility")
    expected_compatibility = {
        "owner": "PR10",
        "compatibility_package": "compiler/companion/src/tsx_ir_compat",
        "coverage_manifest_label": "//compiler/companion:tsx_feature_coverage_manifest",
        "source_contract_label": "//docs/rust_migration:replacement_contract_yaml",
        "source_fixture_manifest_label": "//valdi_rust/fixtures:contract_fixture_manifest",
        "rust_app_dependency": False,
        "rust_crate_dependencies": [],
        "test_labels": [
            "//valdi_rust/tests:tsx_to_ir_compat_contract_test",
            "//valdi_rust/tests:rust_app_no_ts_dependency_test",
        ],
    }
    if compatibility != expected_compatibility:
        fail(f"tsx_to_ir_compatibility metadata mismatch: {compatibility} != {expected_compatibility}")

    for crate in graph.get("crates", []):
        crate_text = flatten(crate)
        for forbidden in FORBIDDEN_CRATE_DEPENDENCY_NAMES:
            if forbidden in crate_text:
                fail(f"{crate['name']} exposes forbidden TS compatibility dependency: {forbidden}")
        for dependency in crate.get("dependencies", []):
            if dependency in FORBIDDEN_CRATE_DEPENDENCY_NAMES:
                fail(f"{crate['name']} depends on forbidden TS compatibility crate: {dependency}")
        for dependency in crate.get("external_dependencies", []):
            if dependency in FORBIDDEN_CRATE_DEPENDENCY_NAMES:
                fail(f"{crate['name']} depends on forbidden TS compatibility external dependency: {dependency}")

    for source_path in source_paths:
        source_text = source_path.read_text(encoding="utf-8")
        for forbidden in FORBIDDEN_RUST_IMPORT_OR_PATH_TERMS:
            if forbidden in source_text:
                fail(f"{source_path} contains forbidden Rust path/import term: {forbidden}")

    print(f"validated {len(graph.get('crates', []))} Rust crates have no TS compatibility dependency")


if __name__ == "__main__":
    main()
