#!/usr/bin/env python3

import json
import sys
from pathlib import Path


REQUIRED_CRATES = {
    "valdi_rust_backend",
    "valdi_rust_cli",
    "valdi_rust_codec",
    "valdi_rust_codegen",
    "valdi_rust_fixtures",
    "valdi_rust_ir",
    "valdi_rust_runtime",
}
EXPECTED_DEPENDENCIES = {
    "valdi_rust_backend": ["valdi_rust_ir"],
    "valdi_rust_cli": [
        "valdi_rust_backend",
        "valdi_rust_codec",
        "valdi_rust_codegen",
        "valdi_rust_ir",
        "valdi_rust_runtime",
    ],
    "valdi_rust_codec": ["valdi_rust_ir"],
    "valdi_rust_codegen": ["valdi_rust_backend", "valdi_rust_ir"],
    "valdi_rust_fixtures": ["valdi_rust_ir"],
    "valdi_rust_ir": [],
    "valdi_rust_runtime": ["valdi_rust_backend", "valdi_rust_ir"],
}
EXPECTED_OWNERS = {
    "valdi_rust_backend": "PR02",
    "valdi_rust_cli": "PR02",
    "valdi_rust_codec": "PR05",
    "valdi_rust_codegen": "PR02",
    "valdi_rust_fixtures": "PR04",
    "valdi_rust_ir": "PR02",
    "valdi_rust_runtime": "PR02",
}
REQUIRED_PLATFORMS = {"android", "ios", "png", "web"}
REQUIRED_GLUE_TARGETS = {"js_dom", "kotlin", "rust_host", "swift"}
EXPECTED_FIXTURE_CORPUS = {
    "owner": "PR04",
    "crate": "valdi_rust_fixtures",
    "manifest_label": "//valdi_rust/fixtures:contract_fixture_manifest",
    "invalid_manifest_label": "//valdi_rust/fixtures:invalid_fixture_manifest",
    "serialized_fixture_label": "//valdi_rust/fixtures:serialized_fixtures",
    "test_suite_label": "//valdi_rust:fixture_corpus_tests",
    "coverage_test_labels": [
        "//valdi_rust/tests:fixture_corpus_contract_test",
        "//valdi_rust/tests:invalid_fixture_manifest_test",
    ],
}
EXPECTED_CODEC_VALIDATOR = {
    "owner": "PR05",
    "crate": "valdi_rust_codec",
    "test_suite_label": "//valdi_rust:codec_validator_tests",
    "codec_test_labels": [
        "//valdi_rust/codec:roundtrip_test",
        "//valdi_rust/codec:validator_test",
        "//valdi_rust/codec:version_compatibility_test",
        "//valdi_rust/codec:inspect_snapshot_test",
    ],
    "cli_label": "//valdi_rust/cli:cli",
    "dependency_repository": "@valdi_rust_crates",
}
EXPECTED_BACKEND_OPERATIONS = {
    "owner": "PR06",
    "crate": "valdi_rust_backend",
    "test_suite_label": "//valdi_rust:backend_operation_tests",
    "backend_test_labels": [
        "//valdi_rust/backend:backend_contract_coverage_test",
        "//valdi_rust/backend:backend_trait_compile_test",
        "//valdi_rust/backend:capability_validator_test",
        "//valdi_rust/backend:mock_backend_snapshot_test",
    ],
    "snapshot_label": "//valdi_rust/backend:fixture_tag_backend_ops_snapshot",
    "source_fixture_manifest_label": "//valdi_rust/fixtures:contract_fixture_manifest",
    "configured_rust_targets": [
        "host",
        "aarch64-apple-ios",
        "aarch64-linux-android",
    ],
    "wasm_compile_proof": "blocked_no_rust_wasm_toolchain",
}
EXPECTED_RUNTIME_TREE_DIFF = {
    "owner": "PR07",
    "crate": "valdi_rust_runtime",
    "test_suite_label": "//valdi_rust:runtime_tree_diff_tests",
    "runtime_test_labels": [
        "//valdi_rust/runtime:document_loader_test",
        "//valdi_rust/runtime:tree_diff_test",
        "//valdi_rust/runtime:identity_patch_test",
        "//valdi_rust/runtime:rebuild_diagnostic_test",
        "//valdi_rust/runtime:static_fixture_backend_ops_snapshot_test",
    ],
    "snapshot_label": "//valdi_rust/runtime:static_fixture_backend_ops_snapshot",
    "operation_contract_crate": "valdi_rust_backend",
    "scope": "runtime_tree_diff_only",
}
EXPECTED_STATE_BINDINGS_ACTIONS = {
    "owner": "PR08",
    "crate": "valdi_rust_runtime",
    "test_suite_label": "//valdi_rust:state_bindings_actions_tests",
    "runtime_test_labels": [
        "//valdi_rust/runtime:state_store_test",
        "//valdi_rust/runtime:binding_evaluator_test",
        "//valdi_rust/runtime:action_scheduler_test",
        "//valdi_rust/runtime:event_action_binding_test",
        "//valdi_rust/runtime:state_patch_compatibility_test",
        "//valdi_rust/runtime:state_binding_action_snapshot_test",
    ],
    "snapshot_label": "//valdi_rust/runtime:state_binding_action_trace_snapshot",
    "scope": "state_bindings_actions_only",
}


