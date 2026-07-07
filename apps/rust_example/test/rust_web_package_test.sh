#!/usr/bin/env bash
set -euo pipefail

package_dir="${TEST_SRCDIR}/${TEST_WORKSPACE}/apps/rust_example/rust_example_npm"

test -f "${package_dir}/native/rust_example/web/RustCounter.js"
test -f "${package_dir}/native/rust_example/web/rust_counter_web_wasm.wasm"
test -f "${package_dir}/src/RegisterNativeModules.js"

grep -q "rust_counter_web_wasm.wasm" "${package_dir}/native/rust_example/web/RustCounter.js"
grep -q "rust_example/src/RustCounter" "${package_dir}/src/RegisterNativeModules.js"
