#!/usr/bin/env python3
"""Validate the Rust migration replacement contract and generated Markdown.

The repository has YAML parser dependencies in package-specific TypeScript
workspaces, but no repo-wide Python YAML dependency. To keep this check
self-contained, replacement_contract.yaml is constrained to JSON-form YAML.
This script validates that exact subset through Python's JSON parser and then
performs contract-specific checks.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from collections import Counter
from pathlib import Path
from typing import Any, Iterable


REPO_ROOT = Path(__file__).resolve().parent.parent
DEFAULT_CONTRACT = REPO_ROOT / "docs/rust_migration/replacement_contract.yaml"
DEFAULT_MARKDOWN = REPO_ROOT / "docs/rust_migration/replacement_contract.md"

REQUIRED_FIXTURE_TAGS = ["ios", "android", "web", "dom_snapshot", "screenshot", "static_png", "retained_backend", "rust_backend", "module", "native_view", "hot_reload", "dynamic_ui", "tsx_compat"]
REQUIRED_SURFACES = ["Schema and versioning", "Component identity", "Tree structure", "Element taxonomy", "Layout", "Styling", "Text", "Assets", "Events and gestures", "Actions and state", "Bindings and expressions", "Animations", "Native modules", "Native views", "Accessibility", "Hot reload", "Diagnostics", "Web DOM", "PNG backend", "Dynamic UI", "TS compatibility", "Build graph"]
REQUIRED_FORBIDDEN_DEPENDENCIES = ["TypeScript compiler/runtime", "JS direct renderer", "TSN C emitter/runtime", "RenderRequest", "ViewNodeRenderer", "ViewNodeTree", "C++ renderer adapter targets", "transition adapter targets"]

REQUIRED_TOP_LEVEL_KEYS = {"schema_version", "document", "fixture_tags", "platform_targets", "required_surfaces", "forbidden_dependencies", "rows"}
REQUIRED_DOCUMENT_KEYS = {"title", "source_yaml", "source_plan", "source_step", "generated_markdown", "contract_check"}
REQUIRED_ROW_KEYS = {"id", "surface", "coverage", "owner_prs", "implementation_prs", "schema_prs", "fixture_prs", "validator_prs", "backend_operation_prs", "proof_prs", "proof_gates", "fixture_tags", "platform_targets"}

STAND_IN_PATTERNS = [
    re.compile(pattern, re.IGNORECASE)
    for pattern in [
        r"\bTBD\b",
        r"\bTODO\b",
        r"\bplaceholder\b",
        r"\bstand[- ]in\b",
        r"\bopaque\b",
        r"\bunspecified\b",
        r"\bdefer(?:red|ring)?\b",
        r"\blater\b",
        r"\bfuture work\b",
        r"\betc\.\b",
        r"\bto be (?:defined|determined)\b",
        r"\bto-do\b",
    ]
]

PR_RE = re.compile(r"^PR\d{2}$")
ROW_ID_RE = re.compile(r"^[a-z][a-z0-9_]*$")


class ContractError(Exception):
    pass


def fail(message: str) -> None:
    raise ContractError(message)


def reject_duplicate_keys(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            fail(f"duplicate mapping key '{key}' in replacement_contract.yaml")
        result[key] = value
    return result


def load_contract(path: Path) -> dict[str, Any]:
    try:
        with path.open("r", encoding="utf-8") as handle:
            data = json.load(handle, object_pairs_hook=reject_duplicate_keys)
    except json.JSONDecodeError as exc:
        fail(
            "replacement_contract.yaml must use the supported JSON-form YAML "
            f"subset: {exc.msg} at line {exc.lineno}, column {exc.colno}"
        )
    except OSError as exc:
        fail(f"cannot read {path}: {exc}")

    if not isinstance(data, dict):
        fail("contract root must be an object")
    return data


def require_exact_keys(value: dict[str, Any], expected: set[str], path: str) -> None:
    actual = set(value)
    missing = expected - actual
    extra = actual - expected
    if missing:
        fail(f"{path} missing required keys: {', '.join(sorted(missing))}")
    if extra:
        fail(f"{path} contains unsupported keys: {', '.join(sorted(extra))}")


def require_string(value: Any, path: str) -> str:
    if not isinstance(value, str) or not value:
        fail(f"{path} must be a non-empty string")
    return value


def require_string_list(value: Any, path: str) -> list[str]:
    if not isinstance(value, list) or not value:
        fail(f"{path} must be a non-empty list")
    for index, item in enumerate(value):
        if not isinstance(item, str) or not item:
            fail(f"{path}[{index}] must be a non-empty string")
    if len(value) != len(set(value)):
        fail(f"{path} contains duplicate values")
    return value


def require_pr_list(value: Any, path: str) -> list[str]:
    prs = require_string_list(value, path)
    invalid = [pr for pr in prs if not PR_RE.fullmatch(pr)]
    if invalid:
        fail(f"{path} contains invalid PR references: {', '.join(invalid)}")
    return prs


def validate_no_stand_in_words(value: Any, path: str) -> None:
    if isinstance(value, str):
        for pattern in STAND_IN_PATTERNS:
            if pattern.search(value):
                fail(f"{path} contains stand-in wording: {value!r}")
    elif isinstance(value, list):
        for index, item in enumerate(value):
            validate_no_stand_in_words(item, f"{path}[{index}]")
    elif isinstance(value, dict):
        for key, item in value.items():
            validate_no_stand_in_words(item, f"{path}.{key}")


def validate_forbidden_dependencies(contract: dict[str, Any]) -> None:
    dependencies = contract["forbidden_dependencies"]
    if not isinstance(dependencies, list) or not dependencies:
        fail("forbidden_dependencies must be a non-empty list")

    names: list[str] = []
    ids: list[str] = []
    for index, dependency in enumerate(dependencies):
        path = f"forbidden_dependencies[{index}]"
        if not isinstance(dependency, dict):
            fail(f"{path} must be an object")
        require_exact_keys(
            dependency,
            {"id", "name", "forbidden_in", "proof_prs", "references"},
            path,
        )
        dep_id = require_string(dependency["id"], f"{path}.id")
        if not ROW_ID_RE.fullmatch(dep_id):
            fail(f"{path}.id must be snake_case")
        ids.append(dep_id)
        names.append(require_string(dependency["name"], f"{path}.name"))
        require_string_list(dependency["forbidden_in"], f"{path}.forbidden_in")
        require_pr_list(dependency["proof_prs"], f"{path}.proof_prs")
        require_string_list(dependency["references"], f"{path}.references")

    if len(ids) != len(set(ids)):
        fail("forbidden_dependencies contains duplicate ids")

    missing = sorted(set(REQUIRED_FORBIDDEN_DEPENDENCIES) - set(names))
    if missing:
        fail("forbidden dependency list missing required entries: " + ", ".join(missing))


def validate_rows(contract: dict[str, Any]) -> None:
    fixture_tags = set(contract["fixture_tags"])
    platform_targets = set(contract["platform_targets"])
    rows = contract["rows"]
    if not isinstance(rows, list) or not rows:
        fail("rows must be a non-empty list")
    if len(rows) != len(REQUIRED_SURFACES):
        fail(
            "rows count must match required surfaces: "
            f"{len(rows)} rows for {len(REQUIRED_SURFACES)} required surfaces"
        )

    row_ids: list[str] = []
    surfaces: list[str] = []
    for index, row in enumerate(rows):
        path = f"rows[{index}]"
        if not isinstance(row, dict):
            fail(f"{path} must be an object")
        require_exact_keys(row, REQUIRED_ROW_KEYS, path)

        row_id = require_string(row["id"], f"{path}.id")
        if not ROW_ID_RE.fullmatch(row_id):
            fail(f"{path}.id must be snake_case")
        row_ids.append(row_id)

        surface = require_string(row["surface"], f"{path}.surface")
        surfaces.append(surface)

        require_string_list(row["coverage"], f"{path}.coverage")
        require_pr_list(row["owner_prs"], f"{path}.owner_prs")
        require_pr_list(row["implementation_prs"], f"{path}.implementation_prs")
        require_pr_list(row["schema_prs"], f"{path}.schema_prs")
        require_pr_list(row["fixture_prs"], f"{path}.fixture_prs")
        require_pr_list(row["validator_prs"], f"{path}.validator_prs")
        require_pr_list(row["backend_operation_prs"], f"{path}.backend_operation_prs")
        require_pr_list(row["proof_prs"], f"{path}.proof_prs")
        require_string_list(row["proof_gates"], f"{path}.proof_gates")

        row_fixture_tags = require_string_list(row["fixture_tags"], f"{path}.fixture_tags")
        unknown_tags = sorted(set(row_fixture_tags) - fixture_tags)
        if unknown_tags:
            fail(f"{path}.fixture_tags contains unknown tags: {', '.join(unknown_tags)}")

        row_platform_targets = require_string_list(
            row["platform_targets"], f"{path}.platform_targets"
        )
        unknown_targets = sorted(set(row_platform_targets) - platform_targets)
        if unknown_targets:
            fail(
                f"{path}.platform_targets contains unknown targets: "
                + ", ".join(unknown_targets)
            )

    duplicates = sorted(row_id for row_id, count in Counter(row_ids).items() if count > 1)
    if duplicates:
        fail("duplicate row IDs: " + ", ".join(duplicates))

    duplicate_surfaces = sorted(
        surface for surface, count in Counter(surfaces).items() if count > 1
    )
    if duplicate_surfaces:
        fail("duplicate surfaces: " + ", ".join(duplicate_surfaces))

    missing_surfaces = sorted(set(REQUIRED_SURFACES) - set(surfaces))
    extra_surfaces = sorted(set(surfaces) - set(REQUIRED_SURFACES))
    if missing_surfaces:
        fail("contract missing required surfaces: " + ", ".join(missing_surfaces))
    if extra_surfaces:
        fail("contract contains unknown surfaces: " + ", ".join(extra_surfaces))


def validate_contract(contract: dict[str, Any]) -> None:
    require_exact_keys(contract, REQUIRED_TOP_LEVEL_KEYS, "contract")
    if contract["schema_version"] != 1:
        fail("schema_version must be 1")

    document = contract["document"]
    if not isinstance(document, dict):
        fail("document must be an object")
    require_exact_keys(document, REQUIRED_DOCUMENT_KEYS, "document")
    for key in REQUIRED_DOCUMENT_KEYS:
        require_string(document[key], f"document.{key}")

    fixture_tags = require_string_list(contract["fixture_tags"], "fixture_tags")
    if fixture_tags != REQUIRED_FIXTURE_TAGS:
        fail(
            "fixture_tags must exactly match the PR01 vocabulary: "
            + ", ".join(REQUIRED_FIXTURE_TAGS)
        )

    require_string_list(contract["platform_targets"], "platform_targets")
    required_surfaces = require_string_list(contract["required_surfaces"], "required_surfaces")
    if required_surfaces != REQUIRED_SURFACES:
        fail("required_surfaces must exactly match the plan coverage summary")

    validate_forbidden_dependencies(contract)
    validate_rows(contract)
    validate_no_stand_in_words(contract, "contract")


def join_list(values: Iterable[str]) -> str:
    return ", ".join(values)


def render_markdown(contract: dict[str, Any]) -> str:
    lines: list[str] = [
        "# Rust Migration Replacement Contract",
        "",
        "<!-- Generated by scripts/check_rust_migration_contract.py. Do not edit by hand. -->",
        "",
        f"Source YAML: `{contract['document']['source_yaml']}`",
        f"Source plan: `{contract['document']['source_plan']}`",
        f"Source step: `{contract['document']['source_step']}`",
        f"Schema version: `{contract['schema_version']}`",
        "",
        "## Fixture Tag Vocabulary",
        "",
        "| Tag |",
        "| --- |",
    ]

    for tag in contract["fixture_tags"]:
        lines.append(f"| `{tag}` |")

    lines.extend(
        [
            "",
            "## Platform Targets",
            "",
            "| Target |",
            "| --- |",
        ]
    )
    for target in contract["platform_targets"]:
        lines.append(f"| `{target}` |")

    lines.extend(
        [
            "",
            "## Forbidden Dependencies For Production Rust Apps",
            "",
            "| ID | Name | Forbidden In | Proof PRs | References |",
            "| --- | --- | --- | --- | --- |",
        ]
    )
    for dependency in contract["forbidden_dependencies"]:
        lines.append(
            "| `{id}` | {name} | {forbidden_in} | {proof_prs} | {references} |".format(
                id=dependency["id"],
                name=dependency["name"],
                forbidden_in=join_list(dependency["forbidden_in"]),
                proof_prs=join_list(dependency["proof_prs"]),
                references="<br>".join(f"`{item}`" for item in dependency["references"]),
            )
        )

    lines.extend(
        [
            "",
            "## Replacement Rows",
            "",
            "| Row ID | Surface | Coverage | Owners | Proof PRs | Proof Gates | Fixture Tags | Platform Targets |",
            "| --- | --- | --- | --- | --- | --- | --- | --- |",
        ]
    )
    for row in contract["rows"]:
        lines.append(
            "| `{id}` | {surface} | {coverage} | {owners} | {proof_prs} | {proof_gates} | {fixture_tags} | {platform_targets} |".format(
                id=row["id"],
                surface=row["surface"],
                coverage="<br>".join(row["coverage"]),
                owners=join_list(row["owner_prs"]),
                proof_prs=join_list(row["proof_prs"]),
                proof_gates="<br>".join(row["proof_gates"]),
                fixture_tags=join_list(f"`{tag}`" for tag in row["fixture_tags"]),
                platform_targets=join_list(f"`{target}`" for target in row["platform_targets"]),
            )
        )

    return "\n".join(lines).rstrip() + "\n"


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Validate replacement_contract.yaml and its generated Markdown."
    )
    parser.add_argument(
        "--contract",
        type=Path,
        default=DEFAULT_CONTRACT,
        help="Path to replacement_contract.yaml.",
    )
    parser.add_argument(
        "--markdown",
        type=Path,
        default=DEFAULT_MARKDOWN,
        help="Path to replacement_contract.md.",
    )
    parser.add_argument(
        "--write",
        action="store_true",
        help="Rewrite the generated Markdown from the YAML contract.",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    try:
        contract = load_contract(args.contract)
        validate_contract(contract)
        rendered_markdown = render_markdown(contract)

        if args.write:
            args.markdown.parent.mkdir(parents=True, exist_ok=True)
            args.markdown.write_text(rendered_markdown, encoding="utf-8")
        else:
            try:
                existing_markdown = args.markdown.read_text(encoding="utf-8")
            except OSError as exc:
                fail(f"cannot read {args.markdown}: {exc}")
            if existing_markdown != rendered_markdown:
                fail(
                    f"{args.markdown} is not generated from {args.contract}; "
                    "run scripts/check_rust_migration_contract.py --write"
                )

        print(
            "Contract check passed: "
            f"{len(contract['rows'])} rows, "
            f"{len(contract['fixture_tags'])} fixture tags, "
            f"{len(contract['forbidden_dependencies'])} forbidden dependencies."
        )
        return 0
    except ContractError as exc:
        print(f"Contract check failed: {exc}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
