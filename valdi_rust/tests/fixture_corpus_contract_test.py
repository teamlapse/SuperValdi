#!/usr/bin/env python3

import json
import sys
from pathlib import Path


SERIALIZED_ROOT = "valdi_rust/fixtures/serialized/"


def fail(message):
    raise AssertionError(message)


def load_json(path):
    return json.loads(Path(path).read_text(encoding="utf-8"))


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


def serialized_repo_path(path):
    text = Path(path).as_posix()
    index = text.rfind(SERIALIZED_ROOT)
    if index == -1:
        fail(f"serialized fixture path is outside {SERIALIZED_ROOT}: {path}")
    return text[index:]


def expected_fixture_id(row_id):
    return f"contract.{row_id}.v1"


def expected_artifact_path(row_id):
    return f"{SERIALIZED_ROOT}{row_id}.ir.json"


def require_equal(actual, expected, context):
    if actual != expected:
        fail(f"{context} drift: {actual!r} != {expected!r}")


def main():
    contract_path = Path(sys.argv[1])
    manifest_path = Path(sys.argv[2])
    serialized_paths = [Path(path) for path in sys.argv[3:]]
    contract = load_json(contract_path)
    manifest = load_json(manifest_path)

    require_equal(manifest.get("schema_version"), 1, "manifest schema_version")
    require_equal(manifest.get("owner_pr"), "PR04", "manifest owner_pr")
    require_equal(manifest.get("source_contract"), "docs/rust_migration/replacement_contract.yaml", "manifest source_contract")
    require_equal(manifest.get("fixture_id_format"), "contract.<contract_row_id>.v1", "manifest fixture_id_format")
    require_equal(manifest.get("serialized_artifact_root"), "valdi_rust/fixtures/serialized", "manifest serialized_artifact_root")

    rows = contract.get("rows", [])
    rows_by_id = require_unique(rows, "id", "contract row id")
    fixtures = manifest.get("fixtures", [])
    fixtures_by_id = require_unique(fixtures, "fixture_id", "fixture_id")
    require_unique(fixtures, "contract_row_id", "contract fixture row")
    require_unique(fixtures, "serialized_artifact_path", "serialized artifact path")

    serialized_by_path = {}
    for path in serialized_paths:
        repo_path = serialized_repo_path(path)
        if repo_path in serialized_by_path:
            fail(f"duplicate serialized fixture path {repo_path}")
        serialized_by_path[repo_path] = load_json(path)

    expected_ids = {expected_fixture_id(row_id) for row_id in rows_by_id}
    if set(fixtures_by_id) != expected_ids:
        missing = sorted(expected_ids - set(fixtures_by_id))
        extra = sorted(set(fixtures_by_id) - expected_ids)
        fail(f"fixture ID set drift, missing={missing}, extra={extra}")

    expected_artifact_paths = {expected_artifact_path(row_id) for row_id in rows_by_id}
    if set(serialized_by_path) != expected_artifact_paths:
        missing = sorted(expected_artifact_paths - set(serialized_by_path))
        extra = sorted(set(serialized_by_path) - expected_artifact_paths)
        fail(f"serialized fixture path set drift, missing={missing}, extra={extra}")

    covered_tag_pairs = set()
    for row_id, row in rows_by_id.items():
        fixture = fixtures_by_id[expected_fixture_id(row_id)]
        artifact_path = expected_artifact_path(row_id)
        require_equal(fixture.get("contract_row_id"), row_id, f"{row_id} contract_row_id")
        require_equal(fixture.get("surface"), row["surface"], f"{row_id} surface")
        require_equal(fixture.get("fixture_tags"), row["fixture_tags"], f"{row_id} fixture tags")
        require_equal(fixture.get("platform_targets"), row["platform_targets"], f"{row_id} platform targets")
        require_equal(fixture.get("owner_prs"), row["owner_prs"], f"{row_id} owner PRs")
        require_equal(fixture.get("proof_prs"), row["proof_prs"], f"{row_id} proof PRs")
        require_equal(fixture.get("serialized_artifact_path"), artifact_path, f"{row_id} serialized artifact path")
        for tag in row["fixture_tags"]:
            covered_tag_pairs.add((row_id, tag))

        if artifact_path not in serialized_by_path:
            fail(f"{row_id} missing serialized fixture {artifact_path}")
        serialized = serialized_by_path[artifact_path]
        require_equal(serialized.get("fixture_schema_version"), 1, f"{row_id} serialized fixture_schema_version")
        require_equal(serialized.get("fixture_id"), expected_fixture_id(row_id), f"{row_id} serialized fixture_id")
        require_equal(serialized.get("contract_row_id"), row_id, f"{row_id} serialized contract_row_id")
        require_equal(serialized.get("surface"), row["surface"], f"{row_id} serialized surface")
        require_equal(serialized.get("fixture_tags"), row["fixture_tags"], f"{row_id} serialized fixture tags")
        require_equal(serialized.get("platform_targets"), row["platform_targets"], f"{row_id} serialized platform targets")
        metadata = serialized.get("metadata", {})
        require_equal(metadata.get("owner_prs"), row["owner_prs"], f"{row_id} serialized owner PRs")
        require_equal(metadata.get("proof_prs"), row["proof_prs"], f"{row_id} serialized proof PRs")
        require_equal(metadata.get("serialized_artifact_path"), artifact_path, f"{row_id} serialized artifact path")
        ir_debug = serialized.get("ir_debug", {})
        require_equal(ir_debug.get("contract_surface"), row_id, f"{row_id} serialized contract surface")
        require_equal(ir_debug.get("coverage_tokens"), row["coverage"], f"{row_id} serialized coverage tokens")

    required_tag_pairs = {(row["id"], tag) for row in rows for tag in row["fixture_tags"]}
    if covered_tag_pairs != required_tag_pairs:
        missing = sorted(required_tag_pairs - covered_tag_pairs)
        extra = sorted(covered_tag_pairs - required_tag_pairs)
        fail(f"fixture tag-pair coverage drift, missing={missing}, extra={extra}")

    print(
        "validated "
        f"{len(fixtures)} fixture IDs, {len(rows_by_id)} contract rows, "
        f"and {len(required_tag_pairs)} contract row/tag pairs"
    )


if __name__ == "__main__":
    main()
