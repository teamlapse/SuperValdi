#!/usr/bin/env python3

import json
import sys
from pathlib import Path


EXPECTED_DIAGNOSTICS = {
    "schema_path": ("invalid.schema_path.v1", "IR_SCHEMA_PATH_INVALID", "$.root.children[0].kind"),
    "source_span": ("invalid.source_span.v1", "IR_SOURCE_SPAN_MISSING", "source://fixtures/diagnostics.valdi.rs:1:1"),
    "backend_path": ("invalid.backend_path.v1", "IR_BACKEND_PATH_UNRESOLVED", "backend://rust_backend/root"),
    "capability_error": ("invalid.capability_error.v1", "IR_CAPABILITY_UNSUPPORTED", "$.capabilities[missing]"),
    "unsupported_surface": ("invalid.unsupported_surface.v1", "IR_SURFACE_UNSUPPORTED", "$.element.unsupported"),
    "fixture_id": ("invalid.fixture_id.v1", "IR_FIXTURE_ID_INVALID", "$.fixture_id"),
    "action_id": ("invalid.action_id.v1", "IR_ACTION_ID_UNRESOLVED", "$.actions[missing]"),
    "module_id": ("invalid.module_id.v1", "IR_MODULE_ID_UNRESOLVED", "$.modules[missing]"),
}


def fail(message):
    raise AssertionError(message)


def require_equal(actual, expected, context):
    if actual != expected:
        fail(f"{context} drift: {actual!r} != {expected!r}")


def require_unique(items, key, label):
    seen = {}
    for item in items:
        if key not in item:
            fail(f"{label} missing {key}: {item}")
        value = item.get(key)
        if value in seen:
            fail(f"duplicate {label} {value}")
        seen[value] = item
    return seen


def main():
    manifest_path = Path(sys.argv[1])
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))

    require_equal(manifest.get("schema_version"), 1, "invalid manifest schema_version")
    require_equal(manifest.get("owner_pr"), "PR04", "invalid manifest owner_pr")
    require_equal(manifest.get("source_ir_module"), "valdi_rust/ir/src/diagnostics.rs", "invalid manifest source_ir_module")
    require_equal(manifest.get("invalid_fixture_id_format"), "invalid.<diagnostic_family>.v1", "invalid fixture ID format")

    entries = manifest.get("invalid_fixtures", [])
    entries_by_id = require_unique(entries, "fixture_id", "invalid fixture_id")
    entries_by_family = require_unique(entries, "diagnostic_family", "invalid diagnostic family")
    expected_ids = {item[0] for item in EXPECTED_DIAGNOSTICS.values()}
    if set(entries_by_id) != expected_ids:
        missing = sorted(expected_ids - set(entries_by_id))
        extra = sorted(set(entries_by_id) - expected_ids)
        fail(f"invalid fixture ID set drift, missing={missing}, extra={extra}")

    if set(entries_by_family) != set(EXPECTED_DIAGNOSTICS):
        missing = sorted(set(EXPECTED_DIAGNOSTICS) - set(entries_by_family))
        extra = sorted(set(entries_by_family) - set(EXPECTED_DIAGNOSTICS))
        fail(f"invalid diagnostic family coverage drift, missing={missing}, extra={extra}")

    for family, (fixture_id, code, path) in EXPECTED_DIAGNOSTICS.items():
        entry = entries_by_family[family]
        require_equal(entry.get("fixture_id"), fixture_id, f"{family} fixture_id")
        diagnostic = entry.get("expected_diagnostic", {})
        require_equal(diagnostic.get("code"), code, f"{family} diagnostic code")
        require_equal(diagnostic.get("path"), path, f"{family} diagnostic path")
        require_equal(diagnostic.get("severity"), "error", f"{family} diagnostic severity")

    print(f"validated {len(entries)} invalid fixtures across {len(EXPECTED_DIAGNOSTICS)} diagnostic families")


if __name__ == "__main__":
    main()
