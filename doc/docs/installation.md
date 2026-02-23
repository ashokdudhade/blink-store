---
sidebar_position: 2
title: Installation
---

# Installation

No Git clone required. Pick the method that suits your environment.

---

## Option 1 — Install script (recommended)

Downloads the correct binary for your OS and architecture automatically.

**One-liner (Linux / macOS):**

```bash
curl -sSLf https://raw.githubusercontent.com/ashokdudhade/blink-store/main/scripts/install-from-github.sh \
  | bash -s -- latest ./bin
```

**Pin to a specific version:**

```bash
curl -sSLf https://raw.githubusercontent.com/ashokdudhade/blink-store/main/scripts/install-from-github.sh \
  | bash -s -- v0.1.0 ./bin
```

The binary is saved to `./bin/blink-store`. Start the server:

```bash
./bin/blink-store serve --tcp 127.0.0.1:8765
```

| Platform | Architecture |
|----------|-------------|
| Linux    | x86_64, aarch64 |
| macOS    | x86_64, arm64 (Apple Silicon) |
| Windows  | x86_64 |

---

## Option 2 — Direct binary download

If you prefer a single `curl` command without the install script:

**Linux x86_64:**
```bash
curl -sSLf -o blink-store \
  https://github.com/ashokdudhade/blink-store/releases/download/latest/blink-store-x86_64-unknown-linux-gnu
chmod +x blink-store
```

**macOS arm64 (Apple Silicon):**
```bash
curl -sSLf -o blink-store \
  https://github.com/ashokdudhade/blink-store/releases/download/latest/blink-store-aarch64-apple-darwin
chmod +x blink-store
```

**Windows x86_64 (PowerShell):**
```powershell
curl -o blink-store.exe `
  https://github.com/ashokdudhade/blink-store/releases/download/latest/blink-store-x86_64-pc-windows-msvc.exe
```

Replace `latest` in the URL with a version tag (e.g. `v0.1.0`) to pin.

---

## Option 3 — Docker

```bash
docker run -p 8765:8765 ghcr.io/ashokdudhade/blink-store:latest
```

Custom port and memory limit:

```bash
docker run -p 9000:9000 \
  -e BLINK_PORT=9000 \
  -e BLINK_MEMORY_LIMIT=104857600 \
  ghcr.io/ashokdudhade/blink-store:latest
```

Or build from a Dockerfile (no clone):

```bash
curl -sSLf -o Dockerfile \
  https://raw.githubusercontent.com/ashokdudhade/blink-store/main/Dockerfile
docker build -t blink-store .
docker run -p 8765:8765 blink-store serve --tcp 0.0.0.0:8765
```

---

## Option 4 — Build from source

For contributors or custom builds. Requires [Rust](https://rustup.rs/).

```bash
git clone https://github.com/ashokdudhade/blink-store.git
cd blink-store
cargo build --release
./target/release/blink-store serve --tcp 127.0.0.1:8765
```

---

## Verify the installation

With the server running, open a second terminal:

```bash
echo "SET hello world" | nc 127.0.0.1 8765
# → OK

echo "GET hello" | nc 127.0.0.1 8765
# → VALUE d29ybGQ=

echo "d29ybGQ=" | base64 -d
# → world
```

---

## Version pinning

Every release is published with a permanent version tag (e.g. `v0.1.0`) **and** a moving `latest` tag that always points to the newest release.

| Tag | Behavior |
|-----|----------|
| `latest` | Updated on every release. Always the newest binary. |
| `v0.1.0`, `v0.2.0`, ... | Fixed. Never overwritten. Use these for reproducible deployments. |
