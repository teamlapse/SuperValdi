#!/usr/bin/env python3

import json
import re
import sys
from pathlib import Path


OPAQUE_PLATFORM_BAG_TERMS = [
    "HashMap<",
    "HashMap<String",
    "BTreeMap<",
    "BTreeMap<String",
    "Box<dyn Any",
    "Box<dyn Any>",
    "serde_json",
    "serde_json::Value",
    "RawValue",
    "AnyPlatform",
    "raw_json",
    "json_blob",
]


EXPECTED_MODULES = {
    "schema_versioning": "schema",
    "component_identity": "ids",
    "tree_structure": "tree",
    "element_taxonomy": "elements",
    "layout": "layout",
    "styling": "styling",
    "text": "text",
    "assets": "assets",
    "events_gestures": "events",
    "actions_state": "actions",
    "bindings_expressions": "bindings",
    "animations": "animations",
    "native_modules": "native_modules",
    "native_views": "native_views",
    "accessibility": "accessibility",
    "hot_reload": "hot_reload",
    "diagnostics": "diagnostics",
    "web_dom": "web_dom",
    "png_backend": "png",
    "dynamic_ui": "dynamic_ui",
    "ts_compatibility": "ts_compatibility",
    "build_graph": "build_graph",
}


def fail(message):
    raise AssertionError(message)


def load_sources(paths):
    sources = {}
    for path in paths:
        source_path = Path(path)
        sources[source_path.name] = source_path.read_text(encoding="utf-8")
    return sources


def coverage_block(schema_text, row_id):
    marker = f'id: "{row_id}"'
    marker_index = schema_text.find(marker)
    if marker_index == -1:
        fail(f"{row_id} missing ContractRowCoverage id")
    start = schema_text.rfind("ContractRowCoverage {", 0, marker_index)
    end = schema_text.find(" },", marker_index)
    if start == -1 or end == -1:
        fail(f"{row_id} ContractRowCoverage entry is malformed")
    return schema_text[start:end]


def coverage_public_types(block):
    match = re.search(r"public_types: &\[(.*?)\]", block)
    if not match:
        fail("ContractRowCoverage entry missing public_types")
    return re.findall(r'"([^"]+)"', match.group(1))


def module_declares_public_type(module_text, type_name):
    patterns = [
        rf"\bpub struct {re.escape(type_name)}\b",
        rf"\bpub enum {re.escape(type_name)}\b",
        rf"\bpub type {re.escape(type_name)}\b",
        rf"\bid_type!\({re.escape(type_name)}\)",
    ]
    return any(re.search(pattern, module_text) for pattern in patterns)


def main():
    contract_path = Path(sys.argv[1])
    source_paths = [Path(path) for path in sys.argv[2:]]
    contract = json.loads(contract_path.read_text(encoding="utf-8"))
    sources = load_sources(source_paths)
    source_text = "\n".join(sources.values())
    schema_text = sources.get("schema.rs", "")
    lib_text = sources.get("lib.rs", "")
    extensions_text = sources.get("extensions.rs", "")

    contract_rows = contract["rows"]
    if len(contract_rows) != 22:
        fail(f"expected 22 contract rows, got {len(contract_rows)}")

    for row in contract_rows:
        row_id = row["id"]
        surface = row["surface"]
        module = EXPECTED_MODULES.get(row_id)
        if module is None:
            fail(f"{row_id} has no expected Rust module")
        if f'pub mod {module};' not in lib_text:
            fail(f"{row_id} missing public module {module}")
        block = coverage_block(schema_text, row_id)
        if f'surface: "{surface}"' not in block:
            fail(f"{row_id} missing ContractRowCoverage surface")
        if f'module: "{module}"' not in block:
            fail(f"{row_id} missing ContractRowCoverage module {module}")
        public_types = coverage_public_types(block)
        if not public_types:
            fail(f"{row_id} has no public Rust schema types")
        module_text = sources.get(f"{module}.rs", "")
        for type_name in public_types:
            if not module_declares_public_type(module_text, type_name):
                fail(f"{row_id} public type {type_name} is not declared in {module}.rs")

    for term in OPAQUE_PLATFORM_BAG_TERMS:
        if term in source_text:
            fail(f"IR schema contains opaque platform bag term: {term}")

    if "pub enum PlatformExtensionPayload" not in extensions_text:
        fail("PlatformExtensionPayload must be a typed enum")
    for variant in ["Ios", "Android", "Web", "Png", "RustHost"]:
        if not re.search(rf"\b{variant}\([A-Za-z]+PlatformExtension\)", extensions_text):
            fail(f"PlatformExtensionPayload missing typed {variant} variant")
    for field in ["namespace: ExtensionNamespace", "version: ExtensionVersion", "capability_id: CapabilityId"]:
        if field not in extensions_text:
            fail(f"PlatformExtension missing typed field {field}")

    print(f"validated {len(contract_rows)} PR01 contract rows against Rust IR schema metadata")


if __name__ == "__main__":
    main()
