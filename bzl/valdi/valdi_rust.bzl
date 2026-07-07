load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library")

def _crate_name(name):
    return name.replace("-", "_")

def valdi_rust_framework_crate(name, srcs, crate_name = None, deps = [], visibility = None):
    """Declares a Rust framework crate plus a source filegroup for lint tests."""
    rust_library(
        name = name,
        srcs = srcs,
        crate_name = crate_name or _crate_name(name),
        edition = "2024",
        deps = deps,
        visibility = visibility,
    )

    native.filegroup(
        name = name + "_sources",
        srcs = srcs,
        visibility = ["//valdi_rust/tests:__pkg__"],
    )

def valdi_rust_cli(name, srcs, deps = [], visibility = None):
    """Declares a Rust CLI binary plus a source filegroup for lint tests."""
    rust_binary(
        name = name,
        srcs = srcs,
        edition = "2024",
        deps = deps,
        visibility = visibility,
    )

    native.filegroup(
        name = name + "_sources",
        srcs = srcs,
        visibility = ["//valdi_rust/tests:__pkg__"],
    )

def valdi_rust_crate_graph(name, src, visibility = None):
    """Publishes the Rust foundation crate graph metadata."""
    native.filegroup(
        name = name,
        srcs = [src],
        visibility = visibility,
    )

def valdi_rust_platform_host_placeholder(name, platform, src = "empty_platform_labels_test.py"):
    """Creates an executable placeholder test for a platform host label."""
    native.py_test(
        name = name,
        srcs = [src],
        main = src,
        args = [
            "--kind=platform_host",
            "--target=%s" % platform,
        ],
    )

def valdi_rust_generated_glue_placeholder(name, target, src = "empty_platform_labels_test.py"):
    """Creates an executable placeholder test for generated glue labels."""
    native.py_test(
        name = name,
        srcs = [src],
        main = src,
        args = [
            "--kind=generated_glue",
            "--target=%s" % target,
        ],
    )

def valdi_rust_fixture_test_label(name, platform, src = "empty_platform_labels_test.py"):
    """Creates an executable empty fixture label for backend conformance wiring."""
    native.py_test(
        name = name,
        srcs = [src],
        main = src,
        args = [
            "--kind=fixture",
            "--target=%s" % platform,
        ],
    )
