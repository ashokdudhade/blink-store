#!/usr/bin/env bash
# Build release and copy binaries to dist/ for local distribution.
# Run from repo root: ./scripts/build-dist.sh
set -e
cd "$(dirname "$0")/.."

echo "Building release (binary + examples)..."
cargo build --release --examples

mkdir -p dist
if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
  cp -f target/release/blink-store.exe dist/
  cp -f target/release/examples/blink_client.exe dist/ 2>/dev/null || true
  cp -f target/release/examples/backend_http.exe dist/ 2>/dev/null || true
  echo "Created dist/blink-store.exe (+ blink_client.exe, backend_http.exe)"
else
  cp -f target/release/blink-store dist/
  cp -f target/release/examples/blink_client dist/ 2>/dev/null || true
  cp -f target/release/examples/backend_http dist/ 2>/dev/null || true
  echo "Created dist/blink-store (+ blink_client, backend_http)"
fi
