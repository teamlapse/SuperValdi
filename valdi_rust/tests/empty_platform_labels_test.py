#!/usr/bin/env python3

import argparse


ALLOWED_TARGETS = {
    "fixture": {"android", "ios", "png", "web"},
    "generated_glue": {"js_dom", "kotlin", "rust_host", "swift"},
    "platform_host": {"android", "ios", "png", "web"},
}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--kind", required=True, choices=sorted(ALLOWED_TARGETS))
    parser.add_argument("--target", required=True)
    args = parser.parse_args()

    if args.target not in ALLOWED_TARGETS[args.kind]:
        raise AssertionError(f"unsupported {args.kind} target: {args.target}")

    print(f"empty {args.kind} label ready for {args.target}")


if __name__ == "__main__":
    main()
