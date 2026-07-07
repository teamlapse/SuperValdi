#!/usr/bin/env python3

import json
import sys
from pathlib import Path


def fail(message):
    raise AssertionError(message)


def flatten_metadata(value):
    if isinstance(value, list):
        return " ".join(flatten_metadata(item) for item in value)
    if isinstance(value, dict):
        return " ".join(flatten_metadata(item) for item in value.values())
    return str(value)


def main():
    graph_path = Path(sys.argv[1])
    source_paths = [Path(path) for path in sys.argv[2:]]
    graph = json.loads(graph_path.read_text(encoding="utf-8"))

    for term in graph["public_api_lint"]["disallowed_public_metadata_terms"]:
        for crate in graph["crates"]:
            public_metadata = flatten_metadata(crate)
            if term in public_metadata:
                fail(f"{crate['name']} exposes forbidden public metadata term: {term}")

    for source_path in source_paths:
        source_text = source_path.read_text(encoding="utf-8")
        for term in graph["public_api_lint"]["disallowed_rust_source_terms"]:
            if term in source_text:
                fail(f"{source_path} exposes forbidden Rust public API term: {term}")


if __name__ == "__main__":
    main()
