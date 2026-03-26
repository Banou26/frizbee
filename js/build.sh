#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

echo "Building WASM..."
RUSTFLAGS="-C target-feature=+simd128" wasm-pack build "$ROOT_DIR" \
  --target web \
  --out-dir "$SCRIPT_DIR/pkg" \
  --out-name frizbee

echo "Building TypeScript..."
cd "$SCRIPT_DIR"
npx tsc

echo "Done. Output in js/dist/ and js/pkg/"
