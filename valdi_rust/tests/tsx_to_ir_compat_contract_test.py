#!/usr/bin/env python3

import copy
import json
import sys
from pathlib import Path


REQUIRED_FIELDS = [
    "contract_row_id",
    "fixture_id",
    "root_node_id",
    "component_id",
    "surface_kind",
    "coverage_tokens",
    "serialized_artifact_path",
]


def fail(message):
    raise AssertionError(message)


def load_json(path):
    return json.loads(Path(path).read_text(encoding="utf-8"))


def require_equal(actual, expected, context):
    if actual != expected:
        fail(f"{context} drift: {actual!r} != {expected!r}")


def require_unique(items, key, label):
    seen = {}
    for item in items:
        value = item.get(key)
        if value in seen:
            fail(f"duplicate {label} {value}")
        seen[value] = item
    return seen


def expected_fixture_id(row_id):
    return f"contract.{row_id}.v1"


def expected_artifact_path(row_id):
    return f"valdi_rust/fixtures/serialized/{row_id}.ir.json"


def expected_snapshot_line(row):
    row_id = row["id"]
    return "|".join(
        [
            row_id,
            expected_fixture_id(row_id),
            f"node.{row_id}.root",
            f"component.{row_id}",
            row_id,
            ",".join(row["coverage"]),
        ]
    )


def validate(contract, fixture_manifest, tsx_manifest):
    require_equal(tsx_manifest.get("schema_version"), 1, "tsx manifest schema_version")
    require_equal(tsx_manifest.get("owner_pr"), "PR10", "tsx manifest owner_pr")
    require_equal(
        tsx_manifest.get("source_contract"),
        "docs/rust_migration/replacement_contract.yaml",
        "tsx manifest source_contract",
    )
    require_equal(
        tsx_manifest.get("source_fixture_manifest"),
        "valdi_rust/fixtures/contract_fixture_manifest.json",
        "tsx manifest source_fixture_manifest",
    )
    require_equal(
        tsx_manifest.get("normalized_ir_emitter"),
        "tsx_normalized_ir_sidecar_v1",
        "tsx manifest normalized_ir_emitter",
    )
    require_equal(
        tsx_manifest.get("direct_renderer_compatibility"),
        "retained_jsx_processor",
        "tsx manifest direct_renderer_compatibility",
    )
    require_equal(tsx_manifest.get("rust_path_dependency"), "none", "tsx manifest rust_path_dependency")

    rows = contract.get("rows", [])
    fixture_entries = fixture_manifest.get("fixtures", [])
    tsx_entries = tsx_manifest.get("fixtures", [])
    require_equal(len(tsx_entries), len(rows), "tsx manifest fixture count")

    rows_by_id = require_unique(rows, "id", "contract row")
    fixture_by_row = require_unique(fixture_entries, "contract_row_id", "fixture manifest row")
    tsx_by_row = require_unique(tsx_entries, "contract_row_id", "tsx manifest row")
    require_unique(tsx_entries, "tsx_fixture_id", "tsx fixture id")
    require_unique(tsx_entries, "serialized_artifact_path", "tsx serialized artifact path")

    require_equal(list(tsx_by_row), list(rows_by_id), "tsx manifest row order")

    snapshot_lines = ["dsl_canonical_ir_v1"]
    for row_id, row in rows_by_id.items():
        fixture = fixture_by_row[row_id]
        tsx_fixture = tsx_by_row[row_id]
        require_equal(tsx_fixture.get("fixture_id"), expected_fixture_id(row_id), f"{row_id} fixture_id")
        require_equal(tsx_fixture.get("fixture_id"), fixture["fixture_id"], f"{row_id} corpus fixture_id")
        require_equal(tsx_fixture.get("surface"), row["surface"], f"{row_id} surface")
        require_equal(tsx_fixture.get("fixture_tags"), row["fixture_tags"], f"{row_id} fixture_tags")
        require_equal(tsx_fixture.get("platform_targets"), row["platform_targets"], f"{row_id} platform_targets")
        require_equal(tsx_fixture.get("coverage"), row["coverage"], f"{row_id} coverage")
        require_equal(
            tsx_fixture.get("serialized_artifact_path"),
            expected_artifact_path(row_id),
            f"{row_id} serialized_artifact_path",
        )
        require_equal(
            tsx_fixture.get("serialized_artifact_path"),
            fixture["serialized_artifact_path"],
            f"{row_id} corpus serialized_artifact_path",
        )
        require_equal(
            tsx_fixture.get("tsx_feature_tags"),
            ["tsx_compat", "direct_renderer_compat", "normalized_ir"],
            f"{row_id} tsx_feature_tags",
        )
        require_equal(
            tsx_fixture.get("normalized_ir_required_fields"),
            REQUIRED_FIELDS,
            f"{row_id} normalized_ir_required_fields",
        )
        snapshot_lines.append(expected_snapshot_line(row))

    expected_snapshot = f"{chr(10).join(snapshot_lines)}\n"
    if not expected_snapshot.startswith("dsl_canonical_ir_v1\n"):
        fail("tsx normalized IR snapshot does not use the PR09 canonical DSL snapshot format")


def expect_failure(label, contract, fixture_manifest, tsx_manifest):
    try:
        validate(contract, fixture_manifest, tsx_manifest)
    except AssertionError:
        return
    fail(f"{label} mutation unexpectedly passed")


def main():
    contract = load_json(sys.argv[1])
    fixture_manifest = load_json(sys.argv[2])
    tsx_manifest = load_json(sys.argv[3])

    validate(contract, fixture_manifest, tsx_manifest)

    missing_row = copy.deepcopy(tsx_manifest)
    missing_row["fixtures"] = missing_row["fixtures"][:-1]
    expect_failure("missing row", contract, fixture_manifest, missing_row)

    duplicate_row = copy.deepcopy(tsx_manifest)
    duplicate_row["fixtures"][-1] = copy.deepcopy(duplicate_row["fixtures"][0])
    expect_failure("duplicate row", contract, fixture_manifest, duplicate_row)

    coverage_drift = copy.deepcopy(tsx_manifest)
    coverage_drift["fixtures"][20]["coverage"] = coverage_drift["fixtures"][20]["coverage"][:-1]
    expect_failure("coverage drift", contract, fixture_manifest, coverage_drift)

    path_drift = copy.deepcopy(tsx_manifest)
    path_drift["fixtures"][0]["serialized_artifact_path"] = "valdi_rust/fixtures/serialized/drift.ir.json"
    expect_failure("serialized path drift", contract, fixture_manifest, path_drift)

    print(
        "validated TSX normalized IR coverage for "
        f"{len(tsx_manifest['fixtures'])} contract rows against PR04 fixtures and PR09 DSL snapshot"
    )


if __name__ == "__main__":
    main()
