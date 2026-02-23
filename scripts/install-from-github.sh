#!/usr/bin/env bash
# Download blink-store binary from GitHub Releases.
# Usage: ./scripts/install-from-github.sh [VERSION] [DEST_DIR]
#   VERSION  default: latest (use e.g. v0.1.0 to pin)
#   DEST_DIR default: ./bin (created if missing)
# Example: ./scripts/install-from-github.sh ./bin
# Example (pin version): ./scripts/install-from-github.sh v0.1.0 ./bin
set -e

# Args: [VERSION] [DEST_DIR]. One arg: if v* then VERSION else DEST_DIR (latest).
if [[ -n "$2" ]]; then
  VERSION="$1"
  DEST_DIR="$2"
elif [[ -n "$1" && "$1" == v* ]]; then
  VERSION="$1"
  DEST_DIR="./bin"
elif [[ -n "$1" ]]; then
  VERSION="latest"
  DEST_DIR="$1"
else
  VERSION="latest"
  DEST_DIR="./bin"
fi
REPO="https://github.com/ashokdudhade/blink-store"

case "$(uname -s)" in
  Linux)
    case "$(uname -m)" in
      x86_64|amd64)  TARGET="x86_64-unknown-linux-gnu" ;;
      aarch64|arm64) TARGET="aarch64-unknown-linux-gnu" ;;
      *) echo "Unsupported arch: $(uname -m)" >&2; exit 1 ;;
    esac
    BINARY_NAME="blink-store"
    ;;
  Darwin)
    case "$(uname -m)" in
      x86_64)  TARGET="x86_64-apple-darwin" ;;
      arm64)   TARGET="aarch64-apple-darwin" ;;
      *) echo "Unsupported arch: $(uname -m)" >&2; exit 1 ;;
    esac
    BINARY_NAME="blink-store"
    ;;
  MINGW*|MSYS*|CYGWIN*)
    TARGET="x86_64-pc-windows-msvc.exe"
    BINARY_NAME="blink-store.exe"
    ;;
  *)
    echo "Unsupported OS: $(uname -s)" >&2
    exit 1
    ;;
esac

# Asset name on GitHub (Windows artifact already has .exe in the name)
if [[ "$TARGET" == *.exe ]]; then
  ASSET="blink-store-$TARGET"
else
  ASSET="blink-store-$TARGET"
fi

URL="${REPO}/releases/download/${VERSION}/${ASSET}"
mkdir -p "$DEST_DIR"
OUT="$DEST_DIR/$BINARY_NAME"

echo "Downloading $VERSION for $TARGET..."
if command -v curl >/dev/null 2>&1; then
  curl -sSLf -o "$OUT" "$URL"
elif command -v wget >/dev/null 2>&1; then
  wget -q -O "$OUT" "$URL"
else
  echo "Need curl or wget" >&2
  exit 1
fi

chmod +x "$OUT"
echo "Installed: $OUT"
echo "Run server: $OUT serve --tcp 127.0.0.1 8765"
