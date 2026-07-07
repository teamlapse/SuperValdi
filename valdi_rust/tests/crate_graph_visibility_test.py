#!/usr/bin/env python3

import json
import sys
from pathlib import Path


REQUIRED_CRATES = {
    "valdi_rust_backend",
    "valdi_rust_cli",
    "valdi_rust_codegen",
    "valdi_rust_ir",
    "valdi_rust_runtime",
}
EXPECTED_DEPENDENCIES = {
    "valdi_rust_backend": ["valdi_rust_ir"],
    "valdi_rust_cli": [
        "valdi_rust_backend",
        "valdi_rust_codegen",
        "valdi_rust_ir",
        "valdi_rust_runtime",
    ],
    "valdi_rust_codegen": ["valdi_rust_backend", "valdi_rust_ir"],
    "valdi_rust_ir": [],
    "valdi_rust_runtime": ["valdi_rust_backend", "valdi_rust_ir"],
}
REQUIRED_PLATFORMS = {"android", "ios", "png", "web"}
REQUIRED_GLUE_TARGETS = {"js_dom", "kotlin", "rust_host", "swift"}


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
        if crate["owner"] != "PR02":
            fail(f"{crate['name']} owner must be PR02")
        if not crate["public_api_boundary"].startswith("rust_"):
            fail(f"{crate['name']} must declare a Rust public API boundary")
        if crate["dependencies"] != EXPECTED_DEPENDENCIES[crate["name"]]:
            fail(
                f"{crate['name']} dependency mismatch: "
                f"{crate['dependencies']} != {EXPECTED_DEPENDENCIES[crate['name']]}"
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
