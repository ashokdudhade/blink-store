# Example: Using a release from GitHub (no clone)

Run **Blink Store** from [GitHub Releases](https://github.com/ashokdudhade/blink-store/releases). No Git clone, no Rust.

- Use **latest** for the newest release.
- Use a version tag (e.g. `v0.1.0`) to pin.

## 1. Install the binary (curl, latest)

**One-liner:**

```bash
curl -sSLf https://raw.githubusercontent.com/ashokdudhade/blink-store/main/scripts/install-from-github.sh | bash -s -- latest ./bin
```

**Or download the script and run it (to pin a version):**

```bash
curl -sSLf https://raw.githubusercontent.com/ashokdudhade/blink-store/main/scripts/install-from-github.sh -o install-from-github.sh
chmod +x install-from-github.sh
./install-from-github.sh latest ./bin
# Pin: ./install-from-github.sh v0.1.0 ./bin
```

This installs `./bin/blink-store` (or `blink-store.exe` on Windows).

## 2. Start the server

```bash
./bin/blink-store serve --tcp 127.0.0.1 8765
```

## 3. Use a client

The release only includes the server. Use any client that speaks the [protocol](https://github.com/ashokdudhade/blink-store/blob/main/docs/PROTOCOL_SPEC.md). Example (no clone):

```bash
curl -sSLf https://raw.githubusercontent.com/ashokdudhade/blink-store/main/examples/clients/python/blink_client.py -o blink_client.py
python3 blink_client.py 127.0.0.1 8765
# Then: SET hello world, GET hello, QUIT
```

## Tags and versions

| Tag / usage | Meaning |
|-------------|---------|
| **latest** | Moving tag; always the newest release. |
| **v0.1.0**, **v0.2.0**, … | Permanent version tags for pinning. |

## Asset names

| Platform   | Asset name |
|-----------|------------|
| Linux x86_64 | `blink-store-x86_64-unknown-linux-gnu` |
| Linux aarch64 | `blink-store-aarch64-unknown-linux-gnu` |
| macOS x86_64 | `blink-store-x86_64-apple-darwin` |
| macOS arm64 | `blink-store-aarch64-apple-darwin` |
| Windows x86_64 | `blink-store-x86_64-pc-windows-msvc.exe` |

Direct download (replace `latest` with a version to pin):

```bash
curl -sSLf https://github.com/ashokdudhade/blink-store/releases/download/latest/blink-store-x86_64-unknown-linux-gnu -o blink-store && chmod +x blink-store
```
