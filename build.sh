#!/usr/bin/env bash
set -euo pipefail

rm -rf build
cargo run --bin uniffi-bindgen generate src/lib.udl --language kotlin --out-dir build/kotlin
cargo run --bin uniffi-bindgen generate src/lib.udl --language swift --out-dir build/swift