def fail(message):
    raise AssertionError(message)


def require_owner(items, path):
    for item in items:
        if item.get("owner") != "PR02":
            fail(f"{path} entry lacks PR02 owner: {item}")


def main():
    graph = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
    if graph.get("schema_version") != 1:
        fail("crate graph schema_version must be 1")

    crates = graph.get("crates", [])
    crate_names = {crate.get("name") for crate in crates}
    if crate_names != REQUIRED_CRATES:
        fail(f"crate graph mismatch: {sorted(crate_names)}")

    for crate in crates:
        for key in ["label", "owner", "visibility", "public_api_boundary", "dependencies"]:
            if key not in crate:
                fail(f"{crate.get('name')} missing {key}")
        if crate["owner"] != EXPECTED_OWNERS[crate["name"]]:
            fail(f"{crate['name']} owner must be {EXPECTED_OWNERS[crate['name']]}")
        if not crate["public_api_boundary"].startswith("rust_"):
            fail(f"{crate['name']} must declare a Rust public API boundary")
        if crate["dependencies"] != EXPECTED_DEPENDENCIES[crate["name"]]:
            fail(
                f"{crate['name']} dependency mismatch: "
                f"{crate['dependencies']} != {EXPECTED_DEPENDENCIES[crate['name']]}"
            )

    fixture_corpus = graph.get("fixture_corpus")
    if fixture_corpus != EXPECTED_FIXTURE_CORPUS:
        fail(f"fixture_corpus metadata mismatch: {fixture_corpus} != {EXPECTED_FIXTURE_CORPUS}")

    codec_validator = graph.get("codec_validator")
    if codec_validator != EXPECTED_CODEC_VALIDATOR:
        fail(f"codec_validator metadata mismatch: {codec_validator} != {EXPECTED_CODEC_VALIDATOR}")

    backend_operations = graph.get("backend_operations")
    if backend_operations != EXPECTED_BACKEND_OPERATIONS:
        fail(f"backend_operations metadata mismatch: {backend_operations} != {EXPECTED_BACKEND_OPERATIONS}")

    runtime_tree_diff = graph.get("runtime_tree_diff")
    if runtime_tree_diff != EXPECTED_RUNTIME_TREE_DIFF:
        fail(f"runtime_tree_diff metadata mismatch: {runtime_tree_diff} != {EXPECTED_RUNTIME_TREE_DIFF}")

    state_bindings_actions = graph.get("state_bindings_actions")
    if state_bindings_actions != EXPECTED_STATE_BINDINGS_ACTIONS:
        fail(
            "state_bindings_actions metadata mismatch: "
            f"{state_bindings_actions} != {EXPECTED_STATE_BINDINGS_ACTIONS}"
        )

    platform_hosts = graph.get("platform_host_placeholders", [])
    fixture_tests = graph.get("fixture_test_labels", [])
    generated_glue = graph.get("generated_glue_placeholders", [])
    if {item.get("platform") for item in platform_hosts} != REQUIRED_PLATFORMS:
        fail("platform host placeholders must cover ios, android, web, and png")
    if {item.get("platform") for item in fixture_tests} != REQUIRED_PLATFORMS:
        fail("fixture test labels must cover ios, android, web, and png")
    if {item.get("target") for item in generated_glue} != REQUIRED_GLUE_TARGETS:
        fail("generated glue placeholders must cover Swift, Kotlin, JS DOM, and Rust host")

    require_owner(platform_hosts, "platform_host_placeholders")
    require_owner(fixture_tests, "fixture_test_labels")
    require_owner(generated_glue, "generated_glue_placeholders")


if __name__ == "__main__":
    main()
